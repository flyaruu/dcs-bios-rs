//! Generated from model-definitions/AJS37.json.
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

/// ADI AoA Indicator
/// Output: gauge position
pub struct AdiAoaIndicator(AnalogGauge);
impl AdiAoaIndicator {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(18004, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AdiAoaIndicator {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AdiAoaIndicator {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ADI Heading
/// Output: gauge position
pub struct AdiHeading(AnalogGauge);
impl AdiHeading {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17992, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AdiHeading {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AdiHeading {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ADI Horizontal ILS
/// Output: gauge position
pub struct AdiHorizontalIls(AnalogGauge);
impl AdiHorizontalIls {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(18000, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AdiHorizontalIls {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AdiHorizontalIls {
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
        Self(AnalogGauge::new(17990, 65535, 0))
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

/// ADI Roll
/// Output: gauge position
pub struct AdiRoll(AnalogGauge);
impl AdiRoll {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17994, 65535, 0))
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

/// ADI Slipball
/// Output: gauge position
pub struct AdiSlipball(AnalogGauge);
impl AdiSlipball {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(18002, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AdiSlipball {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AdiSlipball {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ADI Vertical ILS
/// Output: gauge position
pub struct AdiVerticalIls(AnalogGauge);
impl AdiVerticalIls {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17998, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AdiVerticalIls {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AdiVerticalIls {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ADI Vertical Velocity
/// Output: gauge position
pub struct AdiVerticalVelocity(AnalogGauge);
impl AdiVerticalVelocity {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17996, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AdiVerticalVelocity {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AdiVerticalVelocity {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// AFK 15 Deg Mode
/// Input: toggle switch state
/// Output: selector position
pub struct Afk15DegMode(Selector);
impl Afk15DegMode {
    pub const fn new() -> Self {
        Self(Selector::new(17940, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Afk15DegMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Afk15DegMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Autothrottle
/// Output: 0 if light is off, 1 if light is on
pub struct AfkFel(Led);
impl AfkFel {
    pub const fn new() -> Self {
        Self(Led::new(18028, 4096, 12))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for AfkFel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AfkFel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// AFK Lever
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AfkLever(Selector);
impl AfkLever {
    pub const fn new() -> Self {
        Self(Selector::new(17934, 1024, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AfkLever {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AfkLever {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// AFK Mode 3
/// Input: toggle switch state
/// Output: selector position
pub struct AfkMode3(Selector);
impl AfkMode3 {
    pub const fn new() -> Self {
        Self(Selector::new(17940, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AfkMode3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AfkMode3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Airspeed m/s
/// Output: gauge position
pub struct AirspeedMS(AnalogGauge);
impl AirspeedMS {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17968, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AirspeedMS {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AirspeedMS {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Airspeed Value
/// Output: Airspeed Value
pub struct AirspeedValue(Metadata);
impl AirspeedValue {
    pub const fn new() -> Self {
        Self(Metadata::new(18038, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AirspeedValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AirspeedValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Destination Panel Data Digit 1
/// Output: Destination Panel Data Digit 1
pub struct Ajs37DestIndicatorData1(Display<1>);
impl Ajs37DestIndicatorData1 {
    pub const fn new() -> Self {
        Self(Display::<1>::new(18094, 1))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for Ajs37DestIndicatorData1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Ajs37DestIndicatorData1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Destination Panel Data Digit 2
/// Output: Destination Panel Data Digit 2
pub struct Ajs37DestIndicatorData2(Display<1>);
impl Ajs37DestIndicatorData2 {
    pub const fn new() -> Self {
        Self(Display::<1>::new(18096, 1))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for Ajs37DestIndicatorData2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Ajs37DestIndicatorData2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Navigataion Panel Data Digit 1
/// Output: Navigataion Panel Data Digit 1
pub struct Ajs37NavIndicatorData1(Display<1>);
impl Ajs37NavIndicatorData1 {
    pub const fn new() -> Self {
        Self(Display::<1>::new(18082, 1))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for Ajs37NavIndicatorData1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Ajs37NavIndicatorData1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Navigataion Panel Data Digit 2
/// Output: Navigataion Panel Data Digit 2
pub struct Ajs37NavIndicatorData2(Display<1>);
impl Ajs37NavIndicatorData2 {
    pub const fn new() -> Self {
        Self(Display::<1>::new(18084, 1))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for Ajs37NavIndicatorData2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Ajs37NavIndicatorData2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Navigataion Panel Data Digit 3
/// Output: Navigataion Panel Data Digit 3
pub struct Ajs37NavIndicatorData3(Display<1>);
impl Ajs37NavIndicatorData3 {
    pub const fn new() -> Self {
        Self(Display::<1>::new(18086, 1))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for Ajs37NavIndicatorData3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Ajs37NavIndicatorData3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Navigataion Panel Data Digit 4
/// Output: Navigataion Panel Data Digit 4
pub struct Ajs37NavIndicatorData4(Display<1>);
impl Ajs37NavIndicatorData4 {
    pub const fn new() -> Self {
        Self(Display::<1>::new(18088, 1))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for Ajs37NavIndicatorData4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Ajs37NavIndicatorData4 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Navigataion Panel Data Digit 5
/// Output: Navigataion Panel Data Digit 5
pub struct Ajs37NavIndicatorData5(Display<1>);
impl Ajs37NavIndicatorData5 {
    pub const fn new() -> Self {
        Self(Display::<1>::new(18090, 1))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for Ajs37NavIndicatorData5 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Ajs37NavIndicatorData5 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Navigataion Panel Data Digit 6
/// Output: Navigataion Panel Data Digit 6
pub struct Ajs37NavIndicatorData6(Display<1>);
impl Ajs37NavIndicatorData6 {
    pub const fn new() -> Self {
        Self(Display::<1>::new(18092, 1))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for Ajs37NavIndicatorData6 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Ajs37NavIndicatorData6 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altimeter Setting
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct AltSet(LimitedDial);
impl AltSet {
    pub const fn new() -> Self {
        Self(LimitedDial::new(18054, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltSet {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltSet {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altimeter 10000 Meter
/// Output: gauge position
pub struct Altimeter10000m(AnalogGauge);
impl Altimeter10000m {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17978, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Altimeter10000m {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Altimeter10000m {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altimeter 1000 Meter
/// Output: gauge position
pub struct Altimeter1000m(AnalogGauge);
impl Altimeter1000m {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17980, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Altimeter1000m {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Altimeter1000m {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altimeter Baro 1000 hPa X000
/// Output: gauge position
pub struct AltimeterBaro1000Hpa(AnalogGauge);
impl AltimeterBaro1000Hpa {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17988, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltimeterBaro1000Hpa {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltimeterBaro1000Hpa {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altimeter Baro 100 hPa 0X00
/// Output: gauge position
pub struct AltimeterBaro100Hpa(AnalogGauge);
impl AltimeterBaro100Hpa {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17986, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltimeterBaro100Hpa {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltimeterBaro100Hpa {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altimeter Baro 10 hPa 00X0
/// Output: gauge position
pub struct AltimeterBaro10Hpa(AnalogGauge);
impl AltimeterBaro10Hpa {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17984, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltimeterBaro10Hpa {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltimeterBaro10Hpa {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altimeter Baro 1 hPa 000X
/// Output: gauge position
pub struct AltimeterBaro1Hpa(AnalogGauge);
impl AltimeterBaro1Hpa {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17982, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltimeterBaro1Hpa {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltimeterBaro1Hpa {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altitude Hold
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AltitudeHold(Selector);
impl AltitudeHold {
    pub const fn new() -> Self {
        Self(Selector::new(17936, 32, 5))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltitudeHold {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltitudeHold {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altitude Value
/// Output: Altitude Value
pub struct AltitudeValue(Metadata);
impl AltitudeValue {
    pub const fn new() -> Self {
        Self(Metadata::new(18040, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltitudeValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltitudeValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Anti Collision Lights
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AntiCollisionLights(Selector);
impl AntiCollisionLights {
    pub const fn new() -> Self {
        Self(Selector::new(17924, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AntiCollisionLights {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AntiCollisionLights {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Anti Jamming Mode (AS) Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct AntiJamMode(Selector);
impl AntiJamMode {
    pub const fn new() -> Self {
        Self(Selector::new(17922, 7, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AntiJamMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AntiJamMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ATT Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct AttLamp(Led);
impl AttLamp {
    pub const fn new() -> Self {
        Self(Led::new(17944, 2048, 11))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for AttLamp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AttLamp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Attitude Hold
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AttitudeHold(Selector);
impl AttitudeHold {
    pub const fn new() -> Self {
        Self(Selector::new(17936, 16, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AttitudeHold {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AttitudeHold {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Autopilot Yaw Trim
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct AutoYawTrim(LimitedDial);
impl AutoYawTrim {
    pub const fn new() -> Self {
        Self(LimitedDial::new(18060, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AutoYawTrim {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AutoYawTrim {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Backup ADI Cage
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct BackAdiCage(Selector);
impl BackAdiCage {
    pub const fn new() -> Self {
        Self(Selector::new(18032, 512, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BackAdiCage {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BackAdiCage {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Backup Altimeter Setting
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct BackupAltSetting(LimitedDial);
impl BackupAltSetting {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17938, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BackupAltSetting {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BackupAltSetting {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Backup Altimeter 10000 Meter
/// Output: gauge position
pub struct BackupAltimeter10000m(AnalogGauge);
impl BackupAltimeter10000m {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(18014, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BackupAltimeter10000m {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BackupAltimeter10000m {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Backup Altimeter 1000 Meter
/// Output: gauge position
pub struct BackupAltimeter1000m(AnalogGauge);
impl BackupAltimeter1000m {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(18016, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BackupAltimeter1000m {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BackupAltimeter1000m {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Backup Generator
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct BackupGenerator(Selector);
impl BackupGenerator {
    pub const fn new() -> Self {
        Self(Selector::new(17934, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BackupGenerator {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BackupGenerator {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Backup Indicated Airspeed
/// Output: gauge position
pub struct BackupIndicatedAirspeed(AnalogGauge);
impl BackupIndicatedAirspeed {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(18012, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BackupIndicatedAirspeed {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BackupIndicatedAirspeed {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Backup Pitch
/// Output: gauge position
pub struct BackupPitch(AnalogGauge);
impl BackupPitch {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(18018, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BackupPitch {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BackupPitch {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Backup Roll
/// Output: gauge position
pub struct BackupRoll(AnalogGauge);
impl BackupRoll {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(18020, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BackupRoll {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BackupRoll {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Low Fuel
/// Output: 0 if light is off, 1 if light is on
pub struct Bra24(Led);
impl Bra24 {
    pub const fn new() -> Self {
        Self(Led::new(18030, 1024, 10))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Bra24 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Bra24 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuel Distribution Low Pressure
/// Output: 0 if light is off, 1 if light is on
pub struct BraUppf(Led);
impl BraUppf {
    pub const fn new() -> Self {
        Self(Led::new(17944, 32768, 15))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for BraUppf {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BraUppf {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Engine Fire 1
/// Output: 0 if light is off, 1 if light is on
pub struct Brand1(Led);
impl Brand1 {
    pub const fn new() -> Self {
        Self(Led::new(17944, 8192, 13))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Brand1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Brand1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Engine Fire 2
/// Output: 0 if light is off, 1 if light is on
pub struct Brand2(Led);
impl Brand2 {
    pub const fn new() -> Self {
        Self(Led::new(17944, 16384, 14))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Brand2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Brand2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Turbine Starter Fire
/// Output: 0 if light is off, 1 if light is on
pub struct BrandGts(Led);
impl BrandGts {
    pub const fn new() -> Self {
        Self(Led::new(18030, 2048, 11))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for BrandGts {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BrandGts {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Cabin Air Valve
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct CabinAirValve(Selector);
impl CabinAirValve {
    pub const fn new() -> Self {
        Self(Selector::new(18076, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CabinAirValve {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CabinAirValve {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Canopy Jettison
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct CanopyJettison(Selector);
impl CanopyJettison {
    pub const fn new() -> Self {
        Self(Selector::new(17940, 4, 2))
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

/// Canopy Open/Close
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct CanopyOpenClose(Selector);
impl CanopyOpenClose {
    pub const fn new() -> Self {
        Self(Selector::new(17940, 3, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CanopyOpenClose {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CanopyOpenClose {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// CB Autopilot SA
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct CbAutopilot(Selector);
impl CbAutopilot {
    pub const fn new() -> Self {
        Self(Selector::new(17926, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CbAutopilot {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CbAutopilot {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// CB CI/SI
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct CbCiSi(Selector);
impl CbCiSi {
    pub const fn new() -> Self {
        Self(Selector::new(17926, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CbCiSi {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CbCiSi {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// CB Ejection System
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct CbEjectionSystem(Selector);
impl CbEjectionSystem {
    pub const fn new() -> Self {
        Self(Selector::new(17934, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CbEjectionSystem {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CbEjectionSystem {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// CB Engine
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct CbEngine(Selector);
impl CbEngine {
    pub const fn new() -> Self {
        Self(Selector::new(17934, 2, 1))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CbEngine {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CbEngine {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// CB High Alpha Warning
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct CbHighAlphaWarn(Selector);
impl CbHighAlphaWarn {
    pub const fn new() -> Self {
        Self(Selector::new(17926, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CbHighAlphaWarn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CbHighAlphaWarn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// CB Trim System
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct CbTrimSystem(Selector);
impl CbTrimSystem {
    pub const fn new() -> Self {
        Self(Selector::new(17926, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CbTrimSystem {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CbTrimSystem {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// CI Commanded Heading Value
/// Output: CI Commanded Heading Value
pub struct CiCommandHeadingValue(Metadata);
impl CiCommandHeadingValue {
    pub const fn new() -> Self {
        Self(Metadata::new(18048, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CiCommandHeadingValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CiCommandHeadingValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// CI Commanded Heading
/// Output: gauge position
pub struct CiCommandedHeading(AnalogGauge);
impl CiCommandedHeading {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(18008, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CiCommandedHeading {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CiCommandedHeading {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// CI Heading
/// Output: gauge position
pub struct CiHeading(AnalogGauge);
impl CiHeading {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(18006, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CiHeading {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CiHeading {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// CI Heading Value
/// Output: CI Heading Value
pub struct CiHeadingValue(Metadata);
impl CiHeadingValue {
    pub const fn new() -> Self {
        Self(Metadata::new(18046, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CiHeadingValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CiHeadingValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Computer
/// Output: 0 if light is off, 1 if light is on
pub struct Ck(Led);
impl Ck {
    pub const fn new() -> Self {
        Self(Led::new(18030, 8, 3))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Ck {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Ck {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// CK37 Rensa Clear
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Ck37RensaClear(Selector);
impl Ck37RensaClear {
    pub const fn new() -> Self {
        Self(Selector::new(17942, 32, 5))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Ck37RensaClear {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Ck37RensaClear {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Clock Setting
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct ClockSet(LimitedDial);
impl ClockSet {
    pub const fn new() -> Self {
        Self(LimitedDial::new(18062, 65535, 0))
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

/// Countermeasure Chaff/Flares Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct CountermeasureChaffFlaresSelector(Selector);
impl CountermeasureChaffFlaresSelector {
    pub const fn new() -> Self {
        Self(Selector::new(17944, 192, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CountermeasureChaffFlaresSelector {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CountermeasureChaffFlaresSelector {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Countermeasure Fast Release
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct CountermeasureFastRelease(Selector);
impl CountermeasureFastRelease {
    pub const fn new() -> Self {
        Self(Selector::new(17940, 8, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CountermeasureFastRelease {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CountermeasureFastRelease {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Countermeasure Operation Mode Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct CountermeasureModeSelector(Selector);
impl CountermeasureModeSelector {
    pub const fn new() -> Self {
        Self(Selector::new(17944, 56, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CountermeasureModeSelector {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CountermeasureModeSelector {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Countermeasure Release Mode
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct CountermeasureReleaseMode(Selector);
impl CountermeasureReleaseMode {
    pub const fn new() -> Self {
        Self(Selector::new(17944, 768, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CountermeasureReleaseMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CountermeasureReleaseMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Countermeasure Streak Mode Selector
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct CountermeasureStreakModeSelector(Selector);
impl CountermeasureStreakModeSelector {
    pub const fn new() -> Self {
        Self(Selector::new(17942, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CountermeasureStreakModeSelector {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CountermeasureStreakModeSelector {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Insert/Remove Data Cartridge
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct DataCartridge(Selector);
impl DataCartridge {
    pub const fn new() -> Self {
        Self(Selector::new(17934, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DataCartridge {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DataCartridge {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Data In/Out
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct DataInOut(Selector);
impl DataInOut {
    pub const fn new() -> Self {
        Self(Selector::new(17942, 8, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DataInOut {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DataInOut {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Datapanel Key 0
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct DatapanelKey0(Selector);
impl DatapanelKey0 {
    pub const fn new() -> Self {
        Self(Selector::new(17922, 256, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DatapanelKey0 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DatapanelKey0 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Datapanel Key 1
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct DatapanelKey1(Selector);
impl DatapanelKey1 {
    pub const fn new() -> Self {
        Self(Selector::new(17922, 512, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DatapanelKey1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DatapanelKey1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Datapanel Key 2
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct DatapanelKey2(Selector);
impl DatapanelKey2 {
    pub const fn new() -> Self {
        Self(Selector::new(17922, 1024, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DatapanelKey2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DatapanelKey2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Datapanel Key 3
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct DatapanelKey3(Selector);
impl DatapanelKey3 {
    pub const fn new() -> Self {
        Self(Selector::new(17922, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DatapanelKey3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DatapanelKey3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Datapanel Key 4
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct DatapanelKey4(Selector);
impl DatapanelKey4 {
    pub const fn new() -> Self {
        Self(Selector::new(17922, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DatapanelKey4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DatapanelKey4 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Datapanel Key 5
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct DatapanelKey5(Selector);
impl DatapanelKey5 {
    pub const fn new() -> Self {
        Self(Selector::new(17922, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DatapanelKey5 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DatapanelKey5 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Datapanel Key 6
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct DatapanelKey6(Selector);
impl DatapanelKey6 {
    pub const fn new() -> Self {
        Self(Selector::new(17922, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DatapanelKey6 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DatapanelKey6 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Datapanel Key 7
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct DatapanelKey7(Selector);
impl DatapanelKey7 {
    pub const fn new() -> Self {
        Self(Selector::new(17922, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DatapanelKey7 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DatapanelKey7 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Datapanel Key 8
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct DatapanelKey8(Selector);
impl DatapanelKey8 {
    pub const fn new() -> Self {
        Self(Selector::new(17924, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DatapanelKey8 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DatapanelKey8 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Datapanel Key 9
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct DatapanelKey9(Selector);
impl DatapanelKey9 {
    pub const fn new() -> Self {
        Self(Selector::new(17924, 2, 1))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DatapanelKey9 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DatapanelKey9 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Datapanel Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct DatapanelSelector(Selector);
impl DatapanelSelector {
    pub const fn new() -> Self {
        Self(Selector::new(17942, 7, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DatapanelSelector {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DatapanelSelector {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Windscreen De-Ice
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct DeIce(LimitedDial);
impl DeIce {
    pub const fn new() -> Self {
        Self(LimitedDial::new(18070, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DeIce {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DeIce {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Distance Indicator
/// Output: gauge position
pub struct DistanceIndicator(AnalogGauge);
impl DistanceIndicator {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(18022, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DistanceIndicator {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DistanceIndicator {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// DME Selector
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct DmeSelector(Selector);
impl DmeSelector {
    pub const fn new() -> Self {
        Self(Selector::new(17934, 16, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DmeSelector {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DmeSelector {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Doppler Land/Sea Mode
/// Input: toggle switch state
/// Output: selector position
pub struct DopplerLandSeaMode(Selector);
impl DopplerLandSeaMode {
    pub const fn new() -> Self {
        Self(Selector::new(17936, 2, 1))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DopplerLandSeaMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DopplerLandSeaMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Drysuit Ventilation Adjustment
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct Drysuit(LimitedDial);
impl Drysuit {
    pub const fn new() -> Self {
        Self(LimitedDial::new(18074, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Drysuit {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Drysuit {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Thrust Reverser Inoperable
/// Output: 0 if light is off, 1 if light is on
pub struct EjRev(Led);
impl EjRev {
    pub const fn new() -> Self {
        Self(Led::new(18028, 8192, 13))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for EjRev {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EjRev {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Ejection Seat Arm
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EjectionSeatArm(Selector);
impl EjectionSeatArm {
    pub const fn new() -> Self {
        Self(Selector::new(17936, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EjectionSeatArm {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EjectionSeatArm {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Electrical System
/// Output: 0 if light is off, 1 if light is on
pub struct Elfel(Led);
impl Elfel {
    pub const fn new() -> Self {
        Self(Led::new(18028, 256, 8))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Elfel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Elfel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Emergency Lights
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EmergencyLights(Selector);
impl EmergencyLights {
    pub const fn new() -> Self {
        Self(Selector::new(17926, 16, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EmergencyLights {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EmergencyLights {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Emergency Pitch Trim
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EmergencyPitchTrim(Selector);
impl EmergencyPitchTrim {
    pub const fn new() -> Self {
        Self(Selector::new(17940, 32, 5))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EmergencyPitchTrim {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EmergencyPitchTrim {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Emergency Roll Trim
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EmergencyRollTrim(Selector);
impl EmergencyRollTrim {
    pub const fn new() -> Self {
        Self(Selector::new(17940, 16, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EmergencyRollTrim {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EmergencyRollTrim {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Emergency Yaw Trim
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EmergencyYawTrim(Selector);
impl EmergencyYawTrim {
    pub const fn new() -> Self {
        Self(Selector::new(17940, 64, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EmergencyYawTrim {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EmergencyYawTrim {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Engine De-Ice
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EngineDeice(Selector);
impl EngineDeice {
    pub const fn new() -> Self {
        Self(Selector::new(17926, 1024, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EngineDeice {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EngineDeice {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Engine RPM 10
/// Output: gauge position
pub struct EngineRpm10(AnalogGauge);
impl EngineRpm10 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17964, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EngineRpm10 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EngineRpm10 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Engine RPM 100
/// Output: gauge position
pub struct EngineRpm100(AnalogGauge);
impl EngineRpm100 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17962, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EngineRpm100 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EngineRpm100 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Engine RPM Value
/// Output: Engine RPM Value
pub struct EngineRpmValue(Metadata);
impl EngineRpmValue {
    pub const fn new() -> Self {
        Self(Metadata::new(18034, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EngineRpmValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EngineRpmValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Engine Temp
/// Output: gauge position
pub struct EngineTemp(AnalogGauge);
impl EngineTemp {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17966, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EngineTemp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EngineTemp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Engine Temp Value
/// Output: Engine Temp Value
pub struct EngineTempValue(Metadata);
impl EngineTempValue {
    pub const fn new() -> Self {
        Self(Metadata::new(18036, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EngineTempValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EngineTempValue {
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
        Self(Metadata::new(18032, 128, 7))
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

/// Navigation Lights Tail
/// Output: Navigation Lights Tail
pub struct ExtNavLightsTail(Metadata);
impl ExtNavLightsTail {
    pub const fn new() -> Self {
        Self(Metadata::new(18032, 32, 5))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ExtNavLightsTail {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ExtNavLightsTail {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Navigation Lights Wing
/// Output: Navigation Lights Wing
pub struct ExtNavLightsWing(Metadata);
impl ExtNavLightsWing {
    pub const fn new() -> Self {
        Self(Metadata::new(18032, 16, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ExtNavLightsWing {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ExtNavLightsWing {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Position Lights
/// Output: Position Lights
pub struct ExtPositionLights(Metadata);
impl ExtPositionLights {
    pub const fn new() -> Self {
        Self(Metadata::new(18032, 8, 3))
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
pub struct ExtSpeedBrakeLeft(Metadata);
impl ExtSpeedBrakeLeft {
    pub const fn new() -> Self {
        Self(Metadata::new(18052, 65535, 0))
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
        Self(Metadata::new(18050, 65535, 0))
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
pub struct ExtStrobeLights(Metadata);
impl ExtStrobeLights {
    pub const fn new() -> Self {
        Self(Metadata::new(18032, 64, 6))
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

/// Flares Empty
/// Output: 0 if light is off, 1 if light is on
pub struct FacklSl(Led);
impl FacklSl {
    pub const fn new() -> Self {
        Self(Led::new(18032, 1, 0))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for FacklSl {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FacklSl {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flight Recorder
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct FlightRecorder(Selector);
impl FlightRecorder {
    pub const fn new() -> Self {
        Self(Selector::new(17934, 384, 7))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FlightRecorder {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FlightRecorder {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flood Lights
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct FloodLights(LimitedDial);
impl FloodLights {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17928, 65535, 0))
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

/// Formation Lights
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FormationLights(Selector);
impl FormationLights {
    pub const fn new() -> Self {
        Self(Selector::new(17924, 32768, 15))
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

/// Thrust Reverser Warning
/// Output: 0 if light is off, 1 if light is on
pub struct ForvForb(Led);
impl ForvForb {
    pub const fn new() -> Self {
        Self(Led::new(18028, 8, 3))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ForvForb {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ForvForb {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// FR22 Base Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct Fr22Base(Selector);
impl Fr22Base {
    pub const fn new() -> Self {
        Self(Selector::new(18066, 120, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22Base {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22Base {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Channel A/G
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Fr22ChannelAg(Selector);
impl Fr22ChannelAg {
    pub const fn new() -> Self {
        Self(Selector::new(18064, 1024, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22ChannelAg {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22ChannelAg {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Channel B
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Fr22ChannelB(Selector);
impl Fr22ChannelB {
    pub const fn new() -> Self {
        Self(Selector::new(18064, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22ChannelB {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22ChannelB {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Channel C2
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Fr22ChannelC2(Selector);
impl Fr22ChannelC2 {
    pub const fn new() -> Self {
        Self(Selector::new(18064, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22ChannelC2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22ChannelC2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Channel C/F
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Fr22ChannelCf(Selector);
impl Fr22ChannelCf {
    pub const fn new() -> Self {
        Self(Selector::new(18064, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22ChannelCf {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22ChannelCf {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Channel D/E
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Fr22ChannelDe(Selector);
impl Fr22ChannelDe {
    pub const fn new() -> Self {
        Self(Selector::new(18064, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22ChannelDe {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22ChannelDe {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Channel H
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Fr22ChannelH(Selector);
impl Fr22ChannelH {
    pub const fn new() -> Self {
        Self(Selector::new(18064, 32, 5))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22ChannelH {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22ChannelH {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flight 0
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Fr22Flight0(Selector);
impl Fr22Flight0 {
    pub const fn new() -> Self {
        Self(Selector::new(18032, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22Flight0 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22Flight0 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flight 1
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Fr22Flight1(Selector);
impl Fr22Flight1 {
    pub const fn new() -> Self {
        Self(Selector::new(18032, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22Flight1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22Flight1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flight 2
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Fr22Flight2(Selector);
impl Fr22Flight2 {
    pub const fn new() -> Self {
        Self(Selector::new(18032, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22Flight2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22Flight2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flight 3
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Fr22Flight3(Selector);
impl Fr22Flight3 {
    pub const fn new() -> Self {
        Self(Selector::new(18032, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22Flight3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22Flight3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flight 4
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Fr22Flight4(Selector);
impl Fr22Flight4 {
    pub const fn new() -> Self {
        Self(Selector::new(18032, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22Flight4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22Flight4 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flight 5
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Fr22Flight5(Selector);
impl Fr22Flight5 {
    pub const fn new() -> Self {
        Self(Selector::new(18064, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22Flight5 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22Flight5 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flight 6
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Fr22Flight6(Selector);
impl Fr22Flight6 {
    pub const fn new() -> Self {
        Self(Selector::new(18064, 2, 1))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22Flight6 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22Flight6 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flight 7
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Fr22Flight7(Selector);
impl Fr22Flight7 {
    pub const fn new() -> Self {
        Self(Selector::new(18064, 4, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22Flight7 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22Flight7 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flight 8
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Fr22Flight8(Selector);
impl Fr22Flight8 {
    pub const fn new() -> Self {
        Self(Selector::new(18064, 8, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22Flight8 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22Flight8 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flight 9
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Fr22Flight9(Selector);
impl Fr22Flight9 {
    pub const fn new() -> Self {
        Self(Selector::new(18064, 16, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22Flight9 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22Flight9 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Ground Intercom
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Fr22GroundCom(Selector);
impl Fr22GroundCom {
    pub const fn new() -> Self {
        Self(Selector::new(18064, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22GroundCom {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22GroundCom {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// FR22 Group Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct Fr22Group(Selector);
impl Fr22Group {
    pub const fn new() -> Self {
        Self(Selector::new(18066, 1920, 7))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22Group {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22Group {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Radio Frequency Knob Inner Left
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct Fr22InnerLeftKnob(AnalogDial);
impl Fr22InnerLeftKnob {
    pub const fn new() -> Self {
        Self(AnalogDial::new(17946, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22InnerLeftKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22InnerLeftKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Radio Frequency Knob Inner Right
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct Fr22InnerRightKnob(AnalogDial);
impl Fr22InnerRightKnob {
    pub const fn new() -> Self {
        Self(AnalogDial::new(17950, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22InnerRightKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22InnerRightKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Minus
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Fr22Minus(Selector);
impl Fr22Minus {
    pub const fn new() -> Self {
        Self(Selector::new(18064, 512, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22Minus {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22Minus {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// FR22 Mode Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct Fr22Mode(Selector);
impl Fr22Mode {
    pub const fn new() -> Self {
        Self(Selector::new(18066, 7, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22Mode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22Mode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Radio Frequency Knob Outer Left
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct Fr22OuterLeftKnob(AnalogDial);
impl Fr22OuterLeftKnob {
    pub const fn new() -> Self {
        Self(AnalogDial::new(17948, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22OuterLeftKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22OuterLeftKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Radio Frequency Knob Outer Right
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct Fr22OuterRightKnob(AnalogDial);
impl Fr22OuterRightKnob {
    pub const fn new() -> Self {
        Self(AnalogDial::new(17952, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22OuterRightKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22OuterRightKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Radio Manual Frequency Setting Modulation
/// Input: toggle switch state
/// Output: selector position
pub struct Fr22SetModulation(Selector);
impl Fr22SetModulation {
    pub const fn new() -> Self {
        Self(Selector::new(17942, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22SetModulation {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22SetModulation {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Special 1
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Fr22Special1(Selector);
impl Fr22Special1 {
    pub const fn new() -> Self {
        Self(Selector::new(18064, 64, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22Special1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22Special1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Special 2
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Fr22Special2(Selector);
impl Fr22Special2 {
    pub const fn new() -> Self {
        Self(Selector::new(18064, 128, 7))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22Special2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22Special2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Special 3
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Fr22Special3(Selector);
impl Fr22Special3 {
    pub const fn new() -> Self {
        Self(Selector::new(18064, 256, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22Special3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22Special3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Radio Volume
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct Fr22Vol(AnalogDial);
impl Fr22Vol {
    pub const fn new() -> Self {
        Self(AnalogDial::new(18068, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fr22Vol {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fr22Vol {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuel Level
/// Output: gauge position
pub struct FuelLevel(AnalogGauge);
impl FuelLevel {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(18024, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelLevel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelLevel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuel Level Value
/// Output: Fuel Level Value
pub struct FuelLevelValue(Metadata);
impl FuelLevelValue {
    pub const fn new() -> Self {
        Self(Metadata::new(18042, 65535, 0))
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

/// Fuel Needed
/// Output: gauge position
pub struct FuelNeeded(AnalogGauge);
impl FuelNeeded {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(18026, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelNeeded {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelNeeded {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuel Needed Value
/// Output: Fuel Needed Value
pub struct FuelNeededValue(Metadata);
impl FuelNeededValue {
    pub const fn new() -> Self {
        Self(Metadata::new(18044, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelNeededValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelNeededValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Gear Handle
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct GearHandle(Selector);
impl GearHandle {
    pub const fn new() -> Self {
        Self(Selector::new(17936, 4, 2))
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

/// Generator
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Generator(Selector);
impl Generator {
    pub const fn new() -> Self {
        Self(Selector::new(17934, 8192, 13))
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

/// Autopilot Hold
/// Output: 0 if light is off, 1 if light is on
pub struct HallFunk(Led);
impl HallFunk {
    pub const fn new() -> Self {
        Self(Led::new(18030, 1, 0))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for HallFunk {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HallFunk {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// High Pressure Fuel Valve
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct HighPresFuelValve(Selector);
impl HighPresFuelValve {
    pub const fn new() -> Self {
        Self(Selector::new(17926, 512, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HighPresFuelValve {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HighPresFuelValve {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// HOJD CISI
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct HojdCisi(Selector);
impl HojdCisi {
    pub const fn new() -> Self {
        Self(Selector::new(17936, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HojdCisi {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HojdCisi {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// HOJD Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct HojdLamp(Led);
impl HojdLamp {
    pub const fn new() -> Self {
        Self(Led::new(17944, 4096, 12))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for HojdLamp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HojdLamp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Right Gear
/// Output: 0 if light is off, 1 if light is on
pub struct Hstall(Led);
impl Hstall {
    pub const fn new() -> Self {
        Self(Led::new(18028, 64, 6))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Hstall {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Hstall {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// HUD Brightness Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct HudBright(LimitedDial);
impl HudBright {
    pub const fn new() -> Self {
        Self(LimitedDial::new(18056, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HudBright {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HudBright {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// HUD Reflector Glass Position
/// Input: toggle switch state
/// Output: selector position
pub struct HudGlassPosition(Selector);
impl HudGlassPosition {
    pub const fn new() -> Self {
        Self(Selector::new(17940, 1024, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HudGlassPosition {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HudGlassPosition {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Ejection Seat/Canopy
/// Output: 0 if light is off, 1 if light is on
pub struct HuvOStol(Led);
impl HuvOStol {
    pub const fn new() -> Self {
        Self(Led::new(18030, 32, 5))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for HuvOStol {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HuvOStol {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Hydraulic System 1 Pressure
/// Output: 0 if light is off, 1 if light is on
pub struct HydraTa1(Led);
impl HydraTa1 {
    pub const fn new() -> Self {
        Self(Led::new(18028, 2048, 11))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for HydraTa1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HydraTa1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Hydraulic System 2 Pressure
/// Output: 0 if light is off, 1 if light is on
pub struct HydraTa2(Led);
impl HydraTa2 {
    pub const fn new() -> Self {
        Self(Led::new(18028, 1024, 10))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for HydraTa2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HydraTa2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// IFF Channel Selector
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct IffChannel(Selector);
impl IffChannel {
    pub const fn new() -> Self {
        Self(Selector::new(17934, 8, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for IffChannel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IffChannel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// IFF Code
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct IffCode(Selector);
impl IffCode {
    pub const fn new() -> Self {
        Self(Selector::new(18066, 61440, 12))
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

/// IFF Power
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct IffPower(Selector);
impl IffPower {
    pub const fn new() -> Self {
        Self(Selector::new(18066, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for IffPower {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IffPower {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// IFF/Transponder Power
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct IffTransponderPower(Selector);
impl IffTransponderPower {
    pub const fn new() -> Self {
        Self(Selector::new(17934, 4, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for IffTransponderPower {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IffTransponderPower {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Ignition System
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct IgnitionSystem(Selector);
impl IgnitionSystem {
    pub const fn new() -> Self {
        Self(Selector::new(17934, 32, 5))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for IgnitionSystem {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IgnitionSystem {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Indicator System Test
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct IndicatorSystemTest(Selector);
impl IndicatorSystemTest {
    pub const fn new() -> Self {
        Self(Selector::new(17942, 512, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for IndicatorSystemTest {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IndicatorSystemTest {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Instrument Lights
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct InstrumentLights(LimitedDial);
impl InstrumentLights {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17932, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InstrumentLights {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InstrumentLights {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Jammer Band Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct JammerBandSelector(Selector);
impl JammerBandSelector {
    pub const fn new() -> Self {
        Self(Selector::new(17944, 7, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for JammerBandSelector {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JammerBandSelector {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Jammer Mode Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct JammerModeSelector(Selector);
impl JammerModeSelector {
    pub const fn new() -> Self {
        Self(Selector::new(17942, 14336, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for JammerModeSelector {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JammerModeSelector {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Cabin Pressure
/// Output: 0 if light is off, 1 if light is on
pub struct Kabinhojd(Led);
impl Kabinhojd {
    pub const fn new() -> Self {
        Self(Led::new(18030, 16, 4))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Kabinhojd {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Kabinhojd {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Right Countermeasures Pod Empty/ECM Failure
/// Output: 0 if light is off, 1 if light is on
pub struct KbHKaSl(Led);
impl KbHKaSl {
    pub const fn new() -> Self {
        Self(Led::new(18030, 32768, 15))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for KbHKaSl {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for KbHKaSl {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Left Countermeasures Pod Empty
/// Output: 0 if light is off, 1 if light is on
pub struct KbVSlut(Led);
impl KbVSlut {
    pub const fn new() -> Self {
        Self(Led::new(18030, 16384, 14))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for KbVSlut {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for KbVSlut {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Landing Gear
/// Output: 0 if light is off, 1 if light is on
pub struct Landstall(Led);
impl Landstall {
    pub const fn new() -> Self {
        Self(Led::new(18028, 4, 2))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Landstall {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Landstall {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Low Pressure Fuel Valve
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct LowPresFuelValve(Selector);
impl LowPresFuelValve {
    pub const fn new() -> Self {
        Self(Selector::new(17926, 256, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LowPresFuelValve {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LowPresFuelValve {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Airbrakes
/// Output: 0 if light is off, 1 if light is on
pub struct Luftbroms(Led);
impl Luftbroms {
    pub const fn new() -> Self {
        Self(Led::new(18032, 4, 2))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Luftbroms {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Luftbroms {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Mach Meter First Decimal 0.X0
/// Output: gauge position
pub struct MachMeterFirstDecimal(AnalogGauge);
impl MachMeterFirstDecimal {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17972, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MachMeterFirstDecimal {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MachMeterFirstDecimal {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Mach Meter Integer X.00
/// Output: gauge position
pub struct MachMeterInteger(AnalogGauge);
impl MachMeterInteger {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17970, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MachMeterInteger {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MachMeterInteger {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Mach Meter Second Decimal 0.0X
/// Output: gauge position
pub struct MachMeterSecondDecimal(AnalogGauge);
impl MachMeterSecondDecimal {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17974, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MachMeterSecondDecimal {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MachMeterSecondDecimal {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Magnetic Declination Correction
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct MagCorrection(LimitedDial);
impl MagCorrection {
    pub const fn new() -> Self {
        Self(LimitedDial::new(18058, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MagCorrection {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MagCorrection {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Magnetic Declination Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MagDecCover(Selector);
impl MagDecCover {
    pub const fn new() -> Self {
        Self(Selector::new(17940, 128, 7))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MagDecCover {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MagDecCover {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Magnetic Heading
/// Output: gauge position
pub struct MagneticHeading(AnalogGauge);
impl MagneticHeading {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(18010, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MagneticHeading {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MagneticHeading {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Main Electric Power
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MainElectricPower(Selector);
impl MainElectricPower {
    pub const fn new() -> Self {
        Self(Selector::new(17934, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MainElectricPower {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MainElectricPower {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Manual Afterburner Fuel Regulator
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct ManAfterburnFuelReg(Selector);
impl ManAfterburnFuelReg {
    pub const fn new() -> Self {
        Self(Selector::new(17934, 64, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ManAfterburnFuelReg {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ManAfterburnFuelReg {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Manual Fuel Regulator
/// Output: 0 if light is off, 1 if light is on
pub struct ManBgReg(Led);
impl ManBgReg {
    pub const fn new() -> Self {
        Self(Led::new(18030, 256, 8))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ManBgReg {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ManBgReg {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Manual Fuel Regulator
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct ManualFuelReg(Selector);
impl ManualFuelReg {
    pub const fn new() -> Self {
        Self(Selector::new(17926, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ManualFuelReg {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ManualFuelReg {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Master Caution Reset
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MasterCautionReset(Selector);
impl MasterCautionReset {
    pub const fn new() -> Self {
        Self(Selector::new(17942, 1024, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MasterCautionReset {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MasterCautionReset {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Master Mode Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct MasterModeSelect(Selector);
impl MasterModeSelect {
    pub const fn new() -> Self {
        Self(Selector::new(17940, 57344, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MasterModeSelect {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MasterModeSelect {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Master Volume / Sidewinder Tone
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct MasterVol(LimitedDial);
impl MasterVol {
    pub const fn new() -> Self {
        Self(LimitedDial::new(18078, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MasterVol {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MasterVol {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Max G Reset
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MaxGReset(Selector);
impl MaxGReset {
    pub const fn new() -> Self {
        Self(Selector::new(18032, 256, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MaxGReset {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MaxGReset {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Missile Select Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MissileSelectButton(Selector);
impl MissileSelectButton {
    pub const fn new() -> Self {
        Self(Selector::new(17934, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MissileSelectButton {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MissileSelectButton {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Countermeasures/RWR
/// Output: 0 if light is off, 1 if light is on
pub struct Motverk(Led);
impl Motverk {
    pub const fn new() -> Self {
        Self(Led::new(18032, 2, 1))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Motverk {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Motverk {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Navigation Selector Button B1
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct NavSelectBtnB1(Selector);
impl NavSelectBtnB1 {
    pub const fn new() -> Self {
        Self(Selector::new(17924, 4, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NavSelectBtnB1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NavSelectBtnB1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Navigation Selector Button B2
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct NavSelectBtnB2(Selector);
impl NavSelectBtnB2 {
    pub const fn new() -> Self {
        Self(Selector::new(17924, 8, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NavSelectBtnB2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NavSelectBtnB2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Navigation Selector Button B3
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct NavSelectBtnB3(Selector);
impl NavSelectBtnB3 {
    pub const fn new() -> Self {
        Self(Selector::new(17924, 16, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NavSelectBtnB3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NavSelectBtnB3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Navigation Selector Button B4
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct NavSelectBtnB4(Selector);
impl NavSelectBtnB4 {
    pub const fn new() -> Self {
        Self(Selector::new(17924, 32, 5))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NavSelectBtnB4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NavSelectBtnB4 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Navigation Selector Button B5
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct NavSelectBtnB5(Selector);
impl NavSelectBtnB5 {
    pub const fn new() -> Self {
        Self(Selector::new(17924, 64, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NavSelectBtnB5 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NavSelectBtnB5 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Navigation Selector Button B6
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct NavSelectBtnB6(Selector);
impl NavSelectBtnB6 {
    pub const fn new() -> Self {
        Self(Selector::new(17924, 128, 7))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NavSelectBtnB6 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NavSelectBtnB6 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Navigation Selector Button B7
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct NavSelectBtnB7(Selector);
impl NavSelectBtnB7 {
    pub const fn new() -> Self {
        Self(Selector::new(17924, 256, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NavSelectBtnB7 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NavSelectBtnB7 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Navigation Selector Button B8
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct NavSelectBtnB8(Selector);
impl NavSelectBtnB8 {
    pub const fn new() -> Self {
        Self(Selector::new(17924, 512, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NavSelectBtnB8 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NavSelectBtnB8 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Navigation Selector Button B9
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct NavSelectBtnB9(Selector);
impl NavSelectBtnB9 {
    pub const fn new() -> Self {
        Self(Selector::new(17924, 1024, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NavSelectBtnB9 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NavSelectBtnB9 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Navigation Selector Button BX
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct NavSelectBtnBx(Selector);
impl NavSelectBtnBx {
    pub const fn new() -> Self {
        Self(Selector::new(17924, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NavSelectBtnBx {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NavSelectBtnBx {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Navigation Selector Button L MAL
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct NavSelectBtnLMal(Selector);
impl NavSelectBtnLMal {
    pub const fn new() -> Self {
        Self(Selector::new(17924, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NavSelectBtnLMal {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NavSelectBtnLMal {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Navigation Selector Button LS
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct NavSelectBtnLs(Selector);
impl NavSelectBtnLs {
    pub const fn new() -> Self {
        Self(Selector::new(17924, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NavSelectBtnLs {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NavSelectBtnLs {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Navigation System
/// Output: 0 if light is off, 1 if light is on
pub struct NavSyst(Led);
impl NavSyst {
    pub const fn new() -> Self {
        Self(Led::new(18030, 8192, 13))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for NavSyst {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NavSyst {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Navigation Lights
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct NavigationLights(Selector);
impl NavigationLights {
    pub const fn new() -> Self {
        Self(Selector::new(17926, 3, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NavigationLights {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NavigationLights {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Nose Gear
/// Output: 0 if light is off, 1 if light is on
pub struct Nosstall(Led);
impl Nosstall {
    pub const fn new() -> Self {
        Self(Led::new(18028, 16, 4))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Nosstall {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Nosstall {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Oil Pressure
/// Output: 0 if light is off, 1 if light is on
pub struct Oljetryck(Led);
impl Oljetryck {
    pub const fn new() -> Self {
        Self(Led::new(18028, 16384, 14))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Oljetryck {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Oljetryck {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Oxygen Lever
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct OxygenLever(Selector);
impl OxygenLever {
    pub const fn new() -> Self {
        Self(Selector::new(17936, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OxygenLever {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OxygenLever {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Panel Lights
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct PanelLights(LimitedDial);
impl PanelLights {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17930, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PanelLights {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PanelLights {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Parking Brake
/// Input: toggle switch state
/// Output: selector position
pub struct ParkingBrake(Selector);
impl ParkingBrake {
    pub const fn new() -> Self {
        Self(Selector::new(17940, 512, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ParkingBrake {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ParkingBrake {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Pedals
/// Output: gauge position
pub struct Pedals(AnalogGauge);
impl Pedals {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17958, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Pedals {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Pedals {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Pitch Gear Automatic/Landing
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct PitchGearMode(Selector);
impl PitchGearMode {
    pub const fn new() -> Self {
        Self(Selector::new(17936, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PitchGearMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PitchGearMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Position Lights
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct PositionLights(Selector);
impl PositionLights {
    pub const fn new() -> Self {
        Self(Selector::new(17926, 4, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PositionLights {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PositionLights {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Position Lights Brightness
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct PositionLightsBrightness(Selector);
impl PositionLightsBrightness {
    pub const fn new() -> Self {
        Self(Selector::new(17926, 96, 5))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PositionLightsBrightness {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PositionLightsBrightness {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Radar Altimeter Power
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RadarAltimeterPower(Selector);
impl RadarAltimeterPower {
    pub const fn new() -> Self {
        Self(Selector::new(17936, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadarAltimeterPower {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadarAltimeterPower {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Radar Brightness
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct RadarBright(LimitedDial);
impl RadarBright {
    pub const fn new() -> Self {
        Self(LimitedDial::new(18080, 65535, 0))
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

/// Lin/Log Radar Gain Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RadarGain(Selector);
impl RadarGain {
    pub const fn new() -> Self {
        Self(Selector::new(17920, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadarGain {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadarGain {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// IFF Identification
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RadarIffId(Selector);
impl RadarIffId {
    pub const fn new() -> Self {
        Self(Selector::new(17922, 64, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadarIffId {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadarIffId {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Ignition Coils
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RadarIgnitionCoils(Selector);
impl RadarIgnitionCoils {
    pub const fn new() -> Self {
        Self(Selector::new(17922, 32, 5))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadarIgnitionCoils {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadarIgnitionCoils {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Radar/EL Maintenance Test
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RadarMaintenanceTest(Selector);
impl RadarMaintenanceTest {
    pub const fn new() -> Self {
        Self(Selector::new(17922, 16, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadarMaintenanceTest {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadarMaintenanceTest {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Pulse Normal/Short Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RadarPulseNormalShort(Selector);
impl RadarPulseNormalShort {
    pub const fn new() -> Self {
        Self(Selector::new(17920, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadarPulseNormalShort {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadarPulseNormalShort {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Passive Recce On/Off Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RadarRecceOnOff(Selector);
impl RadarRecceOnOff {
    pub const fn new() -> Self {
        Self(Selector::new(17922, 8, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadarRecceOnOff {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadarRecceOnOff {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Radar Warning Indication Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct RadarWarnSelect(Selector);
impl RadarWarnSelect {
    pub const fn new() -> Self {
        Self(Selector::new(17942, 192, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadarWarnSelect {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadarWarnSelect {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// RB-04/RB-15/BK Release Mode Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RbBkRelMode(Selector);
impl RbBkRelMode {
    pub const fn new() -> Self {
        Self(Selector::new(17920, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RbBkRelMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RbBkRelMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Rensa Button Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RensaButtonCover(Selector);
impl RensaButtonCover {
    pub const fn new() -> Self {
        Self(Selector::new(17942, 16, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RensaButtonCover {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RensaButtonCover {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Backup Hydraulic
/// Output: 0 if light is off, 1 if light is on
pub struct Reserveff(Led);
impl Reserveff {
    pub const fn new() -> Self {
        Self(Led::new(18028, 512, 9))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Reserveff {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Reserveff {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Restart
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Restart(Selector);
impl Restart {
    pub const fn new() -> Self {
        Self(Selector::new(17934, 512, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Restart {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Restart {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Thrust Reverser On/Off
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Reversal(Selector);
impl Reversal {
    pub const fn new() -> Self {
        Self(Selector::new(17922, 128, 7))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Reversal {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Reversal {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Radar Altimeter
/// Output: 0 if light is off, 1 if light is on
pub struct RhmFel(Led);
impl RhmFel {
    pub const fn new() -> Self {
        Self(Led::new(18030, 2, 1))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for RhmFel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RhmFel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Roll Centering
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RollCenter(Selector);
impl RollCenter {
    pub const fn new() -> Self {
        Self(Selector::new(18032, 1024, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RollCenter {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RollCenter {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Roll Gearing
/// Output: 0 if light is off, 1 if light is on
pub struct RollVaxel(Led);
impl RollVaxel {
    pub const fn new() -> Self {
        Self(Led::new(18030, 4, 2))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for RollVaxel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RollVaxel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Slav SI
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct SlavSi(Selector);
impl SlavSi {
    pub const fn new() -> Self {
        Self(Selector::new(17936, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SlavSi {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SlavSi {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// SPAK
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Spak(Selector);
impl Spak {
    pub const fn new() -> Self {
        Self(Selector::new(17936, 8, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Spak {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Spak {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// SPAK Error
/// Output: 0 if light is off, 1 if light is on
pub struct SpakError(Led);
impl SpakError {
    pub const fn new() -> Self {
        Self(Led::new(18028, 32768, 15))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for SpakError {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SpakError {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// SPAK Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct SpakLamp(Led);
impl SpakLamp {
    pub const fn new() -> Self {
        Self(Led::new(17944, 1024, 10))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for SpakLamp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SpakLamp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Start System
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct StartSystem(Selector);
impl StartSystem {
    pub const fn new() -> Self {
        Self(Selector::new(17926, 128, 7))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for StartSystem {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for StartSystem {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Starter System
/// Output: 0 if light is off, 1 if light is on
pub struct Startsyst(Led);
impl Startsyst {
    pub const fn new() -> Self {
        Self(Led::new(18030, 128, 7))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Startsyst {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Startsyst {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Stick Pitch
/// Output: gauge position
pub struct StickPitch(AnalogGauge);
impl StickPitch {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17954, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for StickPitch {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for StickPitch {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Stick Roll
/// Output: gauge position
pub struct StickRoll(AnalogGauge);
impl StickRoll {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17956, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for StickRoll {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for StickRoll {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Oxygen
/// Output: 0 if light is off, 1 if light is on
pub struct Syrgas(Led);
impl Syrgas {
    pub const fn new() -> Self {
        Self(Led::new(18030, 512, 9))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Syrgas {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Syrgas {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Ignition System
/// Output: 0 if light is off, 1 if light is on
pub struct Tandsyst(Led);
impl Tandsyst {
    pub const fn new() -> Self {
        Self(Led::new(18030, 64, 6))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Tandsyst {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Tandsyst {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuel Pump
/// Output: 0 if light is off, 1 if light is on
pub struct TankPump(Led);
impl TankPump {
    pub const fn new() -> Self {
        Self(Led::new(18028, 2, 1))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for TankPump {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TankPump {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// External Tank Release Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct TankRelease(Selector);
impl TankRelease {
    pub const fn new() -> Self {
        Self(Selector::new(17920, 16, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TankRelease {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TankRelease {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// External Tank Release Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct TankReleaseCover(Selector);
impl TankReleaseCover {
    pub const fn new() -> Self {
        Self(Selector::new(17920, 8, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TankReleaseCover {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TankReleaseCover {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Taxi/Landing Lights
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct TaxiLandingLights(Selector);
impl TaxiLandingLights {
    pub const fn new() -> Self {
        Self(Selector::new(17926, 8, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TaxiLandingLights {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TaxiLandingLights {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Maintenance Testing Mode
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct TestMode(LimitedDial);
impl TestMode {
    pub const fn new() -> Self {
        Self(LimitedDial::new(18072, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TestMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TestMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Throttle
/// Output: gauge position
pub struct Throttle(AnalogGauge);
impl Throttle {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17960, 65535, 0))
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

/// TILS
/// Output: 0 if light is off, 1 if light is on
pub struct Tils(Led);
impl Tils {
    pub const fn new() -> Self {
        Self(Led::new(18030, 4096, 12))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Tils {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Tils {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// TILS Channel Layer Selection
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct TilsChannelLayer(Selector);
impl TilsChannelLayer {
    pub const fn new() -> Self {
        Self(Selector::new(17936, 1024, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TilsChannelLayer {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TilsChannelLayer {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// TILS Channel Selection
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct TilsChannelSelect(Selector);
impl TilsChannelSelect {
    pub const fn new() -> Self {
        Self(Selector::new(17936, 960, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TilsChannelSelect {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TilsChannelSelect {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Pitch Gearing
/// Output: 0 if light is off, 1 if light is on
pub struct Tippvaxel(Led);
impl Tippvaxel {
    pub const fn new() -> Self {
        Self(Led::new(18028, 128, 7))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Tippvaxel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Tippvaxel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Trigger Safety Bracket
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct TriggerSafetyBracket(Selector);
impl TriggerSafetyBracket {
    pub const fn new() -> Self {
        Self(Selector::new(17920, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TriggerSafetyBracket {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TriggerSafetyBracket {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Vertical Acceleration G Meter
/// Output: gauge position
pub struct VerticalAcceleration(AnalogGauge);
impl VerticalAcceleration {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17976, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VerticalAcceleration {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VerticalAcceleration {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Left Gear
/// Output: 0 if light is off, 1 if light is on
pub struct Vstall(Led);
impl Vstall {
    pub const fn new() -> Self {
        Self(Led::new(18028, 32, 5))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Vstall {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Vstall {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Warning Panel Light Test
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct WarningPanelTest(Selector);
impl WarningPanelTest {
    pub const fn new() -> Self {
        Self(Selector::new(17942, 256, 8))
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

/// Weapon Interval Selector Mode Knob
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct WeaponInterval(Selector);
impl WeaponInterval {
    pub const fn new() -> Self {
        Self(Selector::new(17920, 3840, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for WeaponInterval {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for WeaponInterval {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Weapon Release Mode Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct WeaponRelMode(Selector);
impl WeaponRelMode {
    pub const fn new() -> Self {
        Self(Selector::new(17920, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for WeaponRelMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for WeaponRelMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Weapon Emergency Release Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct WeaponRelease(Selector);
impl WeaponRelease {
    pub const fn new() -> Self {
        Self(Selector::new(17920, 4, 2))
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

/// Weapon Emergency Release Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct WeaponReleaseCover(Selector);
impl WeaponReleaseCover {
    pub const fn new() -> Self {
        Self(Selector::new(17920, 2, 1))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for WeaponReleaseCover {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for WeaponReleaseCover {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Weapon Selector Knob
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct WeaponSelect(Selector);
impl WeaponSelect {
    pub const fn new() -> Self {
        Self(Selector::new(17920, 224, 5))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for WeaponSelect {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for WeaponSelect {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// External Fuel Tank Feed System
/// Output: 0 if light is off, 1 if light is on
pub struct XTankBra(Led);
impl XTankBra {
    pub const fn new() -> Self {
        Self(Led::new(18028, 1, 0))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for XTankBra {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for XTankBra {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Autopilot Yaw Trim Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct YawTrimCover(Selector);
impl YawTrimCover {
    pub const fn new() -> Self {
        Self(Selector::new(17940, 256, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for YawTrimCover {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for YawTrimCover {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

pub enum RegisteredControl<'a> {
    AdiAoaIndicator {
        control: AdiAoaIndicator,
        callback: IntegerCallback<'a>,
    },
    AdiHeading {
        control: AdiHeading,
        callback: IntegerCallback<'a>,
    },
    AdiHorizontalIls {
        control: AdiHorizontalIls,
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
    AdiSlipball {
        control: AdiSlipball,
        callback: IntegerCallback<'a>,
    },
    AdiVerticalIls {
        control: AdiVerticalIls,
        callback: IntegerCallback<'a>,
    },
    AdiVerticalVelocity {
        control: AdiVerticalVelocity,
        callback: IntegerCallback<'a>,
    },
    Afk15DegMode {
        control: Afk15DegMode,
        callback: IntegerCallback<'a>,
    },
    AfkFel {
        control: AfkFel,
        callback: LedCallback<'a>,
    },
    AfkLever {
        control: AfkLever,
        callback: IntegerCallback<'a>,
    },
    AfkMode3 {
        control: AfkMode3,
        callback: IntegerCallback<'a>,
    },
    AirspeedMS {
        control: AirspeedMS,
        callback: IntegerCallback<'a>,
    },
    AirspeedValue {
        control: AirspeedValue,
        callback: IntegerCallback<'a>,
    },
    Ajs37DestIndicatorData1 {
        control: Ajs37DestIndicatorData1,
        callback: DisplayCallback<'a>,
    },
    Ajs37DestIndicatorData2 {
        control: Ajs37DestIndicatorData2,
        callback: DisplayCallback<'a>,
    },
    Ajs37NavIndicatorData1 {
        control: Ajs37NavIndicatorData1,
        callback: DisplayCallback<'a>,
    },
    Ajs37NavIndicatorData2 {
        control: Ajs37NavIndicatorData2,
        callback: DisplayCallback<'a>,
    },
    Ajs37NavIndicatorData3 {
        control: Ajs37NavIndicatorData3,
        callback: DisplayCallback<'a>,
    },
    Ajs37NavIndicatorData4 {
        control: Ajs37NavIndicatorData4,
        callback: DisplayCallback<'a>,
    },
    Ajs37NavIndicatorData5 {
        control: Ajs37NavIndicatorData5,
        callback: DisplayCallback<'a>,
    },
    Ajs37NavIndicatorData6 {
        control: Ajs37NavIndicatorData6,
        callback: DisplayCallback<'a>,
    },
    AltSet {
        control: AltSet,
        callback: IntegerCallback<'a>,
    },
    Altimeter10000m {
        control: Altimeter10000m,
        callback: IntegerCallback<'a>,
    },
    Altimeter1000m {
        control: Altimeter1000m,
        callback: IntegerCallback<'a>,
    },
    AltimeterBaro1000Hpa {
        control: AltimeterBaro1000Hpa,
        callback: IntegerCallback<'a>,
    },
    AltimeterBaro100Hpa {
        control: AltimeterBaro100Hpa,
        callback: IntegerCallback<'a>,
    },
    AltimeterBaro10Hpa {
        control: AltimeterBaro10Hpa,
        callback: IntegerCallback<'a>,
    },
    AltimeterBaro1Hpa {
        control: AltimeterBaro1Hpa,
        callback: IntegerCallback<'a>,
    },
    AltitudeHold {
        control: AltitudeHold,
        callback: IntegerCallback<'a>,
    },
    AltitudeValue {
        control: AltitudeValue,
        callback: IntegerCallback<'a>,
    },
    AntiCollisionLights {
        control: AntiCollisionLights,
        callback: IntegerCallback<'a>,
    },
    AntiJamMode {
        control: AntiJamMode,
        callback: IntegerCallback<'a>,
    },
    AttLamp {
        control: AttLamp,
        callback: LedCallback<'a>,
    },
    AttitudeHold {
        control: AttitudeHold,
        callback: IntegerCallback<'a>,
    },
    AutoYawTrim {
        control: AutoYawTrim,
        callback: IntegerCallback<'a>,
    },
    BackAdiCage {
        control: BackAdiCage,
        callback: IntegerCallback<'a>,
    },
    BackupAltSetting {
        control: BackupAltSetting,
        callback: IntegerCallback<'a>,
    },
    BackupAltimeter10000m {
        control: BackupAltimeter10000m,
        callback: IntegerCallback<'a>,
    },
    BackupAltimeter1000m {
        control: BackupAltimeter1000m,
        callback: IntegerCallback<'a>,
    },
    BackupGenerator {
        control: BackupGenerator,
        callback: IntegerCallback<'a>,
    },
    BackupIndicatedAirspeed {
        control: BackupIndicatedAirspeed,
        callback: IntegerCallback<'a>,
    },
    BackupPitch {
        control: BackupPitch,
        callback: IntegerCallback<'a>,
    },
    BackupRoll {
        control: BackupRoll,
        callback: IntegerCallback<'a>,
    },
    Bra24 {
        control: Bra24,
        callback: LedCallback<'a>,
    },
    BraUppf {
        control: BraUppf,
        callback: LedCallback<'a>,
    },
    Brand1 {
        control: Brand1,
        callback: LedCallback<'a>,
    },
    Brand2 {
        control: Brand2,
        callback: LedCallback<'a>,
    },
    BrandGts {
        control: BrandGts,
        callback: LedCallback<'a>,
    },
    CabinAirValve {
        control: CabinAirValve,
        callback: IntegerCallback<'a>,
    },
    CanopyJettison {
        control: CanopyJettison,
        callback: IntegerCallback<'a>,
    },
    CanopyOpenClose {
        control: CanopyOpenClose,
        callback: IntegerCallback<'a>,
    },
    CbAutopilot {
        control: CbAutopilot,
        callback: IntegerCallback<'a>,
    },
    CbCiSi {
        control: CbCiSi,
        callback: IntegerCallback<'a>,
    },
    CbEjectionSystem {
        control: CbEjectionSystem,
        callback: IntegerCallback<'a>,
    },
    CbEngine {
        control: CbEngine,
        callback: IntegerCallback<'a>,
    },
    CbHighAlphaWarn {
        control: CbHighAlphaWarn,
        callback: IntegerCallback<'a>,
    },
    CbTrimSystem {
        control: CbTrimSystem,
        callback: IntegerCallback<'a>,
    },
    CiCommandHeadingValue {
        control: CiCommandHeadingValue,
        callback: IntegerCallback<'a>,
    },
    CiCommandedHeading {
        control: CiCommandedHeading,
        callback: IntegerCallback<'a>,
    },
    CiHeading {
        control: CiHeading,
        callback: IntegerCallback<'a>,
    },
    CiHeadingValue {
        control: CiHeadingValue,
        callback: IntegerCallback<'a>,
    },
    Ck {
        control: Ck,
        callback: LedCallback<'a>,
    },
    Ck37RensaClear {
        control: Ck37RensaClear,
        callback: IntegerCallback<'a>,
    },
    ClockSet {
        control: ClockSet,
        callback: IntegerCallback<'a>,
    },
    CountermeasureChaffFlaresSelector {
        control: CountermeasureChaffFlaresSelector,
        callback: IntegerCallback<'a>,
    },
    CountermeasureFastRelease {
        control: CountermeasureFastRelease,
        callback: IntegerCallback<'a>,
    },
    CountermeasureModeSelector {
        control: CountermeasureModeSelector,
        callback: IntegerCallback<'a>,
    },
    CountermeasureReleaseMode {
        control: CountermeasureReleaseMode,
        callback: IntegerCallback<'a>,
    },
    CountermeasureStreakModeSelector {
        control: CountermeasureStreakModeSelector,
        callback: IntegerCallback<'a>,
    },
    DataCartridge {
        control: DataCartridge,
        callback: IntegerCallback<'a>,
    },
    DataInOut {
        control: DataInOut,
        callback: IntegerCallback<'a>,
    },
    DatapanelKey0 {
        control: DatapanelKey0,
        callback: IntegerCallback<'a>,
    },
    DatapanelKey1 {
        control: DatapanelKey1,
        callback: IntegerCallback<'a>,
    },
    DatapanelKey2 {
        control: DatapanelKey2,
        callback: IntegerCallback<'a>,
    },
    DatapanelKey3 {
        control: DatapanelKey3,
        callback: IntegerCallback<'a>,
    },
    DatapanelKey4 {
        control: DatapanelKey4,
        callback: IntegerCallback<'a>,
    },
    DatapanelKey5 {
        control: DatapanelKey5,
        callback: IntegerCallback<'a>,
    },
    DatapanelKey6 {
        control: DatapanelKey6,
        callback: IntegerCallback<'a>,
    },
    DatapanelKey7 {
        control: DatapanelKey7,
        callback: IntegerCallback<'a>,
    },
    DatapanelKey8 {
        control: DatapanelKey8,
        callback: IntegerCallback<'a>,
    },
    DatapanelKey9 {
        control: DatapanelKey9,
        callback: IntegerCallback<'a>,
    },
    DatapanelSelector {
        control: DatapanelSelector,
        callback: IntegerCallback<'a>,
    },
    DeIce {
        control: DeIce,
        callback: IntegerCallback<'a>,
    },
    DistanceIndicator {
        control: DistanceIndicator,
        callback: IntegerCallback<'a>,
    },
    DmeSelector {
        control: DmeSelector,
        callback: IntegerCallback<'a>,
    },
    DopplerLandSeaMode {
        control: DopplerLandSeaMode,
        callback: IntegerCallback<'a>,
    },
    Drysuit {
        control: Drysuit,
        callback: IntegerCallback<'a>,
    },
    EjRev {
        control: EjRev,
        callback: LedCallback<'a>,
    },
    EjectionSeatArm {
        control: EjectionSeatArm,
        callback: IntegerCallback<'a>,
    },
    Elfel {
        control: Elfel,
        callback: LedCallback<'a>,
    },
    EmergencyLights {
        control: EmergencyLights,
        callback: IntegerCallback<'a>,
    },
    EmergencyPitchTrim {
        control: EmergencyPitchTrim,
        callback: IntegerCallback<'a>,
    },
    EmergencyRollTrim {
        control: EmergencyRollTrim,
        callback: IntegerCallback<'a>,
    },
    EmergencyYawTrim {
        control: EmergencyYawTrim,
        callback: IntegerCallback<'a>,
    },
    EngineDeice {
        control: EngineDeice,
        callback: IntegerCallback<'a>,
    },
    EngineRpm10 {
        control: EngineRpm10,
        callback: IntegerCallback<'a>,
    },
    EngineRpm100 {
        control: EngineRpm100,
        callback: IntegerCallback<'a>,
    },
    EngineRpmValue {
        control: EngineRpmValue,
        callback: IntegerCallback<'a>,
    },
    EngineTemp {
        control: EngineTemp,
        callback: IntegerCallback<'a>,
    },
    EngineTempValue {
        control: EngineTempValue,
        callback: IntegerCallback<'a>,
    },
    ExtFormationLights {
        control: ExtFormationLights,
        callback: IntegerCallback<'a>,
    },
    ExtNavLightsTail {
        control: ExtNavLightsTail,
        callback: IntegerCallback<'a>,
    },
    ExtNavLightsWing {
        control: ExtNavLightsWing,
        callback: IntegerCallback<'a>,
    },
    ExtPositionLights {
        control: ExtPositionLights,
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
    ExtStrobeLights {
        control: ExtStrobeLights,
        callback: IntegerCallback<'a>,
    },
    FacklSl {
        control: FacklSl,
        callback: LedCallback<'a>,
    },
    FlightRecorder {
        control: FlightRecorder,
        callback: IntegerCallback<'a>,
    },
    FloodLights {
        control: FloodLights,
        callback: IntegerCallback<'a>,
    },
    FormationLights {
        control: FormationLights,
        callback: IntegerCallback<'a>,
    },
    ForvForb {
        control: ForvForb,
        callback: LedCallback<'a>,
    },
    Fr22Base {
        control: Fr22Base,
        callback: IntegerCallback<'a>,
    },
    Fr22ChannelAg {
        control: Fr22ChannelAg,
        callback: IntegerCallback<'a>,
    },
    Fr22ChannelB {
        control: Fr22ChannelB,
        callback: IntegerCallback<'a>,
    },
    Fr22ChannelC2 {
        control: Fr22ChannelC2,
        callback: IntegerCallback<'a>,
    },
    Fr22ChannelCf {
        control: Fr22ChannelCf,
        callback: IntegerCallback<'a>,
    },
    Fr22ChannelDe {
        control: Fr22ChannelDe,
        callback: IntegerCallback<'a>,
    },
    Fr22ChannelH {
        control: Fr22ChannelH,
        callback: IntegerCallback<'a>,
    },
    Fr22Flight0 {
        control: Fr22Flight0,
        callback: IntegerCallback<'a>,
    },
    Fr22Flight1 {
        control: Fr22Flight1,
        callback: IntegerCallback<'a>,
    },
    Fr22Flight2 {
        control: Fr22Flight2,
        callback: IntegerCallback<'a>,
    },
    Fr22Flight3 {
        control: Fr22Flight3,
        callback: IntegerCallback<'a>,
    },
    Fr22Flight4 {
        control: Fr22Flight4,
        callback: IntegerCallback<'a>,
    },
    Fr22Flight5 {
        control: Fr22Flight5,
        callback: IntegerCallback<'a>,
    },
    Fr22Flight6 {
        control: Fr22Flight6,
        callback: IntegerCallback<'a>,
    },
    Fr22Flight7 {
        control: Fr22Flight7,
        callback: IntegerCallback<'a>,
    },
    Fr22Flight8 {
        control: Fr22Flight8,
        callback: IntegerCallback<'a>,
    },
    Fr22Flight9 {
        control: Fr22Flight9,
        callback: IntegerCallback<'a>,
    },
    Fr22GroundCom {
        control: Fr22GroundCom,
        callback: IntegerCallback<'a>,
    },
    Fr22Group {
        control: Fr22Group,
        callback: IntegerCallback<'a>,
    },
    Fr22InnerLeftKnob {
        control: Fr22InnerLeftKnob,
        callback: IntegerCallback<'a>,
    },
    Fr22InnerRightKnob {
        control: Fr22InnerRightKnob,
        callback: IntegerCallback<'a>,
    },
    Fr22Minus {
        control: Fr22Minus,
        callback: IntegerCallback<'a>,
    },
    Fr22Mode {
        control: Fr22Mode,
        callback: IntegerCallback<'a>,
    },
    Fr22OuterLeftKnob {
        control: Fr22OuterLeftKnob,
        callback: IntegerCallback<'a>,
    },
    Fr22OuterRightKnob {
        control: Fr22OuterRightKnob,
        callback: IntegerCallback<'a>,
    },
    Fr22SetModulation {
        control: Fr22SetModulation,
        callback: IntegerCallback<'a>,
    },
    Fr22Special1 {
        control: Fr22Special1,
        callback: IntegerCallback<'a>,
    },
    Fr22Special2 {
        control: Fr22Special2,
        callback: IntegerCallback<'a>,
    },
    Fr22Special3 {
        control: Fr22Special3,
        callback: IntegerCallback<'a>,
    },
    Fr22Vol {
        control: Fr22Vol,
        callback: IntegerCallback<'a>,
    },
    FuelLevel {
        control: FuelLevel,
        callback: IntegerCallback<'a>,
    },
    FuelLevelValue {
        control: FuelLevelValue,
        callback: IntegerCallback<'a>,
    },
    FuelNeeded {
        control: FuelNeeded,
        callback: IntegerCallback<'a>,
    },
    FuelNeededValue {
        control: FuelNeededValue,
        callback: IntegerCallback<'a>,
    },
    GearHandle {
        control: GearHandle,
        callback: IntegerCallback<'a>,
    },
    Generator {
        control: Generator,
        callback: IntegerCallback<'a>,
    },
    HallFunk {
        control: HallFunk,
        callback: LedCallback<'a>,
    },
    HighPresFuelValve {
        control: HighPresFuelValve,
        callback: IntegerCallback<'a>,
    },
    HojdCisi {
        control: HojdCisi,
        callback: IntegerCallback<'a>,
    },
    HojdLamp {
        control: HojdLamp,
        callback: LedCallback<'a>,
    },
    Hstall {
        control: Hstall,
        callback: LedCallback<'a>,
    },
    HudBright {
        control: HudBright,
        callback: IntegerCallback<'a>,
    },
    HudGlassPosition {
        control: HudGlassPosition,
        callback: IntegerCallback<'a>,
    },
    HuvOStol {
        control: HuvOStol,
        callback: LedCallback<'a>,
    },
    HydraTa1 {
        control: HydraTa1,
        callback: LedCallback<'a>,
    },
    HydraTa2 {
        control: HydraTa2,
        callback: LedCallback<'a>,
    },
    IffChannel {
        control: IffChannel,
        callback: IntegerCallback<'a>,
    },
    IffCode {
        control: IffCode,
        callback: IntegerCallback<'a>,
    },
    IffPower {
        control: IffPower,
        callback: IntegerCallback<'a>,
    },
    IffTransponderPower {
        control: IffTransponderPower,
        callback: IntegerCallback<'a>,
    },
    IgnitionSystem {
        control: IgnitionSystem,
        callback: IntegerCallback<'a>,
    },
    IndicatorSystemTest {
        control: IndicatorSystemTest,
        callback: IntegerCallback<'a>,
    },
    InstrumentLights {
        control: InstrumentLights,
        callback: IntegerCallback<'a>,
    },
    JammerBandSelector {
        control: JammerBandSelector,
        callback: IntegerCallback<'a>,
    },
    JammerModeSelector {
        control: JammerModeSelector,
        callback: IntegerCallback<'a>,
    },
    Kabinhojd {
        control: Kabinhojd,
        callback: LedCallback<'a>,
    },
    KbHKaSl {
        control: KbHKaSl,
        callback: LedCallback<'a>,
    },
    KbVSlut {
        control: KbVSlut,
        callback: LedCallback<'a>,
    },
    Landstall {
        control: Landstall,
        callback: LedCallback<'a>,
    },
    LowPresFuelValve {
        control: LowPresFuelValve,
        callback: IntegerCallback<'a>,
    },
    Luftbroms {
        control: Luftbroms,
        callback: LedCallback<'a>,
    },
    MachMeterFirstDecimal {
        control: MachMeterFirstDecimal,
        callback: IntegerCallback<'a>,
    },
    MachMeterInteger {
        control: MachMeterInteger,
        callback: IntegerCallback<'a>,
    },
    MachMeterSecondDecimal {
        control: MachMeterSecondDecimal,
        callback: IntegerCallback<'a>,
    },
    MagCorrection {
        control: MagCorrection,
        callback: IntegerCallback<'a>,
    },
    MagDecCover {
        control: MagDecCover,
        callback: IntegerCallback<'a>,
    },
    MagneticHeading {
        control: MagneticHeading,
        callback: IntegerCallback<'a>,
    },
    MainElectricPower {
        control: MainElectricPower,
        callback: IntegerCallback<'a>,
    },
    ManAfterburnFuelReg {
        control: ManAfterburnFuelReg,
        callback: IntegerCallback<'a>,
    },
    ManBgReg {
        control: ManBgReg,
        callback: LedCallback<'a>,
    },
    ManualFuelReg {
        control: ManualFuelReg,
        callback: IntegerCallback<'a>,
    },
    MasterCautionReset {
        control: MasterCautionReset,
        callback: IntegerCallback<'a>,
    },
    MasterModeSelect {
        control: MasterModeSelect,
        callback: IntegerCallback<'a>,
    },
    MasterVol {
        control: MasterVol,
        callback: IntegerCallback<'a>,
    },
    MaxGReset {
        control: MaxGReset,
        callback: IntegerCallback<'a>,
    },
    MissileSelectButton {
        control: MissileSelectButton,
        callback: IntegerCallback<'a>,
    },
    Motverk {
        control: Motverk,
        callback: LedCallback<'a>,
    },
    NavSelectBtnB1 {
        control: NavSelectBtnB1,
        callback: IntegerCallback<'a>,
    },
    NavSelectBtnB2 {
        control: NavSelectBtnB2,
        callback: IntegerCallback<'a>,
    },
    NavSelectBtnB3 {
        control: NavSelectBtnB3,
        callback: IntegerCallback<'a>,
    },
    NavSelectBtnB4 {
        control: NavSelectBtnB4,
        callback: IntegerCallback<'a>,
    },
    NavSelectBtnB5 {
        control: NavSelectBtnB5,
        callback: IntegerCallback<'a>,
    },
    NavSelectBtnB6 {
        control: NavSelectBtnB6,
        callback: IntegerCallback<'a>,
    },
    NavSelectBtnB7 {
        control: NavSelectBtnB7,
        callback: IntegerCallback<'a>,
    },
    NavSelectBtnB8 {
        control: NavSelectBtnB8,
        callback: IntegerCallback<'a>,
    },
    NavSelectBtnB9 {
        control: NavSelectBtnB9,
        callback: IntegerCallback<'a>,
    },
    NavSelectBtnBx {
        control: NavSelectBtnBx,
        callback: IntegerCallback<'a>,
    },
    NavSelectBtnLMal {
        control: NavSelectBtnLMal,
        callback: IntegerCallback<'a>,
    },
    NavSelectBtnLs {
        control: NavSelectBtnLs,
        callback: IntegerCallback<'a>,
    },
    NavSyst {
        control: NavSyst,
        callback: LedCallback<'a>,
    },
    NavigationLights {
        control: NavigationLights,
        callback: IntegerCallback<'a>,
    },
    Nosstall {
        control: Nosstall,
        callback: LedCallback<'a>,
    },
    Oljetryck {
        control: Oljetryck,
        callback: LedCallback<'a>,
    },
    OxygenLever {
        control: OxygenLever,
        callback: IntegerCallback<'a>,
    },
    PanelLights {
        control: PanelLights,
        callback: IntegerCallback<'a>,
    },
    ParkingBrake {
        control: ParkingBrake,
        callback: IntegerCallback<'a>,
    },
    Pedals {
        control: Pedals,
        callback: IntegerCallback<'a>,
    },
    PitchGearMode {
        control: PitchGearMode,
        callback: IntegerCallback<'a>,
    },
    PositionLights {
        control: PositionLights,
        callback: IntegerCallback<'a>,
    },
    PositionLightsBrightness {
        control: PositionLightsBrightness,
        callback: IntegerCallback<'a>,
    },
    RadarAltimeterPower {
        control: RadarAltimeterPower,
        callback: IntegerCallback<'a>,
    },
    RadarBright {
        control: RadarBright,
        callback: IntegerCallback<'a>,
    },
    RadarGain {
        control: RadarGain,
        callback: IntegerCallback<'a>,
    },
    RadarIffId {
        control: RadarIffId,
        callback: IntegerCallback<'a>,
    },
    RadarIgnitionCoils {
        control: RadarIgnitionCoils,
        callback: IntegerCallback<'a>,
    },
    RadarMaintenanceTest {
        control: RadarMaintenanceTest,
        callback: IntegerCallback<'a>,
    },
    RadarPulseNormalShort {
        control: RadarPulseNormalShort,
        callback: IntegerCallback<'a>,
    },
    RadarRecceOnOff {
        control: RadarRecceOnOff,
        callback: IntegerCallback<'a>,
    },
    RadarWarnSelect {
        control: RadarWarnSelect,
        callback: IntegerCallback<'a>,
    },
    RbBkRelMode {
        control: RbBkRelMode,
        callback: IntegerCallback<'a>,
    },
    RensaButtonCover {
        control: RensaButtonCover,
        callback: IntegerCallback<'a>,
    },
    Reserveff {
        control: Reserveff,
        callback: LedCallback<'a>,
    },
    Restart {
        control: Restart,
        callback: IntegerCallback<'a>,
    },
    Reversal {
        control: Reversal,
        callback: IntegerCallback<'a>,
    },
    RhmFel {
        control: RhmFel,
        callback: LedCallback<'a>,
    },
    RollCenter {
        control: RollCenter,
        callback: IntegerCallback<'a>,
    },
    RollVaxel {
        control: RollVaxel,
        callback: LedCallback<'a>,
    },
    SlavSi {
        control: SlavSi,
        callback: IntegerCallback<'a>,
    },
    Spak {
        control: Spak,
        callback: IntegerCallback<'a>,
    },
    SpakError {
        control: SpakError,
        callback: LedCallback<'a>,
    },
    SpakLamp {
        control: SpakLamp,
        callback: LedCallback<'a>,
    },
    StartSystem {
        control: StartSystem,
        callback: IntegerCallback<'a>,
    },
    Startsyst {
        control: Startsyst,
        callback: LedCallback<'a>,
    },
    StickPitch {
        control: StickPitch,
        callback: IntegerCallback<'a>,
    },
    StickRoll {
        control: StickRoll,
        callback: IntegerCallback<'a>,
    },
    Syrgas {
        control: Syrgas,
        callback: LedCallback<'a>,
    },
    Tandsyst {
        control: Tandsyst,
        callback: LedCallback<'a>,
    },
    TankPump {
        control: TankPump,
        callback: LedCallback<'a>,
    },
    TankRelease {
        control: TankRelease,
        callback: IntegerCallback<'a>,
    },
    TankReleaseCover {
        control: TankReleaseCover,
        callback: IntegerCallback<'a>,
    },
    TaxiLandingLights {
        control: TaxiLandingLights,
        callback: IntegerCallback<'a>,
    },
    TestMode {
        control: TestMode,
        callback: IntegerCallback<'a>,
    },
    Throttle {
        control: Throttle,
        callback: IntegerCallback<'a>,
    },
    Tils {
        control: Tils,
        callback: LedCallback<'a>,
    },
    TilsChannelLayer {
        control: TilsChannelLayer,
        callback: IntegerCallback<'a>,
    },
    TilsChannelSelect {
        control: TilsChannelSelect,
        callback: IntegerCallback<'a>,
    },
    Tippvaxel {
        control: Tippvaxel,
        callback: LedCallback<'a>,
    },
    TriggerSafetyBracket {
        control: TriggerSafetyBracket,
        callback: IntegerCallback<'a>,
    },
    VerticalAcceleration {
        control: VerticalAcceleration,
        callback: IntegerCallback<'a>,
    },
    Vstall {
        control: Vstall,
        callback: LedCallback<'a>,
    },
    WarningPanelTest {
        control: WarningPanelTest,
        callback: IntegerCallback<'a>,
    },
    WeaponInterval {
        control: WeaponInterval,
        callback: IntegerCallback<'a>,
    },
    WeaponRelMode {
        control: WeaponRelMode,
        callback: IntegerCallback<'a>,
    },
    WeaponRelease {
        control: WeaponRelease,
        callback: IntegerCallback<'a>,
    },
    WeaponReleaseCover {
        control: WeaponReleaseCover,
        callback: IntegerCallback<'a>,
    },
    WeaponSelect {
        control: WeaponSelect,
        callback: IntegerCallback<'a>,
    },
    XTankBra {
        control: XTankBra,
        callback: LedCallback<'a>,
    },
    YawTrimCover {
        control: YawTrimCover,
        callback: IntegerCallback<'a>,
    },
}

impl RegisteredControl<'_> {
    fn accept(&mut self, packet: Packet<'_>) {
        match self {
            Self::AdiAoaIndicator { control, .. } => control.accept(packet),
            Self::AdiHeading { control, .. } => control.accept(packet),
            Self::AdiHorizontalIls { control, .. } => control.accept(packet),
            Self::AdiPitch { control, .. } => control.accept(packet),
            Self::AdiRoll { control, .. } => control.accept(packet),
            Self::AdiSlipball { control, .. } => control.accept(packet),
            Self::AdiVerticalIls { control, .. } => control.accept(packet),
            Self::AdiVerticalVelocity { control, .. } => control.accept(packet),
            Self::Afk15DegMode { control, .. } => control.accept(packet),
            Self::AfkFel { control, .. } => control.accept(packet),
            Self::AfkLever { control, .. } => control.accept(packet),
            Self::AfkMode3 { control, .. } => control.accept(packet),
            Self::AirspeedMS { control, .. } => control.accept(packet),
            Self::AirspeedValue { control, .. } => control.accept(packet),
            Self::Ajs37DestIndicatorData1 { control, .. } => control.accept(packet),
            Self::Ajs37DestIndicatorData2 { control, .. } => control.accept(packet),
            Self::Ajs37NavIndicatorData1 { control, .. } => control.accept(packet),
            Self::Ajs37NavIndicatorData2 { control, .. } => control.accept(packet),
            Self::Ajs37NavIndicatorData3 { control, .. } => control.accept(packet),
            Self::Ajs37NavIndicatorData4 { control, .. } => control.accept(packet),
            Self::Ajs37NavIndicatorData5 { control, .. } => control.accept(packet),
            Self::Ajs37NavIndicatorData6 { control, .. } => control.accept(packet),
            Self::AltSet { control, .. } => control.accept(packet),
            Self::Altimeter10000m { control, .. } => control.accept(packet),
            Self::Altimeter1000m { control, .. } => control.accept(packet),
            Self::AltimeterBaro1000Hpa { control, .. } => control.accept(packet),
            Self::AltimeterBaro100Hpa { control, .. } => control.accept(packet),
            Self::AltimeterBaro10Hpa { control, .. } => control.accept(packet),
            Self::AltimeterBaro1Hpa { control, .. } => control.accept(packet),
            Self::AltitudeHold { control, .. } => control.accept(packet),
            Self::AltitudeValue { control, .. } => control.accept(packet),
            Self::AntiCollisionLights { control, .. } => control.accept(packet),
            Self::AntiJamMode { control, .. } => control.accept(packet),
            Self::AttLamp { control, .. } => control.accept(packet),
            Self::AttitudeHold { control, .. } => control.accept(packet),
            Self::AutoYawTrim { control, .. } => control.accept(packet),
            Self::BackAdiCage { control, .. } => control.accept(packet),
            Self::BackupAltSetting { control, .. } => control.accept(packet),
            Self::BackupAltimeter10000m { control, .. } => control.accept(packet),
            Self::BackupAltimeter1000m { control, .. } => control.accept(packet),
            Self::BackupGenerator { control, .. } => control.accept(packet),
            Self::BackupIndicatedAirspeed { control, .. } => control.accept(packet),
            Self::BackupPitch { control, .. } => control.accept(packet),
            Self::BackupRoll { control, .. } => control.accept(packet),
            Self::Bra24 { control, .. } => control.accept(packet),
            Self::BraUppf { control, .. } => control.accept(packet),
            Self::Brand1 { control, .. } => control.accept(packet),
            Self::Brand2 { control, .. } => control.accept(packet),
            Self::BrandGts { control, .. } => control.accept(packet),
            Self::CabinAirValve { control, .. } => control.accept(packet),
            Self::CanopyJettison { control, .. } => control.accept(packet),
            Self::CanopyOpenClose { control, .. } => control.accept(packet),
            Self::CbAutopilot { control, .. } => control.accept(packet),
            Self::CbCiSi { control, .. } => control.accept(packet),
            Self::CbEjectionSystem { control, .. } => control.accept(packet),
            Self::CbEngine { control, .. } => control.accept(packet),
            Self::CbHighAlphaWarn { control, .. } => control.accept(packet),
            Self::CbTrimSystem { control, .. } => control.accept(packet),
            Self::CiCommandHeadingValue { control, .. } => control.accept(packet),
            Self::CiCommandedHeading { control, .. } => control.accept(packet),
            Self::CiHeading { control, .. } => control.accept(packet),
            Self::CiHeadingValue { control, .. } => control.accept(packet),
            Self::Ck { control, .. } => control.accept(packet),
            Self::Ck37RensaClear { control, .. } => control.accept(packet),
            Self::ClockSet { control, .. } => control.accept(packet),
            Self::CountermeasureChaffFlaresSelector { control, .. } => control.accept(packet),
            Self::CountermeasureFastRelease { control, .. } => control.accept(packet),
            Self::CountermeasureModeSelector { control, .. } => control.accept(packet),
            Self::CountermeasureReleaseMode { control, .. } => control.accept(packet),
            Self::CountermeasureStreakModeSelector { control, .. } => control.accept(packet),
            Self::DataCartridge { control, .. } => control.accept(packet),
            Self::DataInOut { control, .. } => control.accept(packet),
            Self::DatapanelKey0 { control, .. } => control.accept(packet),
            Self::DatapanelKey1 { control, .. } => control.accept(packet),
            Self::DatapanelKey2 { control, .. } => control.accept(packet),
            Self::DatapanelKey3 { control, .. } => control.accept(packet),
            Self::DatapanelKey4 { control, .. } => control.accept(packet),
            Self::DatapanelKey5 { control, .. } => control.accept(packet),
            Self::DatapanelKey6 { control, .. } => control.accept(packet),
            Self::DatapanelKey7 { control, .. } => control.accept(packet),
            Self::DatapanelKey8 { control, .. } => control.accept(packet),
            Self::DatapanelKey9 { control, .. } => control.accept(packet),
            Self::DatapanelSelector { control, .. } => control.accept(packet),
            Self::DeIce { control, .. } => control.accept(packet),
            Self::DistanceIndicator { control, .. } => control.accept(packet),
            Self::DmeSelector { control, .. } => control.accept(packet),
            Self::DopplerLandSeaMode { control, .. } => control.accept(packet),
            Self::Drysuit { control, .. } => control.accept(packet),
            Self::EjRev { control, .. } => control.accept(packet),
            Self::EjectionSeatArm { control, .. } => control.accept(packet),
            Self::Elfel { control, .. } => control.accept(packet),
            Self::EmergencyLights { control, .. } => control.accept(packet),
            Self::EmergencyPitchTrim { control, .. } => control.accept(packet),
            Self::EmergencyRollTrim { control, .. } => control.accept(packet),
            Self::EmergencyYawTrim { control, .. } => control.accept(packet),
            Self::EngineDeice { control, .. } => control.accept(packet),
            Self::EngineRpm10 { control, .. } => control.accept(packet),
            Self::EngineRpm100 { control, .. } => control.accept(packet),
            Self::EngineRpmValue { control, .. } => control.accept(packet),
            Self::EngineTemp { control, .. } => control.accept(packet),
            Self::EngineTempValue { control, .. } => control.accept(packet),
            Self::ExtFormationLights { control, .. } => control.accept(packet),
            Self::ExtNavLightsTail { control, .. } => control.accept(packet),
            Self::ExtNavLightsWing { control, .. } => control.accept(packet),
            Self::ExtPositionLights { control, .. } => control.accept(packet),
            Self::ExtSpeedBrakeLeft { control, .. } => control.accept(packet),
            Self::ExtSpeedBrakeRight { control, .. } => control.accept(packet),
            Self::ExtStrobeLights { control, .. } => control.accept(packet),
            Self::FacklSl { control, .. } => control.accept(packet),
            Self::FlightRecorder { control, .. } => control.accept(packet),
            Self::FloodLights { control, .. } => control.accept(packet),
            Self::FormationLights { control, .. } => control.accept(packet),
            Self::ForvForb { control, .. } => control.accept(packet),
            Self::Fr22Base { control, .. } => control.accept(packet),
            Self::Fr22ChannelAg { control, .. } => control.accept(packet),
            Self::Fr22ChannelB { control, .. } => control.accept(packet),
            Self::Fr22ChannelC2 { control, .. } => control.accept(packet),
            Self::Fr22ChannelCf { control, .. } => control.accept(packet),
            Self::Fr22ChannelDe { control, .. } => control.accept(packet),
            Self::Fr22ChannelH { control, .. } => control.accept(packet),
            Self::Fr22Flight0 { control, .. } => control.accept(packet),
            Self::Fr22Flight1 { control, .. } => control.accept(packet),
            Self::Fr22Flight2 { control, .. } => control.accept(packet),
            Self::Fr22Flight3 { control, .. } => control.accept(packet),
            Self::Fr22Flight4 { control, .. } => control.accept(packet),
            Self::Fr22Flight5 { control, .. } => control.accept(packet),
            Self::Fr22Flight6 { control, .. } => control.accept(packet),
            Self::Fr22Flight7 { control, .. } => control.accept(packet),
            Self::Fr22Flight8 { control, .. } => control.accept(packet),
            Self::Fr22Flight9 { control, .. } => control.accept(packet),
            Self::Fr22GroundCom { control, .. } => control.accept(packet),
            Self::Fr22Group { control, .. } => control.accept(packet),
            Self::Fr22InnerLeftKnob { control, .. } => control.accept(packet),
            Self::Fr22InnerRightKnob { control, .. } => control.accept(packet),
            Self::Fr22Minus { control, .. } => control.accept(packet),
            Self::Fr22Mode { control, .. } => control.accept(packet),
            Self::Fr22OuterLeftKnob { control, .. } => control.accept(packet),
            Self::Fr22OuterRightKnob { control, .. } => control.accept(packet),
            Self::Fr22SetModulation { control, .. } => control.accept(packet),
            Self::Fr22Special1 { control, .. } => control.accept(packet),
            Self::Fr22Special2 { control, .. } => control.accept(packet),
            Self::Fr22Special3 { control, .. } => control.accept(packet),
            Self::Fr22Vol { control, .. } => control.accept(packet),
            Self::FuelLevel { control, .. } => control.accept(packet),
            Self::FuelLevelValue { control, .. } => control.accept(packet),
            Self::FuelNeeded { control, .. } => control.accept(packet),
            Self::FuelNeededValue { control, .. } => control.accept(packet),
            Self::GearHandle { control, .. } => control.accept(packet),
            Self::Generator { control, .. } => control.accept(packet),
            Self::HallFunk { control, .. } => control.accept(packet),
            Self::HighPresFuelValve { control, .. } => control.accept(packet),
            Self::HojdCisi { control, .. } => control.accept(packet),
            Self::HojdLamp { control, .. } => control.accept(packet),
            Self::Hstall { control, .. } => control.accept(packet),
            Self::HudBright { control, .. } => control.accept(packet),
            Self::HudGlassPosition { control, .. } => control.accept(packet),
            Self::HuvOStol { control, .. } => control.accept(packet),
            Self::HydraTa1 { control, .. } => control.accept(packet),
            Self::HydraTa2 { control, .. } => control.accept(packet),
            Self::IffChannel { control, .. } => control.accept(packet),
            Self::IffCode { control, .. } => control.accept(packet),
            Self::IffPower { control, .. } => control.accept(packet),
            Self::IffTransponderPower { control, .. } => control.accept(packet),
            Self::IgnitionSystem { control, .. } => control.accept(packet),
            Self::IndicatorSystemTest { control, .. } => control.accept(packet),
            Self::InstrumentLights { control, .. } => control.accept(packet),
            Self::JammerBandSelector { control, .. } => control.accept(packet),
            Self::JammerModeSelector { control, .. } => control.accept(packet),
            Self::Kabinhojd { control, .. } => control.accept(packet),
            Self::KbHKaSl { control, .. } => control.accept(packet),
            Self::KbVSlut { control, .. } => control.accept(packet),
            Self::Landstall { control, .. } => control.accept(packet),
            Self::LowPresFuelValve { control, .. } => control.accept(packet),
            Self::Luftbroms { control, .. } => control.accept(packet),
            Self::MachMeterFirstDecimal { control, .. } => control.accept(packet),
            Self::MachMeterInteger { control, .. } => control.accept(packet),
            Self::MachMeterSecondDecimal { control, .. } => control.accept(packet),
            Self::MagCorrection { control, .. } => control.accept(packet),
            Self::MagDecCover { control, .. } => control.accept(packet),
            Self::MagneticHeading { control, .. } => control.accept(packet),
            Self::MainElectricPower { control, .. } => control.accept(packet),
            Self::ManAfterburnFuelReg { control, .. } => control.accept(packet),
            Self::ManBgReg { control, .. } => control.accept(packet),
            Self::ManualFuelReg { control, .. } => control.accept(packet),
            Self::MasterCautionReset { control, .. } => control.accept(packet),
            Self::MasterModeSelect { control, .. } => control.accept(packet),
            Self::MasterVol { control, .. } => control.accept(packet),
            Self::MaxGReset { control, .. } => control.accept(packet),
            Self::MissileSelectButton { control, .. } => control.accept(packet),
            Self::Motverk { control, .. } => control.accept(packet),
            Self::NavSelectBtnB1 { control, .. } => control.accept(packet),
            Self::NavSelectBtnB2 { control, .. } => control.accept(packet),
            Self::NavSelectBtnB3 { control, .. } => control.accept(packet),
            Self::NavSelectBtnB4 { control, .. } => control.accept(packet),
            Self::NavSelectBtnB5 { control, .. } => control.accept(packet),
            Self::NavSelectBtnB6 { control, .. } => control.accept(packet),
            Self::NavSelectBtnB7 { control, .. } => control.accept(packet),
            Self::NavSelectBtnB8 { control, .. } => control.accept(packet),
            Self::NavSelectBtnB9 { control, .. } => control.accept(packet),
            Self::NavSelectBtnBx { control, .. } => control.accept(packet),
            Self::NavSelectBtnLMal { control, .. } => control.accept(packet),
            Self::NavSelectBtnLs { control, .. } => control.accept(packet),
            Self::NavSyst { control, .. } => control.accept(packet),
            Self::NavigationLights { control, .. } => control.accept(packet),
            Self::Nosstall { control, .. } => control.accept(packet),
            Self::Oljetryck { control, .. } => control.accept(packet),
            Self::OxygenLever { control, .. } => control.accept(packet),
            Self::PanelLights { control, .. } => control.accept(packet),
            Self::ParkingBrake { control, .. } => control.accept(packet),
            Self::Pedals { control, .. } => control.accept(packet),
            Self::PitchGearMode { control, .. } => control.accept(packet),
            Self::PositionLights { control, .. } => control.accept(packet),
            Self::PositionLightsBrightness { control, .. } => control.accept(packet),
            Self::RadarAltimeterPower { control, .. } => control.accept(packet),
            Self::RadarBright { control, .. } => control.accept(packet),
            Self::RadarGain { control, .. } => control.accept(packet),
            Self::RadarIffId { control, .. } => control.accept(packet),
            Self::RadarIgnitionCoils { control, .. } => control.accept(packet),
            Self::RadarMaintenanceTest { control, .. } => control.accept(packet),
            Self::RadarPulseNormalShort { control, .. } => control.accept(packet),
            Self::RadarRecceOnOff { control, .. } => control.accept(packet),
            Self::RadarWarnSelect { control, .. } => control.accept(packet),
            Self::RbBkRelMode { control, .. } => control.accept(packet),
            Self::RensaButtonCover { control, .. } => control.accept(packet),
            Self::Reserveff { control, .. } => control.accept(packet),
            Self::Restart { control, .. } => control.accept(packet),
            Self::Reversal { control, .. } => control.accept(packet),
            Self::RhmFel { control, .. } => control.accept(packet),
            Self::RollCenter { control, .. } => control.accept(packet),
            Self::RollVaxel { control, .. } => control.accept(packet),
            Self::SlavSi { control, .. } => control.accept(packet),
            Self::Spak { control, .. } => control.accept(packet),
            Self::SpakError { control, .. } => control.accept(packet),
            Self::SpakLamp { control, .. } => control.accept(packet),
            Self::StartSystem { control, .. } => control.accept(packet),
            Self::Startsyst { control, .. } => control.accept(packet),
            Self::StickPitch { control, .. } => control.accept(packet),
            Self::StickRoll { control, .. } => control.accept(packet),
            Self::Syrgas { control, .. } => control.accept(packet),
            Self::Tandsyst { control, .. } => control.accept(packet),
            Self::TankPump { control, .. } => control.accept(packet),
            Self::TankRelease { control, .. } => control.accept(packet),
            Self::TankReleaseCover { control, .. } => control.accept(packet),
            Self::TaxiLandingLights { control, .. } => control.accept(packet),
            Self::TestMode { control, .. } => control.accept(packet),
            Self::Throttle { control, .. } => control.accept(packet),
            Self::Tils { control, .. } => control.accept(packet),
            Self::TilsChannelLayer { control, .. } => control.accept(packet),
            Self::TilsChannelSelect { control, .. } => control.accept(packet),
            Self::Tippvaxel { control, .. } => control.accept(packet),
            Self::TriggerSafetyBracket { control, .. } => control.accept(packet),
            Self::VerticalAcceleration { control, .. } => control.accept(packet),
            Self::Vstall { control, .. } => control.accept(packet),
            Self::WarningPanelTest { control, .. } => control.accept(packet),
            Self::WeaponInterval { control, .. } => control.accept(packet),
            Self::WeaponRelMode { control, .. } => control.accept(packet),
            Self::WeaponRelease { control, .. } => control.accept(packet),
            Self::WeaponReleaseCover { control, .. } => control.accept(packet),
            Self::WeaponSelect { control, .. } => control.accept(packet),
            Self::XTankBra { control, .. } => control.accept(packet),
            Self::YawTrimCover { control, .. } => control.accept(packet),
        }
    }

    fn commit(&mut self) {
        match self {
            Self::AdiAoaIndicator { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiHeading { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiHorizontalIls { control, callback } => {
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
            Self::AdiSlipball { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiVerticalIls { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiVerticalVelocity { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Afk15DegMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AfkFel { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::AfkLever { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AfkMode3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AirspeedMS { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AirspeedValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Ajs37DestIndicatorData1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Ajs37DestIndicatorData2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Ajs37NavIndicatorData1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Ajs37NavIndicatorData2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Ajs37NavIndicatorData3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Ajs37NavIndicatorData4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Ajs37NavIndicatorData5 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Ajs37NavIndicatorData6 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltSet { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Altimeter10000m { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Altimeter1000m { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltimeterBaro1000Hpa { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltimeterBaro100Hpa { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltimeterBaro10Hpa { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltimeterBaro1Hpa { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltitudeHold { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltitudeValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AntiCollisionLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AntiJamMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AttLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::AttitudeHold { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AutoYawTrim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BackAdiCage { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BackupAltSetting { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BackupAltimeter10000m { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BackupAltimeter1000m { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BackupGenerator { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BackupIndicatedAirspeed { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BackupPitch { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BackupRoll { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Bra24 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::BraUppf { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Brand1 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Brand2 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::BrandGts { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::CabinAirValve { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CanopyJettison { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CanopyOpenClose { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CbAutopilot { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CbCiSi { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CbEjectionSystem { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CbEngine { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CbHighAlphaWarn { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CbTrimSystem { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CiCommandHeadingValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CiCommandedHeading { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CiHeading { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CiHeadingValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Ck { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Ck37RensaClear { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockSet { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CountermeasureChaffFlaresSelector { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CountermeasureFastRelease { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CountermeasureModeSelector { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CountermeasureReleaseMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CountermeasureStreakModeSelector { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DataCartridge { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DataInOut { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DatapanelKey0 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DatapanelKey1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DatapanelKey2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DatapanelKey3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DatapanelKey4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DatapanelKey5 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DatapanelKey6 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DatapanelKey7 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DatapanelKey8 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DatapanelKey9 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DatapanelSelector { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DeIce { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DistanceIndicator { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DmeSelector { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DopplerLandSeaMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Drysuit { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EjRev { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::EjectionSeatArm { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Elfel { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::EmergencyLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EmergencyPitchTrim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EmergencyRollTrim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EmergencyYawTrim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EngineDeice { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EngineRpm10 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EngineRpm100 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EngineRpmValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EngineTemp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EngineTempValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtFormationLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtNavLightsTail { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtNavLightsWing { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtPositionLights { control, callback } => {
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
            Self::ExtStrobeLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FacklSl { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FlightRecorder { control, callback } => {
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
            Self::ForvForb { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Fr22Base { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22ChannelAg { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22ChannelB { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22ChannelC2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22ChannelCf { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22ChannelDe { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22ChannelH { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22Flight0 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22Flight1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22Flight2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22Flight3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22Flight4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22Flight5 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22Flight6 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22Flight7 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22Flight8 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22Flight9 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22GroundCom { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22Group { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22InnerLeftKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22InnerRightKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22Minus { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22Mode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22OuterLeftKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22OuterRightKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22SetModulation { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22Special1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22Special2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22Special3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fr22Vol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelLevel { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelLevelValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelNeeded { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelNeededValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GearHandle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Generator { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HallFunk { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::HighPresFuelValve { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HojdCisi { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HojdLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Hstall { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::HudBright { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HudGlassPosition { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HuvOStol { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::HydraTa1 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::HydraTa2 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::IffChannel { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IffCode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IffPower { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IffTransponderPower { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IgnitionSystem { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IndicatorSystemTest { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::InstrumentLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::JammerBandSelector { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::JammerModeSelector { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Kabinhojd { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::KbHKaSl { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::KbVSlut { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Landstall { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LowPresFuelValve { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Luftbroms { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::MachMeterFirstDecimal { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MachMeterInteger { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MachMeterSecondDecimal { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MagCorrection { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MagDecCover { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MagneticHeading { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MainElectricPower { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ManAfterburnFuelReg { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ManBgReg { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ManualFuelReg { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MasterCautionReset { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MasterModeSelect { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MasterVol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MaxGReset { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MissileSelectButton { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Motverk { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::NavSelectBtnB1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NavSelectBtnB2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NavSelectBtnB3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NavSelectBtnB4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NavSelectBtnB5 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NavSelectBtnB6 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NavSelectBtnB7 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NavSelectBtnB8 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NavSelectBtnB9 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NavSelectBtnBx { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NavSelectBtnLMal { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NavSelectBtnLs { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NavSyst { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::NavigationLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Nosstall { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Oljetryck { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::OxygenLever { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PanelLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ParkingBrake { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Pedals { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PitchGearMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PositionLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PositionLightsBrightness { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarAltimeterPower { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarBright { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarGain { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarIffId { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarIgnitionCoils { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarMaintenanceTest { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarPulseNormalShort { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarRecceOnOff { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarWarnSelect { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RbBkRelMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RensaButtonCover { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Reserveff { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Restart { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Reversal { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RhmFel { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RollCenter { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RollVaxel { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::SlavSi { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Spak { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SpakError { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::SpakLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::StartSystem { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Startsyst { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::StickPitch { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StickRoll { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Syrgas { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Tandsyst { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::TankPump { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::TankRelease { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TankReleaseCover { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TaxiLandingLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TestMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Throttle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Tils { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::TilsChannelLayer { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TilsChannelSelect { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Tippvaxel { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::TriggerSafetyBracket { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VerticalAcceleration { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Vstall { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::WarningPanelTest { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::WeaponInterval { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::WeaponRelMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::WeaponRelease { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::WeaponReleaseCover { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::WeaponSelect { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::XTankBra { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::YawTrimCover { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
        }
    }
}

pub struct Ajs37Builder<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    _packet_capacity: PhantomData<ProtocolParser<MAX_PACKET_DATA>>,
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    Ajs37Builder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
{
    pub fn new() -> Self {
        Self {
            controls: Vec::new(),
            _packet_capacity: PhantomData,
        }
    }

    pub fn register_adi_aoa_indicator<F>(
        &mut self,
        control: AdiAoaIndicator,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiAoaIndicator { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_heading<F>(
        &mut self,
        control: AdiHeading,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiHeading { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_horizontal_ils<F>(
        &mut self,
        control: AdiHorizontalIls,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiHorizontalIls { control, callback })
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

    pub fn register_adi_roll<F>(
        &mut self,
        control: AdiRoll,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiRoll { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_slipball<F>(
        &mut self,
        control: AdiSlipball,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiSlipball { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_vertical_ils<F>(
        &mut self,
        control: AdiVerticalIls,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiVerticalIls { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_vertical_velocity<F>(
        &mut self,
        control: AdiVerticalVelocity,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiVerticalVelocity { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_afk15_deg_mode<F>(
        &mut self,
        control: Afk15DegMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Afk15DegMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_afk_fel<F>(
        &mut self,
        control: AfkFel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AfkFel { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_afk_lever<F>(
        &mut self,
        control: AfkLever,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AfkLever { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_afk_mode3<F>(
        &mut self,
        control: AfkMode3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AfkMode3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_airspeed_m_s<F>(
        &mut self,
        control: AirspeedMS,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AirspeedMS { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_airspeed_value<F>(
        &mut self,
        control: AirspeedValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AirspeedValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ajs37_dest_indicator_data1<F>(
        &mut self,
        control: Ajs37DestIndicatorData1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ajs37DestIndicatorData1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ajs37_dest_indicator_data2<F>(
        &mut self,
        control: Ajs37DestIndicatorData2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ajs37DestIndicatorData2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ajs37_nav_indicator_data1<F>(
        &mut self,
        control: Ajs37NavIndicatorData1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ajs37NavIndicatorData1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ajs37_nav_indicator_data2<F>(
        &mut self,
        control: Ajs37NavIndicatorData2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ajs37NavIndicatorData2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ajs37_nav_indicator_data3<F>(
        &mut self,
        control: Ajs37NavIndicatorData3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ajs37NavIndicatorData3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ajs37_nav_indicator_data4<F>(
        &mut self,
        control: Ajs37NavIndicatorData4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ajs37NavIndicatorData4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ajs37_nav_indicator_data5<F>(
        &mut self,
        control: Ajs37NavIndicatorData5,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ajs37NavIndicatorData5 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ajs37_nav_indicator_data6<F>(
        &mut self,
        control: Ajs37NavIndicatorData6,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ajs37NavIndicatorData6 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_set<F>(
        &mut self,
        control: AltSet,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltSet { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter10000m<F>(
        &mut self,
        control: Altimeter10000m,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Altimeter10000m { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter1000m<F>(
        &mut self,
        control: Altimeter1000m,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Altimeter1000m { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter_baro1000_hpa<F>(
        &mut self,
        control: AltimeterBaro1000Hpa,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltimeterBaro1000Hpa { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter_baro100_hpa<F>(
        &mut self,
        control: AltimeterBaro100Hpa,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltimeterBaro100Hpa { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter_baro10_hpa<F>(
        &mut self,
        control: AltimeterBaro10Hpa,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltimeterBaro10Hpa { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter_baro1_hpa<F>(
        &mut self,
        control: AltimeterBaro1Hpa,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltimeterBaro1Hpa { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altitude_hold<F>(
        &mut self,
        control: AltitudeHold,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltitudeHold { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altitude_value<F>(
        &mut self,
        control: AltitudeValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltitudeValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_anti_collision_lights<F>(
        &mut self,
        control: AntiCollisionLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AntiCollisionLights { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_anti_jam_mode<F>(
        &mut self,
        control: AntiJamMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AntiJamMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_att_lamp<F>(
        &mut self,
        control: AttLamp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AttLamp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_attitude_hold<F>(
        &mut self,
        control: AttitudeHold,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AttitudeHold { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_auto_yaw_trim<F>(
        &mut self,
        control: AutoYawTrim,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AutoYawTrim { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_back_adi_cage<F>(
        &mut self,
        control: BackAdiCage,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BackAdiCage { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_backup_alt_setting<F>(
        &mut self,
        control: BackupAltSetting,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BackupAltSetting { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_backup_altimeter10000m<F>(
        &mut self,
        control: BackupAltimeter10000m,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BackupAltimeter10000m { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_backup_altimeter1000m<F>(
        &mut self,
        control: BackupAltimeter1000m,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BackupAltimeter1000m { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_backup_generator<F>(
        &mut self,
        control: BackupGenerator,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BackupGenerator { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_backup_indicated_airspeed<F>(
        &mut self,
        control: BackupIndicatedAirspeed,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BackupIndicatedAirspeed { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_backup_pitch<F>(
        &mut self,
        control: BackupPitch,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BackupPitch { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_backup_roll<F>(
        &mut self,
        control: BackupRoll,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BackupRoll { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bra24<F>(
        &mut self,
        control: Bra24,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Bra24 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bra_uppf<F>(
        &mut self,
        control: BraUppf,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::BraUppf { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brand1<F>(
        &mut self,
        control: Brand1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Brand1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brand2<F>(
        &mut self,
        control: Brand2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Brand2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brand_gts<F>(
        &mut self,
        control: BrandGts,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::BrandGts { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cabin_air_valve<F>(
        &mut self,
        control: CabinAirValve,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CabinAirValve { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_jettison<F>(
        &mut self,
        control: CanopyJettison,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyJettison { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_open_close<F>(
        &mut self,
        control: CanopyOpenClose,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyOpenClose { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_autopilot<F>(
        &mut self,
        control: CbAutopilot,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbAutopilot { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_ci_si<F>(
        &mut self,
        control: CbCiSi,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbCiSi { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_ejection_system<F>(
        &mut self,
        control: CbEjectionSystem,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbEjectionSystem { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_engine<F>(
        &mut self,
        control: CbEngine,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbEngine { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_high_alpha_warn<F>(
        &mut self,
        control: CbHighAlphaWarn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbHighAlphaWarn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_trim_system<F>(
        &mut self,
        control: CbTrimSystem,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbTrimSystem { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_command_heading_value<F>(
        &mut self,
        control: CiCommandHeadingValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiCommandHeadingValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_commanded_heading<F>(
        &mut self,
        control: CiCommandedHeading,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiCommandedHeading { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_heading<F>(
        &mut self,
        control: CiHeading,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiHeading { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_heading_value<F>(
        &mut self,
        control: CiHeadingValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiHeadingValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ck<F>(
        &mut self,
        control: Ck,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ck { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ck37_rensa_clear<F>(
        &mut self,
        control: Ck37RensaClear,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ck37RensaClear { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_set<F>(
        &mut self,
        control: ClockSet,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockSet { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_countermeasure_chaff_flares_selector<F>(
        &mut self,
        control: CountermeasureChaffFlaresSelector,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CountermeasureChaffFlaresSelector { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_countermeasure_fast_release<F>(
        &mut self,
        control: CountermeasureFastRelease,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CountermeasureFastRelease { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_countermeasure_mode_selector<F>(
        &mut self,
        control: CountermeasureModeSelector,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CountermeasureModeSelector { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_countermeasure_release_mode<F>(
        &mut self,
        control: CountermeasureReleaseMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CountermeasureReleaseMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_countermeasure_streak_mode_selector<F>(
        &mut self,
        control: CountermeasureStreakModeSelector,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CountermeasureStreakModeSelector { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_data_cartridge<F>(
        &mut self,
        control: DataCartridge,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DataCartridge { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_data_in_out<F>(
        &mut self,
        control: DataInOut,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DataInOut { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_key0<F>(
        &mut self,
        control: DatapanelKey0,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelKey0 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_key1<F>(
        &mut self,
        control: DatapanelKey1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelKey1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_key2<F>(
        &mut self,
        control: DatapanelKey2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelKey2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_key3<F>(
        &mut self,
        control: DatapanelKey3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelKey3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_key4<F>(
        &mut self,
        control: DatapanelKey4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelKey4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_key5<F>(
        &mut self,
        control: DatapanelKey5,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelKey5 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_key6<F>(
        &mut self,
        control: DatapanelKey6,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelKey6 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_key7<F>(
        &mut self,
        control: DatapanelKey7,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelKey7 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_key8<F>(
        &mut self,
        control: DatapanelKey8,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelKey8 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_key9<F>(
        &mut self,
        control: DatapanelKey9,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelKey9 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_selector<F>(
        &mut self,
        control: DatapanelSelector,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelSelector { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_de_ice<F>(
        &mut self,
        control: DeIce,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DeIce { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_distance_indicator<F>(
        &mut self,
        control: DistanceIndicator,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DistanceIndicator { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dme_selector<F>(
        &mut self,
        control: DmeSelector,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DmeSelector { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_doppler_land_sea_mode<F>(
        &mut self,
        control: DopplerLandSeaMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DopplerLandSeaMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_drysuit<F>(
        &mut self,
        control: Drysuit,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Drysuit { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ej_rev<F>(
        &mut self,
        control: EjRev,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::EjRev { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ejection_seat_arm<F>(
        &mut self,
        control: EjectionSeatArm,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EjectionSeatArm { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_elfel<F>(
        &mut self,
        control: Elfel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Elfel { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emergency_lights<F>(
        &mut self,
        control: EmergencyLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmergencyLights { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emergency_pitch_trim<F>(
        &mut self,
        control: EmergencyPitchTrim,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmergencyPitchTrim { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emergency_roll_trim<F>(
        &mut self,
        control: EmergencyRollTrim,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmergencyRollTrim { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emergency_yaw_trim<F>(
        &mut self,
        control: EmergencyYawTrim,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmergencyYawTrim { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_deice<F>(
        &mut self,
        control: EngineDeice,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineDeice { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_rpm10<F>(
        &mut self,
        control: EngineRpm10,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineRpm10 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_rpm100<F>(
        &mut self,
        control: EngineRpm100,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineRpm100 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_rpm_value<F>(
        &mut self,
        control: EngineRpmValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineRpmValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_temp<F>(
        &mut self,
        control: EngineTemp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineTemp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_temp_value<F>(
        &mut self,
        control: EngineTempValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineTempValue { control, callback })
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

    pub fn register_ext_nav_lights_tail<F>(
        &mut self,
        control: ExtNavLightsTail,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtNavLightsTail { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_nav_lights_wing<F>(
        &mut self,
        control: ExtNavLightsWing,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtNavLightsWing { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_position_lights<F>(
        &mut self,
        control: ExtPositionLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtPositionLights { control, callback })
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

    pub fn register_ext_strobe_lights<F>(
        &mut self,
        control: ExtStrobeLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtStrobeLights { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fackl_sl<F>(
        &mut self,
        control: FacklSl,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FacklSl { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flight_recorder<F>(
        &mut self,
        control: FlightRecorder,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlightRecorder { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flood_lights<F>(
        &mut self,
        control: FloodLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FloodLights { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_formation_lights<F>(
        &mut self,
        control: FormationLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FormationLights { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_forv_forb<F>(
        &mut self,
        control: ForvForb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ForvForb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_base<F>(
        &mut self,
        control: Fr22Base,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Base { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_channel_ag<F>(
        &mut self,
        control: Fr22ChannelAg,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22ChannelAg { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_channel_b<F>(
        &mut self,
        control: Fr22ChannelB,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22ChannelB { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_channel_c2<F>(
        &mut self,
        control: Fr22ChannelC2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22ChannelC2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_channel_cf<F>(
        &mut self,
        control: Fr22ChannelCf,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22ChannelCf { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_channel_de<F>(
        &mut self,
        control: Fr22ChannelDe,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22ChannelDe { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_channel_h<F>(
        &mut self,
        control: Fr22ChannelH,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22ChannelH { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_flight0<F>(
        &mut self,
        control: Fr22Flight0,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Flight0 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_flight1<F>(
        &mut self,
        control: Fr22Flight1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Flight1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_flight2<F>(
        &mut self,
        control: Fr22Flight2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Flight2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_flight3<F>(
        &mut self,
        control: Fr22Flight3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Flight3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_flight4<F>(
        &mut self,
        control: Fr22Flight4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Flight4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_flight5<F>(
        &mut self,
        control: Fr22Flight5,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Flight5 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_flight6<F>(
        &mut self,
        control: Fr22Flight6,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Flight6 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_flight7<F>(
        &mut self,
        control: Fr22Flight7,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Flight7 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_flight8<F>(
        &mut self,
        control: Fr22Flight8,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Flight8 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_flight9<F>(
        &mut self,
        control: Fr22Flight9,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Flight9 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_ground_com<F>(
        &mut self,
        control: Fr22GroundCom,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22GroundCom { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_group<F>(
        &mut self,
        control: Fr22Group,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Group { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_inner_left_knob<F>(
        &mut self,
        control: Fr22InnerLeftKnob,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22InnerLeftKnob { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_inner_right_knob<F>(
        &mut self,
        control: Fr22InnerRightKnob,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22InnerRightKnob { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_minus<F>(
        &mut self,
        control: Fr22Minus,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Minus { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_mode<F>(
        &mut self,
        control: Fr22Mode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Mode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_outer_left_knob<F>(
        &mut self,
        control: Fr22OuterLeftKnob,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22OuterLeftKnob { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_outer_right_knob<F>(
        &mut self,
        control: Fr22OuterRightKnob,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22OuterRightKnob { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_set_modulation<F>(
        &mut self,
        control: Fr22SetModulation,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22SetModulation { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_special1<F>(
        &mut self,
        control: Fr22Special1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Special1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_special2<F>(
        &mut self,
        control: Fr22Special2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Special2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_special3<F>(
        &mut self,
        control: Fr22Special3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Special3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_vol<F>(
        &mut self,
        control: Fr22Vol,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Vol { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_level<F>(
        &mut self,
        control: FuelLevel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelLevel { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_level_value<F>(
        &mut self,
        control: FuelLevelValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelLevelValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_needed<F>(
        &mut self,
        control: FuelNeeded,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelNeeded { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_needed_value<F>(
        &mut self,
        control: FuelNeededValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelNeededValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gear_handle<F>(
        &mut self,
        control: GearHandle,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GearHandle { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_generator<F>(
        &mut self,
        control: Generator,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Generator { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hall_funk<F>(
        &mut self,
        control: HallFunk,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::HallFunk { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_high_pres_fuel_valve<F>(
        &mut self,
        control: HighPresFuelValve,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HighPresFuelValve { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hojd_cisi<F>(
        &mut self,
        control: HojdCisi,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HojdCisi { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hojd_lamp<F>(
        &mut self,
        control: HojdLamp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::HojdLamp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hstall<F>(
        &mut self,
        control: Hstall,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Hstall { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_bright<F>(
        &mut self,
        control: HudBright,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudBright { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_glass_position<F>(
        &mut self,
        control: HudGlassPosition,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudGlassPosition { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_huv_o_stol<F>(
        &mut self,
        control: HuvOStol,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::HuvOStol { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hydra_ta1<F>(
        &mut self,
        control: HydraTa1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydraTa1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hydra_ta2<F>(
        &mut self,
        control: HydraTa2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydraTa2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_channel<F>(
        &mut self,
        control: IffChannel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffChannel { control, callback })
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

    pub fn register_iff_power<F>(
        &mut self,
        control: IffPower,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffPower { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_transponder_power<F>(
        &mut self,
        control: IffTransponderPower,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffTransponderPower { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ignition_system<F>(
        &mut self,
        control: IgnitionSystem,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IgnitionSystem { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_indicator_system_test<F>(
        &mut self,
        control: IndicatorSystemTest,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IndicatorSystemTest { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_instrument_lights<F>(
        &mut self,
        control: InstrumentLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::InstrumentLights { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jammer_band_selector<F>(
        &mut self,
        control: JammerBandSelector,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::JammerBandSelector { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jammer_mode_selector<F>(
        &mut self,
        control: JammerModeSelector,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::JammerModeSelector { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_kabinhojd<F>(
        &mut self,
        control: Kabinhojd,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Kabinhojd { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_kb_h_ka_sl<F>(
        &mut self,
        control: KbHKaSl,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::KbHKaSl { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_kb_v_slut<F>(
        &mut self,
        control: KbVSlut,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::KbVSlut { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_landstall<F>(
        &mut self,
        control: Landstall,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Landstall { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_low_pres_fuel_valve<F>(
        &mut self,
        control: LowPresFuelValve,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LowPresFuelValve { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_luftbroms<F>(
        &mut self,
        control: Luftbroms,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Luftbroms { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mach_meter_first_decimal<F>(
        &mut self,
        control: MachMeterFirstDecimal,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MachMeterFirstDecimal { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mach_meter_integer<F>(
        &mut self,
        control: MachMeterInteger,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MachMeterInteger { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mach_meter_second_decimal<F>(
        &mut self,
        control: MachMeterSecondDecimal,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MachMeterSecondDecimal { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mag_correction<F>(
        &mut self,
        control: MagCorrection,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MagCorrection { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mag_dec_cover<F>(
        &mut self,
        control: MagDecCover,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MagDecCover { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_magnetic_heading<F>(
        &mut self,
        control: MagneticHeading,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MagneticHeading { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_main_electric_power<F>(
        &mut self,
        control: MainElectricPower,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MainElectricPower { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_man_afterburn_fuel_reg<F>(
        &mut self,
        control: ManAfterburnFuelReg,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ManAfterburnFuelReg { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_man_bg_reg<F>(
        &mut self,
        control: ManBgReg,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ManBgReg { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_manual_fuel_reg<F>(
        &mut self,
        control: ManualFuelReg,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ManualFuelReg { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_caution_reset<F>(
        &mut self,
        control: MasterCautionReset,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterCautionReset { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_mode_select<F>(
        &mut self,
        control: MasterModeSelect,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterModeSelect { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_vol<F>(
        &mut self,
        control: MasterVol,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterVol { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_max_g_reset<F>(
        &mut self,
        control: MaxGReset,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MaxGReset { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_missile_select_button<F>(
        &mut self,
        control: MissileSelectButton,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MissileSelectButton { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_motverk<F>(
        &mut self,
        control: Motverk,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Motverk { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_b1<F>(
        &mut self,
        control: NavSelectBtnB1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnB1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_b2<F>(
        &mut self,
        control: NavSelectBtnB2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnB2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_b3<F>(
        &mut self,
        control: NavSelectBtnB3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnB3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_b4<F>(
        &mut self,
        control: NavSelectBtnB4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnB4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_b5<F>(
        &mut self,
        control: NavSelectBtnB5,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnB5 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_b6<F>(
        &mut self,
        control: NavSelectBtnB6,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnB6 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_b7<F>(
        &mut self,
        control: NavSelectBtnB7,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnB7 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_b8<F>(
        &mut self,
        control: NavSelectBtnB8,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnB8 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_b9<F>(
        &mut self,
        control: NavSelectBtnB9,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnB9 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_bx<F>(
        &mut self,
        control: NavSelectBtnBx,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnBx { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_l_mal<F>(
        &mut self,
        control: NavSelectBtnLMal,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnLMal { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_ls<F>(
        &mut self,
        control: NavSelectBtnLs,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnLs { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_syst<F>(
        &mut self,
        control: NavSyst,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSyst { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_navigation_lights<F>(
        &mut self,
        control: NavigationLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavigationLights { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nosstall<F>(
        &mut self,
        control: Nosstall,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Nosstall { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oljetryck<F>(
        &mut self,
        control: Oljetryck,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Oljetryck { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxygen_lever<F>(
        &mut self,
        control: OxygenLever,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxygenLever { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_panel_lights<F>(
        &mut self,
        control: PanelLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PanelLights { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_parking_brake<F>(
        &mut self,
        control: ParkingBrake,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ParkingBrake { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pedals<F>(
        &mut self,
        control: Pedals,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Pedals { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pitch_gear_mode<F>(
        &mut self,
        control: PitchGearMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PitchGearMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_position_lights<F>(
        &mut self,
        control: PositionLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PositionLights { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_position_lights_brightness<F>(
        &mut self,
        control: PositionLightsBrightness,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PositionLightsBrightness { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_altimeter_power<F>(
        &mut self,
        control: RadarAltimeterPower,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarAltimeterPower { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_bright<F>(
        &mut self,
        control: RadarBright,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarBright { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_gain<F>(
        &mut self,
        control: RadarGain,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarGain { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_iff_id<F>(
        &mut self,
        control: RadarIffId,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarIffId { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_ignition_coils<F>(
        &mut self,
        control: RadarIgnitionCoils,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarIgnitionCoils { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_maintenance_test<F>(
        &mut self,
        control: RadarMaintenanceTest,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarMaintenanceTest { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_pulse_normal_short<F>(
        &mut self,
        control: RadarPulseNormalShort,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarPulseNormalShort { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_recce_on_off<F>(
        &mut self,
        control: RadarRecceOnOff,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarRecceOnOff { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_warn_select<F>(
        &mut self,
        control: RadarWarnSelect,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarWarnSelect { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rb_bk_rel_mode<F>(
        &mut self,
        control: RbBkRelMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RbBkRelMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rensa_button_cover<F>(
        &mut self,
        control: RensaButtonCover,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RensaButtonCover { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_reserveff<F>(
        &mut self,
        control: Reserveff,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Reserveff { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_restart<F>(
        &mut self,
        control: Restart,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Restart { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_reversal<F>(
        &mut self,
        control: Reversal,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Reversal { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rhm_fel<F>(
        &mut self,
        control: RhmFel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RhmFel { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_roll_center<F>(
        &mut self,
        control: RollCenter,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RollCenter { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_roll_vaxel<F>(
        &mut self,
        control: RollVaxel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RollVaxel { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_slav_si<F>(
        &mut self,
        control: SlavSi,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SlavSi { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_spak<F>(
        &mut self,
        control: Spak,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Spak { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_spak_error<F>(
        &mut self,
        control: SpakError,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::SpakError { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_spak_lamp<F>(
        &mut self,
        control: SpakLamp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::SpakLamp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_start_system<F>(
        &mut self,
        control: StartSystem,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StartSystem { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_startsyst<F>(
        &mut self,
        control: Startsyst,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Startsyst { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stick_pitch<F>(
        &mut self,
        control: StickPitch,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StickPitch { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stick_roll<F>(
        &mut self,
        control: StickRoll,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StickRoll { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_syrgas<F>(
        &mut self,
        control: Syrgas,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Syrgas { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tandsyst<F>(
        &mut self,
        control: Tandsyst,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Tandsyst { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tank_pump<F>(
        &mut self,
        control: TankPump,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::TankPump { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tank_release<F>(
        &mut self,
        control: TankRelease,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TankRelease { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tank_release_cover<F>(
        &mut self,
        control: TankReleaseCover,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TankReleaseCover { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_taxi_landing_lights<F>(
        &mut self,
        control: TaxiLandingLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TaxiLandingLights { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_test_mode<F>(
        &mut self,
        control: TestMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TestMode { control, callback })
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

    pub fn register_tils<F>(
        &mut self,
        control: Tils,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Tils { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tils_channel_layer<F>(
        &mut self,
        control: TilsChannelLayer,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TilsChannelLayer { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tils_channel_select<F>(
        &mut self,
        control: TilsChannelSelect,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TilsChannelSelect { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tippvaxel<F>(
        &mut self,
        control: Tippvaxel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Tippvaxel { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_trigger_safety_bracket<F>(
        &mut self,
        control: TriggerSafetyBracket,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TriggerSafetyBracket { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vertical_acceleration<F>(
        &mut self,
        control: VerticalAcceleration,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VerticalAcceleration { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vstall<F>(
        &mut self,
        control: Vstall,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vstall { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_warning_panel_test<F>(
        &mut self,
        control: WarningPanelTest,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WarningPanelTest { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon_interval<F>(
        &mut self,
        control: WeaponInterval,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WeaponInterval { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon_rel_mode<F>(
        &mut self,
        control: WeaponRelMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WeaponRelMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon_release<F>(
        &mut self,
        control: WeaponRelease,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WeaponRelease { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon_release_cover<F>(
        &mut self,
        control: WeaponReleaseCover,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WeaponReleaseCover { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon_select<F>(
        &mut self,
        control: WeaponSelect,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WeaponSelect { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_x_tank_bra<F>(
        &mut self,
        control: XTankBra,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::XTankBra { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_yaw_trim_cover<F>(
        &mut self,
        control: YawTrimCover,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::YawTrimCover { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn build(self) -> Ajs37<'a, MAX_CONTROLS, MAX_PACKET_DATA> {
        Ajs37 {
            parser: ProtocolParser::new(),
            controls: self.controls,
            saw_frame: false,
        }
    }
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> Default
    for Ajs37Builder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
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

pub struct Ajs37<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    parser: ProtocolParser<MAX_PACKET_DATA>,
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    saw_frame: bool,
}

impl<const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    Ajs37<'_, MAX_CONTROLS, MAX_PACKET_DATA>
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
