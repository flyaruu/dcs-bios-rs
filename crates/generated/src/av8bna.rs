//! Generated from model-definitions/AV8BNA.json.
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

/// AAA Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct AaaLight(Led);
impl AaaLight {
    pub const fn new() -> Self {
        Self(Led::new(30848, 256, 8))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for AaaLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AaaLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ACNIP Mode Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AcnipMode(Selector);
impl AcnipMode {
    pub const fn new() -> Self {
        Self(Selector::new(30806, 256, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AcnipMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AcnipMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ACNIP Radio Selector Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AcnipRadioSel(Selector);
impl AcnipRadioSel {
    pub const fn new() -> Self {
        Self(Selector::new(30806, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AcnipRadioSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AcnipRadioSel {
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
        Self(AnalogGauge::new(30988, 65535, 0))
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

/// ADI OFF Flag
/// Output: gauge position
pub struct AdiFlag(AnalogGauge);
impl AdiFlag {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30990, 65535, 0))
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

/// ADI Pitch
/// Output: gauge position
pub struct AdiPitch(AnalogGauge);
impl AdiPitch {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30986, 65535, 0))
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

/// AFC Switch ON/OFF/RESET
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct AfcSw(Selector);
impl AfcSw {
    pub const fn new() -> Self {
        Self(Selector::new(30786, 24576, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AfcSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AfcSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// AFC Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct AfcWlight(Led);
impl AfcWlight {
    pub const fn new() -> Self {
        Self(Led::new(30852, 16384, 14))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for AfcWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AfcWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// AFT BAY Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct AftBayWlight(Led);
impl AftBayWlight {
    pub const fn new() -> Self {
        Self(Led::new(30854, 128, 7))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for AftBayWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AftBayWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// A/G Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct AgLight(Led);
impl AgLight {
    pub const fn new() -> Self {
        Self(Led::new(30848, 2048, 11))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for AgLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AgLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// AG Master Mode Selector
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AgSel(Selector);
impl AgSel {
    pub const fn new() -> Self {
        Self(Selector::new(30720, 4, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AgSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AgSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// AI Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct AiLight(Led);
impl AiLight {
    pub const fn new() -> Self {
        Self(Led::new(30848, 128, 7))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for AiLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AiLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Aileron Needle
/// Output: gauge position
pub struct AileronNeedle(AnalogGauge);
impl AileronNeedle {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31020, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AileronNeedle {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AileronNeedle {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altimeter 10000 FT Counter
/// Output: gauge position
pub struct Alt10000ftCnt(AnalogGauge);
impl Alt10000ftCnt {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30998, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Alt10000ftCnt {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Alt10000ftCnt {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altimeter 1000 FT Counter
/// Output: gauge position
pub struct Alt1000ftCnt(AnalogGauge);
impl Alt1000ftCnt {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31000, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Alt1000ftCnt {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Alt1000ftCnt {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altimeter 100 FT Counter
/// Output: gauge position
pub struct Alt100ftCnt(AnalogGauge);
impl Alt100ftCnt {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31002, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Alt100ftCnt {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Alt100ftCnt {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Alt Hold Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AltHoldSw(Selector);
impl AltHoldSw {
    pub const fn new() -> Self {
        Self(Selector::new(30786, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltHoldSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltHoldSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altimeter Pointer
/// Output: gauge position
pub struct AltPointer(AnalogGauge);
impl AltPointer {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30996, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltPointer {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltPointer {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altimeter Pressure Digit 1
/// Output: gauge position
pub struct AltPressure1(AnalogGauge);
impl AltPressure1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31004, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltPressure1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltPressure1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altimeter Pressure Digit 2
/// Output: gauge position
pub struct AltPressure2(AnalogGauge);
impl AltPressure2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31006, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltPressure2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltPressure2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altimeter Pressure Digit 3
/// Output: gauge position
pub struct AltPressure3(AnalogGauge);
impl AltPressure3 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31008, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltPressure3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltPressure3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altimeter Pressure Digit 4
/// Output: gauge position
pub struct AltPressure4(AnalogGauge);
impl AltPressure4 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31010, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltPressure4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltPressure4 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Annunciator Lights
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct AnnunciatorLights(LimitedDial);
impl AnnunciatorLights {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30820, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AnnunciatorLights {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AnnunciatorLights {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Anti-Collision Lights Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AntiCollLights(Selector);
impl AntiCollLights {
    pub const fn new() -> Self {
        Self(Selector::new(30796, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AntiCollLights {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AntiCollLights {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Anti-Skid Switch TEST/ON/NWS
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct AntiSkid(Selector);
impl AntiSkid {
    pub const fn new() -> Self {
        Self(Selector::new(30778, 12288, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AntiSkid {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AntiSkid {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Speed knt Pointer Short
/// Output: gauge position
pub struct AoaFlag(AnalogGauge);
impl AoaFlag {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31012, 65535, 0))
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

/// Speed knt Pointer Long
/// Output: gauge position
pub struct AoaPointer(AnalogGauge);
impl AoaPointer {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31014, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AoaPointer {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AoaPointer {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// APU Generator Switch OFF/ON/RESET
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct ApuGenSw(Selector);
impl ApuGenSw {
    pub const fn new() -> Self {
        Self(Selector::new(30800, 768, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ApuGenSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ApuGenSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// APU GEN Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct ApuGenWlight(Led);
impl ApuGenWlight {
    pub const fn new() -> Self {
        Self(Led::new(30852, 2, 1))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ApuGenWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ApuGenWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// APU Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct ApuWlight(Led);
impl ApuWlight {
    pub const fn new() -> Self {
        Self(Led::new(30854, 2, 1))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ApuWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ApuWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Armament Mode Control
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct ArmControl(Selector);
impl ArmControl {
    pub const fn new() -> Self {
        Self(Selector::new(30768, 3072, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ArmControl {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ArmControl {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// AUT FLP Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct AutflpWlight(Led);
impl AutflpWlight {
    pub const fn new() -> Self {
        Self(Led::new(30850, 8192, 13))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for AutflpWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AutflpWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ODU Option 1 Select (string)
/// Output: ODU Option 1 Select (string)
pub struct Av8bnaOdu1Select(Display<1>);
impl Av8bnaOdu1Select {
    pub const fn new() -> Self {
        Self(Display::<1>::new(31078, 1))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for Av8bnaOdu1Select {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Av8bnaOdu1Select {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ODU Option 1 Text (string)
/// Output: ODU Option 1 Text (string)
pub struct Av8bnaOdu1Text(Display<4>);
impl Av8bnaOdu1Text {
    pub const fn new() -> Self {
        Self(Display::<4>::new(31080, 4))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for Av8bnaOdu1Text {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Av8bnaOdu1Text {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ODU Option 2 Select (string)
/// Output: ODU Option 2 Select (string)
pub struct Av8bnaOdu2Select(Display<1>);
impl Av8bnaOdu2Select {
    pub const fn new() -> Self {
        Self(Display::<1>::new(31084, 1))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for Av8bnaOdu2Select {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Av8bnaOdu2Select {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ODU Option 2 Text (string)
/// Output: ODU Option 2 Text (string)
pub struct Av8bnaOdu2Text(Display<4>);
impl Av8bnaOdu2Text {
    pub const fn new() -> Self {
        Self(Display::<4>::new(31086, 4))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for Av8bnaOdu2Text {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Av8bnaOdu2Text {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ODU Option 3 Select (string)
/// Output: ODU Option 3 Select (string)
pub struct Av8bnaOdu3Select(Display<1>);
impl Av8bnaOdu3Select {
    pub const fn new() -> Self {
        Self(Display::<1>::new(31090, 1))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for Av8bnaOdu3Select {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Av8bnaOdu3Select {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ODU Option 3 Text (string)
/// Output: ODU Option 3 Text (string)
pub struct Av8bnaOdu3Text(Display<4>);
impl Av8bnaOdu3Text {
    pub const fn new() -> Self {
        Self(Display::<4>::new(31092, 4))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for Av8bnaOdu3Text {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Av8bnaOdu3Text {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ODU Option 4 Select (string)
/// Output: ODU Option 4 Select (string)
pub struct Av8bnaOdu4Select(Display<1>);
impl Av8bnaOdu4Select {
    pub const fn new() -> Self {
        Self(Display::<1>::new(31096, 1))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for Av8bnaOdu4Select {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Av8bnaOdu4Select {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ODU Option 4 Text (string)
/// Output: ODU Option 4 Text (string)
pub struct Av8bnaOdu4Text(Display<4>);
impl Av8bnaOdu4Text {
    pub const fn new() -> Self {
        Self(Display::<4>::new(31098, 4))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for Av8bnaOdu4Text {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Av8bnaOdu4Text {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ODU Option 5 Select (string)
/// Output: ODU Option 5 Select (string)
pub struct Av8bnaOdu5Select(Display<1>);
impl Av8bnaOdu5Select {
    pub const fn new() -> Self {
        Self(Display::<1>::new(31102, 1))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for Av8bnaOdu5Select {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Av8bnaOdu5Select {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ODU Option 5 Text (string)
/// Output: ODU Option 5 Text (string)
pub struct Av8bnaOdu5Text(Display<4>);
impl Av8bnaOdu5Text {
    pub const fn new() -> Self {
        Self(Display::<4>::new(31104, 4))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for Av8bnaOdu5Text {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Av8bnaOdu5Text {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// AV BIT Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct AvBitWlight(Led);
impl AvBitWlight {
    pub const fn new() -> Self {
        Self(Led::new(30854, 256, 8))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for AvBitWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AvBitWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Backup ADI Cage/Pitch Adjust Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct BakAdiCageKnob(LimitedDial);
impl BakAdiCageKnob {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30784, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BakAdiCageKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BakAdiCageKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Backup ADI Cage/Pitch Adjust Pull
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct BakAdiCagePull(Selector);
impl BakAdiCagePull {
    pub const fn new() -> Self {
        Self(Selector::new(30780, 1024, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BakAdiCagePull {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BakAdiCagePull {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Barometric Pressure Calibration
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct BaroPressure(AnalogDial);
impl BaroPressure {
    pub const fn new() -> Self {
        Self(AnalogDial::new(30782, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BaroPressure {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BaroPressure {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Battery Switch  ALERT/OFF/ON
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct BattSw(Selector);
impl BattSw {
    pub const fn new() -> Self {
        Self(Selector::new(30800, 24576, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BattSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BattSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// BINGO 1. Drum Digit
/// Output: gauge position
pub struct Bingo1(AnalogGauge);
impl Bingo1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30924, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Bingo1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Bingo1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// BINGO 2. Drum Digit
/// Output: gauge position
pub struct Bingo2(AnalogGauge);
impl Bingo2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30926, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Bingo2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Bingo2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// BINGO 3. Drum Digit
/// Output: gauge position
pub struct Bingo3(AnalogGauge);
impl Bingo3 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30928, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Bingo3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Bingo3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// BINGO 4. Drum Digit
/// Output: gauge position
pub struct Bingo4(AnalogGauge);
impl Bingo4 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30930, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Bingo4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Bingo4 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Bingo Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct BingoLight(Led);
impl BingoLight {
    pub const fn new() -> Self {
        Self(Led::new(30844, 256, 8))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for BingoLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BingoLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Bingo Fuel Set Knob
/// Input: rotate the knob left or right
/// Output: rotation of the knob
pub struct BingoSet(FixedStepDial);
impl BingoSet {
    pub const fn new() -> Self {
        Self(FixedStepDial::new(30760, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BingoSet {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BingoSet {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// V/UHF RSC Frequency Mode Switch AJ-M/AJ/MAR/PRST/MAN/243/121
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct BombsArmSw(Selector);
impl BombsArmSw {
    pub const fn new() -> Self {
        Self(Selector::new(30806, 112, 4))
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

/// Brake 1. Drum Digit
/// Output: gauge position
pub struct Brake1(AnalogGauge);
impl Brake1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30964, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Brake1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Brake1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Brake 2. Drum Digit
/// Output: gauge position
pub struct Brake2(AnalogGauge);
impl Brake2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30966, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Brake2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Brake2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Brake 3. Drum Digit
/// Output: gauge position
pub struct Brake3(AnalogGauge);
impl Brake3 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30968, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Brake3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Brake3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Brake Accummulator
/// Output: gauge position
pub struct BrakeAccu(AnalogGauge);
impl BrakeAccu {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31048, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BrakeAccu {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BrakeAccu {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// C AUT Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct CAut(Led);
impl CAut {
    pub const fn new() -> Self {
        Self(Led::new(30852, 32768, 15))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for CAut {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CAut {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Cabin Defog Switch NORM/DFOG/MAX
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct CabinDefog(Selector);
impl CabinDefog {
    pub const fn new() -> Self {
        Self(Selector::new(30808, 1536, 9))
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

/// Cabin Pressure Pointer
/// Output: gauge position
pub struct CabinPress(AnalogGauge);
impl CabinPress {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31024, 65535, 0))
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

/// Cabin Pressure Switch NORM/DUMP/RAM
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct CabinPressSwitch(Selector);
impl CabinPressSwitch {
    pub const fn new() -> Self {
        Self(Selector::new(30808, 24576, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CabinPressSwitch {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CabinPressSwitch {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Canopy Handle Left
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct CanopyHandL(Selector);
impl CanopyHandL {
    pub const fn new() -> Self {
        Self(Selector::new(30808, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CanopyHandL {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CanopyHandL {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Canopy Handle Right
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct CanopyHandR(Selector);
impl CanopyHandR {
    pub const fn new() -> Self {
        Self(Selector::new(30844, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CanopyHandR {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CanopyHandR {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Canopy Locking Lever
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct CanopyLock(Selector);
impl CanopyLock {
    pub const fn new() -> Self {
        Self(Selector::new(30844, 2, 1))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CanopyLock {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CanopyLock {
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
        Self(AnalogGauge::new(30846, 65535, 0))
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

/// CANOPY Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct CanopyWlight(Led);
impl CanopyWlight {
    pub const fn new() -> Self {
        Self(Led::new(30852, 512, 9))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for CanopyWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CanopyWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Clock Hours
/// Output: gauge position
pub struct ClockHh(AnalogGauge);
impl ClockHh {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31034, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ClockHh {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClockHh {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Clock Minutes
/// Output: gauge position
pub struct ClockMm(AnalogGauge);
impl ClockMm {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31036, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ClockMm {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClockMm {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Clock Seconds
/// Output: gauge position
pub struct ClockSs(AnalogGauge);
impl ClockSs {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31038, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ClockSs {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClockSs {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// CMBT Thrust Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Cmbt(Selector);
impl Cmbt {
    pub const fn new() -> Self {
        Self(Selector::new(30780, 2, 1))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Cmbt {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Cmbt {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// CMBT Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct CmbtLight(Led);
impl CmbtLight {
    pub const fn new() -> Self {
        Self(Led::new(30848, 8192, 13))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for CmbtLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CmbtLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// COMM1 ARC-210 Frequency (string)
/// Output: COMM1 ARC-210 Frequency (string)
pub struct Comm1StringFreq(Display<7>);
impl Comm1StringFreq {
    pub const fn new() -> Self {
        Self(Display::<7>::new(30744, 7))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for Comm1StringFreq {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Comm1StringFreq {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// COMM2 ARC-210 Frequency (string)
/// Output: COMM2 ARC-210 Frequency (string)
pub struct Comm2StringFreq(Display<7>);
impl Comm2StringFreq {
    pub const fn new() -> Self {
        Self(Display::<7>::new(30752, 7))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for Comm2StringFreq {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Comm2StringFreq {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Compass Light/Test Lights  COPM/OFF/LTS TEST
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct CompLight(Selector);
impl CompLight {
    pub const fn new() -> Self {
        Self(Selector::new(30808, 192, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CompLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CompLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Compassrose Bank
/// Output: gauge position
pub struct CompassBank(AnalogGauge);
impl CompassBank {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31032, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CompassBank {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CompassBank {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Compass Lightning Intensity
/// Output: gauge position
pub struct CompassLights(AnalogGauge);
impl CompassLights {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31046, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CompassLights {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CompassLights {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Compassrose N-S
/// Output: gauge position
pub struct CompassNs(AnalogGauge);
impl CompassNs {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31028, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CompassNs {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CompassNs {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Compassrose Up/Down
/// Output: gauge position
pub struct CompassUd(AnalogGauge);
impl CompassUd {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31030, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CompassUd {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CompassUd {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Console Lights
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct ConsoleLights(LimitedDial);
impl ConsoleLights {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30816, 65535, 0))
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

/// 15 Sek Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct Control15sekLight(Led);
impl Control15sekLight {
    pub const fn new() -> Self {
        Self(Led::new(30844, 64, 6))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Control15sekLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Control15sekLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// CS COOL Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct CsCoolWlight(Led);
impl CsCoolWlight {
    pub const fn new() -> Self {
        Self(Led::new(30852, 128, 7))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for CsCoolWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CsCoolWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// CW JAM Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct CwJamWlight(Led);
impl CwJamWlight {
    pub const fn new() -> Self {
        Self(Led::new(30856, 1, 0))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for CwJamWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CwJamWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// CW Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct CwLight(Led);
impl CwLight {
    pub const fn new() -> Self {
        Self(Led::new(30848, 64, 6))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for CwLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CwLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// CW NOGO Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct CwNogoWlight(Led);
impl CwNogoWlight {
    pub const fn new() -> Self {
        Self(Led::new(30854, 2048, 11))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for CwNogoWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CwNogoWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// DC Test Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct DcTestSw(Selector);
impl DcTestSw {
    pub const fn new() -> Self {
        Self(Selector::new(30800, 128, 7))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DcTestSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DcTestSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// DC Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct DcWlight(Led);
impl DcWlight {
    pub const fn new() -> Self {
        Self(Led::new(30852, 32, 5))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for DcWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DcWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Decoy Dispenser Control OFF/AUT/UP/DWN/RWR
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct DecoyControl(Selector);
impl DecoyControl {
    pub const fn new() -> Self {
        Self(Selector::new(30778, 7, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DecoyControl {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DecoyControl {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// DECS Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct DecsSw(Selector);
impl DecsSw {
    pub const fn new() -> Self {
        Self(Selector::new(30800, 64, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DecsSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DecsSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// DEP RES Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct DepResWlight(Led);
impl DepResWlight {
    pub const fn new() -> Self {
        Self(Led::new(30852, 16, 4))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for DepResWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DepResWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// DMT Toggle On/Off
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Dmt(Selector);
impl Dmt {
    pub const fn new() -> Self {
        Self(Selector::new(30780, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Dmt {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Dmt {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// DROOP Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct DroopWlight(Led);
impl DroopWlight {
    pub const fn new() -> Self {
        Self(Led::new(30854, 32, 5))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for DroopWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DroopWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Display Computer Mode Selector Switch PRIM/AUTO/ALTER
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct DualCpuMode(Selector);
impl DualCpuMode {
    pub const fn new() -> Self {
        Self(Selector::new(30780, 49152, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DualCpuMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DualCpuMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Duct 1. Drum Digit
/// Output: gauge position
pub struct Duct1(AnalogGauge);
impl Duct1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30858, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Duct1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Duct1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Duct 2. Drum Digit
/// Output: gauge position
pub struct Duct2(AnalogGauge);
impl Duct2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30860, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Duct2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Duct2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Duct 3. Drum Digit
/// Output: gauge position
pub struct Duct3(AnalogGauge);
impl Duct3 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30862, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Duct3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Duct3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Aft Equipment Bay ECS Switch RESET/ON/OFF
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct EcsAft(Selector);
impl EcsAft {
    pub const fn new() -> Self {
        Self(Selector::new(30808, 6144, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EcsAft {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EcsAft {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fwd Equipment Bay ECS Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EcsFwd(Selector);
impl EcsFwd {
    pub const fn new() -> Self {
        Self(Selector::new(30808, 256, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EcsFwd {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EcsFwd {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// EDP BIT Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EdpBit(Selector);
impl EdpBit {
    pub const fn new() -> Self {
        Self(Selector::new(30720, 8, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EdpBit {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EdpBit {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// EDP Brightness Control
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct EdpBright(LimitedDial);
impl EdpBright {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30722, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EdpBright {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EdpBright {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// EFC Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct EfcLight(Led);
impl EfcLight {
    pub const fn new() -> Self {
        Self(Led::new(30848, 8, 3))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for EfcLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EfcLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// EFC Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EfcSw(Selector);
impl EfcSw {
    pub const fn new() -> Self {
        Self(Selector::new(30800, 4, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EfcSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EfcSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// EFC Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct EfcWlight(Led);
impl EfcWlight {
    pub const fn new() -> Self {
        Self(Led::new(30852, 4096, 12))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for EfcWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EfcWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Emergency Jettison Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EmergJett(Selector);
impl EmergJett {
    pub const fn new() -> Self {
        Self(Selector::new(30778, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EmergJett {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EmergJett {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// EMS Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EmsB(Selector);
impl EmsB {
    pub const fn new() -> Self {
        Self(Selector::new(30788, 32, 5))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EmsB {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EmsB {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ENG EXC Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct EngExcWlight(Led);
impl EngExcWlight {
    pub const fn new() -> Self {
        Self(Led::new(30854, 16384, 14))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for EngExcWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EngExcWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Engine Start Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EngStartSw(Selector);
impl EngStartSw {
    pub const fn new() -> Self {
        Self(Selector::new(30800, 1024, 10))
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

/// External Auxiliary Lights Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct ExtAuxLights(Selector);
impl ExtAuxLights {
    pub const fn new() -> Self {
        Self(Selector::new(30796, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ExtAuxLights {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ExtAuxLights {
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
        Self(Metadata::new(31056, 65535, 0))
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

/// External Lights Mode Switch NORM/NVG/OFF
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct ExtLights(Selector);
impl ExtLights {
    pub const fn new() -> Self {
        Self(Selector::new(30796, 768, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ExtLights {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ExtLights {
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
        Self(Metadata::new(30856, 32768, 15))
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

/// Speed Brake
/// Output: Speed Brake
pub struct ExtSpeedBrake(Metadata);
impl ExtSpeedBrake {
    pub const fn new() -> Self {
        Self(Metadata::new(31054, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ExtSpeedBrake {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ExtSpeedBrake {
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
        Self(Metadata::new(31058, 1, 0))
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

/// Weight ON Wheels Left Gear
/// Output: Weight ON Wheels Left Gear
pub struct ExtWowLeft(Metadata);
impl ExtWowLeft {
    pub const fn new() -> Self {
        Self(Metadata::new(31058, 8, 3))
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
        Self(Metadata::new(31058, 2, 1))
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
        Self(Metadata::new(31058, 4, 2))
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

/// Weight ON Wheels Tail Gear
/// Output: Weight ON Wheels Tail Gear
pub struct ExtWowTail(Metadata);
impl ExtWowTail {
    pub const fn new() -> Self {
        Self(Metadata::new(31058, 16, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ExtWowTail {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ExtWowTail {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// FF 1. Drum Digit
/// Output: gauge position
pub struct Ff1(AnalogGauge);
impl Ff1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30872, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Ff1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Ff1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// FF 2. Drum Digit
/// Output: gauge position
pub struct Ff2(AnalogGauge);
impl Ff2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30874, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Ff2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Ff2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// FF 3. Drum Digit
/// Output: gauge position
pub struct Ff3(AnalogGauge);
impl Ff3 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30876, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Ff3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Ff3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fire Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct FireLight(Led);
impl FireLight {
    pub const fn new() -> Self {
        Self(Led::new(30844, 1024, 10))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for FireLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FireLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flaps Mode Switch CRUISE/AUTO/STOL
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct FlapMode(Selector);
impl FlapMode {
    pub const fn new() -> Self {
        Self(Selector::new(30778, 384, 7))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FlapMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FlapMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flap Position 1. Drum Digit
/// Output: gauge position
pub struct FlapPos1(AnalogGauge);
impl FlapPos1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30982, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FlapPos1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FlapPos1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flap Position 2. Drum Digit
/// Output: gauge position
pub struct FlapPos2(AnalogGauge);
impl FlapPos2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30984, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FlapPos2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FlapPos2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flaps Power Switch RESET/ON/OFF
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct FlapPower(Selector);
impl FlapPower {
    pub const fn new() -> Self {
        Self(Selector::new(30778, 1536, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FlapPower {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FlapPower {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// FLAPS 1 Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct Flaps1Wlight(Led);
impl Flaps1Wlight {
    pub const fn new() -> Self {
        Self(Led::new(30850, 2048, 11))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Flaps1Wlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Flaps1Wlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// FLAPS 2 Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct Flaps2Wlight(Led);
impl Flaps2Wlight {
    pub const fn new() -> Self {
        Self(Led::new(30850, 4096, 12))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Flaps2Wlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Flaps2Wlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flaps BIT Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FlapsBit(Selector);
impl FlapsBit {
    pub const fn new() -> Self {
        Self(Selector::new(30778, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FlapsBit {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FlapsBit {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flaps Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct FlapsLight(Led);
impl FlapsLight {
    pub const fn new() -> Self {
        Self(Led::new(30844, 4096, 12))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for FlapsLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FlapsLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// FLIR Power Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Flir(Selector);
impl Flir {
    pub const fn new() -> Self {
        Self(Selector::new(30786, 8, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Flir {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Flir {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flood Left BH AFT Back
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct FloodLBhAftB(LimitedDial);
impl FloodLBhAftB {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30834, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FloodLBhAftB {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FloodLBhAftB {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flood Left BH AFT Front
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct FloodLBhAftF(LimitedDial);
impl FloodLBhAftF {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30836, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FloodLBhAftF {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FloodLBhAftF {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flood Left BH FWD
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct FloodLBhFwd(LimitedDial);
impl FloodLBhFwd {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30838, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FloodLBhFwd {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FloodLBhFwd {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flood Left Canopy Frame Btm
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct FloodLCanB(LimitedDial);
impl FloodLCanB {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30840, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FloodLCanB {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FloodLCanB {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flood Left Canopy Frame Top
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct FloodLCanT(LimitedDial);
impl FloodLCanT {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30842, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FloodLCanT {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FloodLCanT {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flood Lightning Intensity
/// Output: gauge position
pub struct FloodLights(AnalogGauge);
impl FloodLights {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31040, 65535, 0))
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

/// Flood Lights
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct FloodLightsDial(LimitedDial);
impl FloodLightsDial {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30818, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FloodLightsDial {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FloodLightsDial {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flood Right BH AFT Back
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct FloodRBhAftB(LimitedDial);
impl FloodRBhAftB {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30832, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FloodRBhAftB {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FloodRBhAftB {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flood Right BH AFT Front
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct FloodRBhAftF(LimitedDial);
impl FloodRBhAftF {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30830, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FloodRBhAftF {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FloodRBhAftF {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flood Right BH FWD
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct FloodRBhFwd(LimitedDial);
impl FloodRBhFwd {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30828, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FloodRBhFwd {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FloodRBhFwd {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flood Right Canopy Frame Btm
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct FloodRCanB(LimitedDial);
impl FloodRCanB {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30826, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FloodRCanB {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FloodRCanB {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Flood Right Canopy Frame Top
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct FloodRCanT(LimitedDial);
impl FloodRCanT {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30824, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FloodRCanT {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FloodRCanT {
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
pub struct FormationLights(LimitedDial);
impl FormationLights {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30798, 65535, 0))
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

/// Fuel Dump L Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FuelDumpL(Selector);
impl FuelDumpL {
    pub const fn new() -> Self {
        Self(Selector::new(30796, 16, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelDumpL {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelDumpL {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuel Dump R Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FuelDumpR(Selector);
impl FuelDumpR {
    pub const fn new() -> Self {
        Self(Selector::new(30796, 32, 5))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelDumpR {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelDumpR {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// FUEL ON/OFF Flag
/// Output: gauge position
pub struct FuelFlag(AnalogGauge);
impl FuelFlag {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30896, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelFlag {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelFlag {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuel Proportioner
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FuelProp(Selector);
impl FuelProp {
    pub const fn new() -> Self {
        Self(Selector::new(30788, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelProp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelProp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuel Pump L Switch DC OPEN/OFF/NORM
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct FuelPumpL(Selector);
impl FuelPumpL {
    pub const fn new() -> Self {
        Self(Selector::new(30788, 49152, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelPumpL {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelPumpL {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuel Pump R Switch DC OPEN/OFF/NORM
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct FuelPumpR(Selector);
impl FuelPumpR {
    pub const fn new() -> Self {
        Self(Selector::new(30796, 3, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelPumpR {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelPumpR {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuel Totalizer Selector OUTBD/INBD/WING/INT/TOT/FEED/BIT
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct FuelSel(Selector);
impl FuelSel {
    pub const fn new() -> Self {
        Self(Selector::new(30732, 57344, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuel Shutoff Lever
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FuelShutoff(Selector);
impl FuelShutoff {
    pub const fn new() -> Self {
        Self(Selector::new(30800, 32, 5))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelShutoff {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelShutoff {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuze 1. Drum Digit
/// Output: gauge position
pub struct Fuz1(AnalogGauge);
impl Fuz1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30934, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fuz1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fuz1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuze 2. Drum Digit
/// Output: gauge position
pub struct Fuz2(AnalogGauge);
impl Fuz2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30936, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Fuz2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Fuz2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuzing Control
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct FuzeControl(Selector);
impl FuzeControl {
    pub const fn new() -> Self {
        Self(Selector::new(30768, 12288, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuzeControl {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuzeControl {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Red Gear Lever Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct GearLeverLight(Led);
impl GearLeverLight {
    pub const fn new() -> Self {
        Self(Led::new(30850, 4, 2))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for GearLeverLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for GearLeverLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Gear Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct GearLight(Led);
impl GearLight {
    pub const fn new() -> Self {
        Self(Led::new(30848, 1, 0))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for GearLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for GearLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// GEN Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct GenLight(Led);
impl GenLight {
    pub const fn new() -> Self {
        Self(Led::new(30848, 16, 4))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for GenLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for GenLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Generator Switch  TEST/OFF/ON
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct GenSw(Selector);
impl GenSw {
    pub const fn new() -> Self {
        Self(Selector::new(30800, 6144, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for GenSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for GenSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Ground IR Cool Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct GndIrCool(Selector);
impl GndIrCool {
    pub const fn new() -> Self {
        Self(Selector::new(30774, 32, 5))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for GndIrCool {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for GndIrCool {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// GPS Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct GpsWlight(Led);
impl GpsWlight {
    pub const fn new() -> Self {
        Self(Led::new(30852, 8, 3))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for GpsWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for GpsWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// H2O 1. Drum Digit
/// Output: gauge position
pub struct H2o1(AnalogGauge);
impl H2o1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30890, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for H2o1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for H2o1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// H2O 2. Drum Digit
/// Output: gauge position
pub struct H2o2(AnalogGauge);
impl H2o2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30892, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for H2o2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for H2o2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// H2O Control Lamp (green)
/// Output: 0 if light is off, 1 if light is on
pub struct H2oControlLight(Led);
impl H2oControlLight {
    pub const fn new() -> Self {
        Self(Led::new(30856, 4, 2))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for H2oControlLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for H2oControlLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// H2O Dump Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct H2oDump(Selector);
impl H2oDump {
    pub const fn new() -> Self {
        Self(Selector::new(30796, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for H2oDump {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for H2oDump {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// H2O Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct H2oLight(Led);
impl H2oLight {
    pub const fn new() -> Self {
        Self(Led::new(30844, 512, 9))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for H2oLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for H2oLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// H2O Mode Switch  LDG/OFF/TO
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct H2oMode(Selector);
impl H2oMode {
    pub const fn new() -> Self {
        Self(Selector::new(30780, 12, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for H2oMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for H2oMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// H2O SEL Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct H2oSel(Led);
impl H2oSel {
    pub const fn new() -> Self {
        Self(Led::new(30854, 1, 0))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for H2oSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for H2oSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// HHYD Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct HhydLight(Led);
impl HhydLight {
    pub const fn new() -> Self {
        Self(Led::new(30844, 32768, 15))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for HhydLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HhydLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Show/Hide Controls
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct HideStick(Selector);
impl HideStick {
    pub const fn new() -> Self {
        Self(Selector::new(30856, 16384, 14))
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

/// HUD Altitude Selector Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct HudAlt(Selector);
impl HudAlt {
    pub const fn new() -> Self {
        Self(Selector::new(30720, 256, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HudAlt {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HudAlt {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// HUD Off/Brightness Control
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct HudBright(LimitedDial);
impl HudBright {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30724, 65535, 0))
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

/// HUD Display Mode Switch DAY/AUTO/NIGHT
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct HudMode(Selector);
impl HudMode {
    pub const fn new() -> Self {
        Self(Selector::new(30720, 192, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HudMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HudMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// HUD Symbology Reject Switch NORM/REJ1/REJ2
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct HudSymbol(Selector);
impl HudSymbol {
    pub const fn new() -> Self {
        Self(Selector::new(30720, 48, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HudSymbol {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HudSymbol {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// HUD Video Brightness Control
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct HudVideoBright(LimitedDial);
impl HudVideoBright {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30726, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HudVideoBright {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HudVideoBright {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// HUD Video Contrast Control
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct HudVideoContr(LimitedDial);
impl HudVideoContr {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30728, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HudVideoContr {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HudVideoContr {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Hydraulic 1 1. Drum Digit
/// Output: gauge position
pub struct Hyd11(AnalogGauge);
impl Hyd11 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30970, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Hyd11 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Hyd11 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Hydraulic 1 2. Drum Digit
/// Output: gauge position
pub struct Hyd12(AnalogGauge);
impl Hyd12 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30972, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Hyd12 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Hyd12 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Hydraulic 1 3. Drum Digit
/// Output: gauge position
pub struct Hyd13(AnalogGauge);
impl Hyd13 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30974, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Hyd13 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Hyd13 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// HYD 1 Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct Hyd1Wlight(Led);
impl Hyd1Wlight {
    pub const fn new() -> Self {
        Self(Led::new(30850, 32, 5))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Hyd1Wlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Hyd1Wlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Hydraulic 2 1. Drum Digit
/// Output: gauge position
pub struct Hyd21(AnalogGauge);
impl Hyd21 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30976, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Hyd21 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Hyd21 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Hydraulic 2 2. Drum Digit
/// Output: gauge position
pub struct Hyd22(AnalogGauge);
impl Hyd22 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30978, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Hyd22 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Hyd22 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Hydraulic 2 3. Drum Digit
/// Output: gauge position
pub struct Hyd23(AnalogGauge);
impl Hyd23 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30980, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Hyd23 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Hyd23 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// HYD 2 Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct Hyd2Wlight(Led);
impl Hyd2Wlight {
    pub const fn new() -> Self {
        Self(Led::new(30850, 64, 6))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Hyd2Wlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Hyd2Wlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ICS Aux Volume Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct IcsAuxVol(LimitedDial);
impl IcsAuxVol {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30810, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for IcsAuxVol {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IcsAuxVol {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ICS Ground Volume Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct IcsGndVol(LimitedDial);
impl IcsGndVol {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30812, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for IcsGndVol {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IcsGndVol {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ICS Mic Operational Mode Switch  COLD/HOT/TEL
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct IcsMicMode(Selector);
impl IcsMicMode {
    pub const fn new() -> Self {
        Self(Selector::new(30808, 12, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for IcsMicMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IcsMicMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// IFF Crypto Mode Switch  ZERO/NORM/HOLD
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct IffCryptMode(Selector);
impl IffCryptMode {
    pub const fn new() -> Self {
        Self(Selector::new(30808, 48, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for IffCryptMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IffCryptMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// IFF Operational Mode Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct IffOperMode(Selector);
impl IffOperMode {
    pub const fn new() -> Self {
        Self(Selector::new(30806, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for IffOperMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IffOperMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// IFF Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct IffWlight(Led);
impl IffWlight {
    pub const fn new() -> Self {
        Self(Led::new(30854, 8, 3))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for IffWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IffWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// INS Mode Knob OFF/SEA/GND/NAV/IFA/GYRO/GB/TEST
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct InsMode(Selector);
impl InsMode {
    pub const fn new() -> Self {
        Self(Selector::new(30786, 112, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// INS Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct InsWlight(Led);
impl InsWlight {
    pub const fn new() -> Self {
        Self(Led::new(30852, 1024, 10))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for InsWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Instruments Lights
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct InstLights(LimitedDial);
impl InstLights {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30814, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InstLights {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InstLights {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// InVT x 10 1. Drum Digit
/// Output: gauge position
pub struct Invt1(AnalogGauge);
impl Invt1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30938, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Invt1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Invt1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// InVT x 10 2. Drum Digit
/// Output: gauge position
pub struct Invt2(AnalogGauge);
impl Invt2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30940, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Invt2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Invt2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// InVT x 10 3. Drum Digit
/// Output: gauge position
pub struct Invt3(AnalogGauge);
impl Invt3 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30942, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Invt3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Invt3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// JAM HOT Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct JamHotWlight(Led);
impl JamHotWlight {
    pub const fn new() -> Self {
        Self(Led::new(30854, 8192, 13))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for JamHotWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JamHotWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Jammer Control OFF/STBY/BIT/RCV/RPT
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct JammerControl(Selector);
impl JammerControl {
    pub const fn new() -> Self {
        Self(Selector::new(30778, 56, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for JammerControl {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JammerControl {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Jettison Mode Selector STA/STOR/SAFE/CMBT/Fuel
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct JettMode(Selector);
impl JettMode {
    pub const fn new() -> Self {
        Self(Selector::new(30772, 7168, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for JettMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JettMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Select Station 1 Toggle
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct JettStation1(Selector);
impl JettStation1 {
    pub const fn new() -> Self {
        Self(Selector::new(30772, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for JettStation1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JettStation1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Select Station 1 LIGHT
/// Output: 0 if light is off, 1 if light is on
pub struct JettStation1Light(Led);
impl JettStation1Light {
    pub const fn new() -> Self {
        Self(Led::new(30774, 64, 6))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for JettStation1Light {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JettStation1Light {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Select Station 2 Toggle
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct JettStation2(Selector);
impl JettStation2 {
    pub const fn new() -> Self {
        Self(Selector::new(30772, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for JettStation2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JettStation2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Select Station 2 LIGHT
/// Output: 0 if light is off, 1 if light is on
pub struct JettStation2Light(Led);
impl JettStation2Light {
    pub const fn new() -> Self {
        Self(Led::new(30774, 128, 7))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for JettStation2Light {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JettStation2Light {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Select Station 3 Toggle
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct JettStation3(Selector);
impl JettStation3 {
    pub const fn new() -> Self {
        Self(Selector::new(30774, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for JettStation3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JettStation3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Select Station 3 LIGHT
/// Output: 0 if light is off, 1 if light is on
pub struct JettStation3Light(Led);
impl JettStation3Light {
    pub const fn new() -> Self {
        Self(Led::new(30774, 256, 8))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for JettStation3Light {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JettStation3Light {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Select Station 4 Toggle
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct JettStation4(Selector);
impl JettStation4 {
    pub const fn new() -> Self {
        Self(Selector::new(30774, 2, 1))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for JettStation4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JettStation4 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Select Station 4 LIGHT
/// Output: 0 if light is off, 1 if light is on
pub struct JettStation4Light(Led);
impl JettStation4Light {
    pub const fn new() -> Self {
        Self(Led::new(30774, 512, 9))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for JettStation4Light {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JettStation4Light {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Select Station 5 Toggle
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct JettStation5(Selector);
impl JettStation5 {
    pub const fn new() -> Self {
        Self(Selector::new(30774, 4, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for JettStation5 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JettStation5 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Select Station 5 LIGHT
/// Output: 0 if light is off, 1 if light is on
pub struct JettStation5Light(Led);
impl JettStation5Light {
    pub const fn new() -> Self {
        Self(Led::new(30774, 1024, 10))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for JettStation5Light {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JettStation5Light {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Select Station 6 Toggle
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct JettStation6(Selector);
impl JettStation6 {
    pub const fn new() -> Self {
        Self(Selector::new(30774, 8, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for JettStation6 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JettStation6 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Select Station 6 LIGHT
/// Output: 0 if light is off, 1 if light is on
pub struct JettStation6Light(Led);
impl JettStation6Light {
    pub const fn new() -> Self {
        Self(Led::new(30774, 2048, 11))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for JettStation6Light {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JettStation6Light {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Select Station 7 Toggle
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct JettStation7(Selector);
impl JettStation7 {
    pub const fn new() -> Self {
        Self(Selector::new(30774, 16, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for JettStation7 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JettStation7 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Select Station 7 LIGHT
/// Output: 0 if light is off, 1 if light is on
pub struct JettStation7Light(Led);
impl JettStation7Light {
    pub const fn new() -> Self {
        Self(Led::new(30774, 4096, 12))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for JettStation7Light {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JettStation7Light {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Jettison Stores
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct JettStores(Selector);
impl JettStores {
    pub const fn new() -> Self {
        Self(Selector::new(30772, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for JettStores {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JettStores {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// JPT 1. Drum Digit
/// Output: gauge position
pub struct Jpt1(AnalogGauge);
impl Jpt1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30878, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Jpt1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Jpt1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// JPT 2. Drum Digit
/// Output: gauge position
pub struct Jpt2(AnalogGauge);
impl Jpt2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30880, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Jpt2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Jpt2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// JPT 3. Drum Digit
/// Output: gauge position
pub struct Jpt3(AnalogGauge);
impl Jpt3 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30882, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Jpt3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Jpt3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// JPTL Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct JptlLight(Led);
impl JptlLight {
    pub const fn new() -> Self {
        Self(Led::new(30848, 4, 2))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for JptlLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JptlLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// JPTL Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct JptlSw(Selector);
impl JptlSw {
    pub const fn new() -> Self {
        Self(Selector::new(30788, 4, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for JptlSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for JptlSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Speed knt Pointer Long
/// Output: gauge position
pub struct KntPointerL(AnalogGauge);
impl KntPointerL {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30994, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for KntPointerL {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for KntPointerL {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Speed knt Pointer Short
/// Output: gauge position
pub struct KntPointerS(AnalogGauge);
impl KntPointerS {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30992, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for KntPointerS {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for KntPointerS {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// KY-1 Code/Mode Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Ky1Mode(Selector);
impl Ky1Mode {
    pub const fn new() -> Self {
        Self(Selector::new(30806, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Ky1Mode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Ky1Mode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// KY-1 Cipher Type Selector Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Ky1Type(Selector);
impl Ky1Type {
    pub const fn new() -> Self {
        Self(Selector::new(30806, 512, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Ky1Type {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Ky1Type {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// KY-2 Code/Mode Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Ky2Mode(Selector);
impl Ky2Mode {
    pub const fn new() -> Self {
        Self(Selector::new(30806, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Ky2Mode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Ky2Mode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// KY-2 Cipher Type Selector Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Ky2Type(Selector);
impl Ky2Type {
    pub const fn new() -> Self {
        Self(Selector::new(30806, 1024, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Ky2Type {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Ky2Type {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// KY-58 Codes Clear Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Ky58CodeClear(Selector);
impl Ky58CodeClear {
    pub const fn new() -> Self {
        Self(Selector::new(30806, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Ky58CodeClear {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Ky58CodeClear {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// KY-58 Remote Codes Load Switch  RV1/OPR/RV2
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct Ky58CodeLoad(Selector);
impl Ky58CodeLoad {
    pub const fn new() -> Self {
        Self(Selector::new(30808, 3, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Ky58CodeLoad {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Ky58CodeLoad {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Landing/Taxi Lights Switch OFF/HVR/APRCH
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct LandingTaxiLights(Selector);
impl LandingTaxiLights {
    pub const fn new() -> Self {
        Self(Selector::new(30796, 192, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LandingTaxiLights {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LandingTaxiLights {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Launch Flare Salvo
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct LaunchFlareSalvo(Selector);
impl LaunchFlareSalvo {
    pub const fn new() -> Self {
        Self(Selector::new(30774, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LaunchFlareSalvo {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LaunchFlareSalvo {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// LAW Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct LawLight(Led);
impl LawLight {
    pub const fn new() -> Self {
        Self(Led::new(30844, 2048, 11))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for LawLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LawLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// LBS Left 1. Drum Digit
/// Output: gauge position
pub struct LbsL1(AnalogGauge);
impl LbsL1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30908, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LbsL1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LbsL1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// LBS Left 2. Drum Digit
/// Output: gauge position
pub struct LbsL2(AnalogGauge);
impl LbsL2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30910, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LbsL2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LbsL2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// LBS Left 3. Drum Digit
/// Output: gauge position
pub struct LbsL3(AnalogGauge);
impl LbsL3 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30912, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LbsL3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LbsL3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// LBS Left 4. Drum Digit
/// Output: gauge position
pub struct LbsL4(AnalogGauge);
impl LbsL4 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30914, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LbsL4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LbsL4 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// LBS Right 1. Drum Digit
/// Output: gauge position
pub struct LbsR1(AnalogGauge);
impl LbsR1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30916, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LbsR1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LbsR1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// LBS Right 2. Drum Digit
/// Output: gauge position
pub struct LbsR2(AnalogGauge);
impl LbsR2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30918, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LbsR2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LbsR2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// LBS Right 3. Drum Digit
/// Output: gauge position
pub struct LbsR3(AnalogGauge);
impl LbsR3 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30920, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LbsR3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LbsR3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// LBS Right 4. Drum Digit
/// Output: gauge position
pub struct LbsR4(AnalogGauge);
impl LbsR4 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30922, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LbsR4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LbsR4 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Left Gear Green Light
/// Output: 0 if light is off, 1 if light is on
pub struct LeftGearG(Led);
impl LeftGearG {
    pub const fn new() -> Self {
        Self(Led::new(30856, 64, 6))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for LeftGearG {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LeftGearG {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Left Gear Yellow Light
/// Output: 0 if light is off, 1 if light is on
pub struct LeftGearY(Led);
impl LeftGearY {
    pub const fn new() -> Self {
        Self(Led::new(30856, 32, 5))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for LeftGearY {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LeftGearY {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Left Lamp Canopy
/// Output: 0 if light is off, 1 if light is on
pub struct LeftLight(Led);
impl LeftLight {
    pub const fn new() -> Self {
        Self(Led::new(30848, 32768, 15))
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

/// Left Fuel Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct LfuelLight(Led);
impl LfuelLight {
    pub const fn new() -> Self {
        Self(Led::new(30844, 16, 4))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for LfuelLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LfuelLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Gear Down Lock Override Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct LgDownLock(Selector);
impl LgDownLock {
    pub const fn new() -> Self {
        Self(Selector::new(30778, 64, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LgDownLock {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LgDownLock {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Emergency Landing Gear Lever
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct LgEmergLever(Selector);
impl LgEmergLever {
    pub const fn new() -> Self {
        Self(Selector::new(30774, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LgEmergLever {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LgEmergLever {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Landing Gear Emergency Battery Lever
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct LgEmergbattLever(Selector);
impl LgEmergbattLever {
    pub const fn new() -> Self {
        Self(Selector::new(30780, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LgEmergbattLever {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LgEmergbattLever {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Gear Lever
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct LgLever(Selector);
impl LgLever {
    pub const fn new() -> Self {
        Self(Selector::new(30778, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LgLever {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LgLever {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// LIDS Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct LidsSw(Selector);
impl LidsSw {
    pub const fn new() -> Self {
        Self(Selector::new(30800, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LidsSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LidsSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// LIDS Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct LidsWlight(Led);
impl LidsWlight {
    pub const fn new() -> Self {
        Self(Led::new(30850, 32768, 15))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for LidsWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LidsWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// 2 Line --  Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct Line2Wlight(Led);
impl Line2Wlight {
    pub const fn new() -> Self {
        Self(Led::new(30854, 512, 9))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Line2Wlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Line2Wlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Line -- Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct LineWlight(Led);
impl LineWlight {
    pub const fn new() -> Self {
        Self(Led::new(30852, 4, 2))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for LineWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LineWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// LOAD Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct LoadWlight(Led);
impl LoadWlight {
    pub const fn new() -> Self {
        Self(Led::new(30852, 256, 8))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for LoadWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LoadWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Left Pump Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct LpumpWlight(Led);
impl LpumpWlight {
    pub const fn new() -> Self {
        Self(Led::new(30850, 128, 7))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for LpumpWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LpumpWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Left Tank Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct LtankLight(Led);
impl LtankLight {
    pub const fn new() -> Self {
        Self(Led::new(30844, 8192, 13))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for LtankLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LtankLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// L TRANS Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct LtransWlight(Led);
impl LtransWlight {
    pub const fn new() -> Self {
        Self(Led::new(30850, 512, 9))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for LtransWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LtransWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Master Caution
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MCaution(Selector);
impl MCaution {
    pub const fn new() -> Self {
        Self(Selector::new(30732, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MCaution {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MCaution {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Master Warning
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MWarning(Selector);
impl MWarning {
    pub const fn new() -> Self {
        Self(Selector::new(30732, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MWarning {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MWarning {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Main Gear Green Light
/// Output: 0 if light is off, 1 if light is on
pub struct MainGearG(Led);
impl MainGearG {
    pub const fn new() -> Self {
        Self(Led::new(30856, 1024, 10))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for MainGearG {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MainGearG {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Main Gear Yellow Light
/// Output: 0 if light is off, 1 if light is on
pub struct MainGearY(Led);
impl MainGearY {
    pub const fn new() -> Self {
        Self(Led::new(30856, 512, 9))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for MainGearY {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MainGearY {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Manual Fuel Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct ManFuelSw(Selector);
impl ManFuelSw {
    pub const fn new() -> Self {
        Self(Selector::new(30788, 64, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ManFuelSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ManFuelSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Master Arm Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MasterArm(Selector);
impl MasterArm {
    pub const fn new() -> Self {
        Self(Selector::new(30774, 16384, 14))
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

/// MasterCaution Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct McLight(Led);
impl McLight {
    pub const fn new() -> Self {
        Self(Led::new(30844, 4, 2))
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

/// MFS Emergency Lever
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MfsEmergLever(Selector);
impl MfsEmergLever {
    pub const fn new() -> Self {
        Self(Selector::new(30780, 16, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MfsEmergLever {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MfsEmergLever {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MFS Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct MfsLight(Led);
impl MfsLight {
    pub const fn new() -> Self {
        Self(Led::new(30844, 128, 7))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for MfsLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MfsLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MIP Backlight Intensity
/// Output: gauge position
pub struct MipLights(AnalogGauge);
impl MipLights {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31042, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MipLights {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MipLights {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Mirror Rendering Toggle Center
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MirrorToggleC(Selector);
impl MirrorToggleC {
    pub const fn new() -> Self {
        Self(Selector::new(30856, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MirrorToggleC {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MirrorToggleC {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Mirror Rendering Toggle Left
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MirrorToggleL(Selector);
impl MirrorToggleL {
    pub const fn new() -> Self {
        Self(Selector::new(30856, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MirrorToggleL {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MirrorToggleL {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Mirror Rendering Toggle Right
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MirrorToggleR(Selector);
impl MirrorToggleR {
    pub const fn new() -> Self {
        Self(Selector::new(30856, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MirrorToggleR {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MirrorToggleR {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Mission Computer Mode Switch OVRD/AUTO/OFF
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct MissPcMode(Selector);
impl MissPcMode {
    pub const fn new() -> Self {
        Self(Selector::new(30786, 6, 1))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MissPcMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MissPcMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MODE Drum Digit
/// Output: gauge position
pub struct ModeGauge(AnalogGauge);
impl ModeGauge {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30932, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ModeGauge {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ModeGauge {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Button 1
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdL1(Selector);
impl MpcdL1 {
    pub const fn new() -> Self {
        Self(Selector::new(30750, 256, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdL1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdL1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Button 10
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdL10(Selector);
impl MpcdL10 {
    pub const fn new() -> Self {
        Self(Selector::new(30758, 512, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdL10 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdL10 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Button 11
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdL11(Selector);
impl MpcdL11 {
    pub const fn new() -> Self {
        Self(Selector::new(30758, 1024, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdL11 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdL11 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Button 12
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdL12(Selector);
impl MpcdL12 {
    pub const fn new() -> Self {
        Self(Selector::new(30758, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdL12 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdL12 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Button 13
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdL13(Selector);
impl MpcdL13 {
    pub const fn new() -> Self {
        Self(Selector::new(30758, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdL13 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdL13 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Button 14
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdL14(Selector);
impl MpcdL14 {
    pub const fn new() -> Self {
        Self(Selector::new(30758, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdL14 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdL14 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Button 15
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdL15(Selector);
impl MpcdL15 {
    pub const fn new() -> Self {
        Self(Selector::new(30758, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdL15 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdL15 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Button 16
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdL16(Selector);
impl MpcdL16 {
    pub const fn new() -> Self {
        Self(Selector::new(30758, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdL16 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdL16 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Button 17
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdL17(Selector);
impl MpcdL17 {
    pub const fn new() -> Self {
        Self(Selector::new(30762, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdL17 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdL17 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Button 18
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdL18(Selector);
impl MpcdL18 {
    pub const fn new() -> Self {
        Self(Selector::new(30762, 2, 1))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdL18 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdL18 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Button 19
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdL19(Selector);
impl MpcdL19 {
    pub const fn new() -> Self {
        Self(Selector::new(30762, 4, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdL19 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdL19 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Button 2
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdL2(Selector);
impl MpcdL2 {
    pub const fn new() -> Self {
        Self(Selector::new(30750, 512, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdL2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdL2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Button 20
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdL20(Selector);
impl MpcdL20 {
    pub const fn new() -> Self {
        Self(Selector::new(30762, 8, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdL20 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdL20 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Button 3
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdL3(Selector);
impl MpcdL3 {
    pub const fn new() -> Self {
        Self(Selector::new(30750, 1024, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdL3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdL3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Button 4
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdL4(Selector);
impl MpcdL4 {
    pub const fn new() -> Self {
        Self(Selector::new(30750, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdL4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdL4 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Button 5
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdL5(Selector);
impl MpcdL5 {
    pub const fn new() -> Self {
        Self(Selector::new(30750, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdL5 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdL5 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Button 6
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdL6(Selector);
impl MpcdL6 {
    pub const fn new() -> Self {
        Self(Selector::new(30750, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdL6 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdL6 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Button 7
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdL7(Selector);
impl MpcdL7 {
    pub const fn new() -> Self {
        Self(Selector::new(30750, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdL7 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdL7 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Button 8
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdL8(Selector);
impl MpcdL8 {
    pub const fn new() -> Self {
        Self(Selector::new(30750, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdL8 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdL8 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Button 9
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdL9(Selector);
impl MpcdL9 {
    pub const fn new() -> Self {
        Self(Selector::new(30758, 256, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdL9 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdL9 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Off/Brightness Control
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct MpcdLBright(LimitedDial);
impl MpcdLBright {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30764, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdLBright {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdLBright {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Display Contrast
/// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
/// Output: selector position
pub struct MpcdLContrast(Selector);
impl MpcdLContrast {
    pub const fn new() -> Self {
        Self(Selector::new(30762, 3072, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdLContrast {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdLContrast {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Display DAY/NIGHT Mode
/// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
/// Output: selector position
pub struct MpcdLDayNight(Selector);
impl MpcdLDayNight {
    pub const fn new() -> Self {
        Self(Selector::new(30762, 48, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdLDayNight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdLDayNight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Display Gain
/// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
/// Output: selector position
pub struct MpcdLGain(Selector);
impl MpcdLGain {
    pub const fn new() -> Self {
        Self(Selector::new(30762, 768, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdLGain {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdLGain {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Left Display Symbology
/// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
/// Output: selector position
pub struct MpcdLSymbol(Selector);
impl MpcdLSymbol {
    pub const fn new() -> Self {
        Self(Selector::new(30762, 192, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdLSymbol {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdLSymbol {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Button 1
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdR1(Selector);
impl MpcdR1 {
    pub const fn new() -> Self {
        Self(Selector::new(30762, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdR1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdR1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Button 10
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdR10(Selector);
impl MpcdR10 {
    pub const fn new() -> Self {
        Self(Selector::new(30766, 32, 5))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdR10 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdR10 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Button 11
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdR11(Selector);
impl MpcdR11 {
    pub const fn new() -> Self {
        Self(Selector::new(30766, 64, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdR11 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdR11 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Button 12
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdR12(Selector);
impl MpcdR12 {
    pub const fn new() -> Self {
        Self(Selector::new(30766, 128, 7))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdR12 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdR12 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Button 13
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdR13(Selector);
impl MpcdR13 {
    pub const fn new() -> Self {
        Self(Selector::new(30766, 256, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdR13 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdR13 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Button 14
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdR14(Selector);
impl MpcdR14 {
    pub const fn new() -> Self {
        Self(Selector::new(30766, 512, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdR14 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdR14 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Button 15
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdR15(Selector);
impl MpcdR15 {
    pub const fn new() -> Self {
        Self(Selector::new(30766, 1024, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdR15 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdR15 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Button 16
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdR16(Selector);
impl MpcdR16 {
    pub const fn new() -> Self {
        Self(Selector::new(30766, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdR16 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdR16 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Button 17
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdR17(Selector);
impl MpcdR17 {
    pub const fn new() -> Self {
        Self(Selector::new(30766, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdR17 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdR17 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Button 18
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdR18(Selector);
impl MpcdR18 {
    pub const fn new() -> Self {
        Self(Selector::new(30766, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdR18 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdR18 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Button 19
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdR19(Selector);
impl MpcdR19 {
    pub const fn new() -> Self {
        Self(Selector::new(30766, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdR19 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdR19 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Button 2
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdR2(Selector);
impl MpcdR2 {
    pub const fn new() -> Self {
        Self(Selector::new(30762, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdR2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdR2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Button 20
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdR20(Selector);
impl MpcdR20 {
    pub const fn new() -> Self {
        Self(Selector::new(30766, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdR20 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdR20 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Button 3
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdR3(Selector);
impl MpcdR3 {
    pub const fn new() -> Self {
        Self(Selector::new(30762, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdR3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdR3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Button 4
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdR4(Selector);
impl MpcdR4 {
    pub const fn new() -> Self {
        Self(Selector::new(30762, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdR4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdR4 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Button 5
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdR5(Selector);
impl MpcdR5 {
    pub const fn new() -> Self {
        Self(Selector::new(30766, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdR5 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdR5 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Button 6
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdR6(Selector);
impl MpcdR6 {
    pub const fn new() -> Self {
        Self(Selector::new(30766, 2, 1))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdR6 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdR6 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Button 7
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdR7(Selector);
impl MpcdR7 {
    pub const fn new() -> Self {
        Self(Selector::new(30766, 4, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdR7 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdR7 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Button 8
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdR8(Selector);
impl MpcdR8 {
    pub const fn new() -> Self {
        Self(Selector::new(30766, 8, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdR8 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdR8 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Button 9
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MpcdR9(Selector);
impl MpcdR9 {
    pub const fn new() -> Self {
        Self(Selector::new(30766, 16, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdR9 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdR9 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Off/Brightness Control
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct MpcdRBright(LimitedDial);
impl MpcdRBright {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30770, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdRBright {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdRBright {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Display Contrast
/// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
/// Output: selector position
pub struct MpcdRContrast(Selector);
impl MpcdRContrast {
    pub const fn new() -> Self {
        Self(Selector::new(30768, 192, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdRContrast {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdRContrast {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Display DAY/NIGHT Mode
/// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
/// Output: selector position
pub struct MpcdRDayNight(Selector);
impl MpcdRDayNight {
    pub const fn new() -> Self {
        Self(Selector::new(30768, 3, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdRDayNight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdRDayNight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Display Gain
/// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
/// Output: selector position
pub struct MpcdRGain(Selector);
impl MpcdRGain {
    pub const fn new() -> Self {
        Self(Selector::new(30768, 48, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdRGain {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdRGain {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MPCD Right Display Symbology
/// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
/// Output: selector position
pub struct MpcdRSymbol(Selector);
impl MpcdRSymbol {
    pub const fn new() -> Self {
        Self(Selector::new(30768, 12, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MpcdRSymbol {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MpcdRSymbol {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Multiplicator Drum Digit
/// Output: gauge position
pub struct MultGauge(AnalogGauge);
impl MultGauge {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30944, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MultGauge {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MultGauge {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Multiple Release Control
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct MultiReleaseControl(Selector);
impl MultiReleaseControl {
    pub const fn new() -> Self {
        Self(Selector::new(30772, 12, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MultiReleaseControl {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MultiReleaseControl {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// MasterWarning Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct MwLight(Led);
impl MwLight {
    pub const fn new() -> Self {
        Self(Led::new(30844, 8, 3))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for MwLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MwLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// NAV Course Setting
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct NavCrs(Selector);
impl NavCrs {
    pub const fn new() -> Self {
        Self(Selector::new(30780, 768, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NavCrs {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NavCrs {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// NAV Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct NavLight(Led);
impl NavLight {
    pub const fn new() -> Self {
        Self(Led::new(30848, 512, 9))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for NavLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NavLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// NAV Master Mode Selector
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct NavSel(Selector);
impl NavSel {
    pub const fn new() -> Self {
        Self(Selector::new(30720, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NavSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NavSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Nose Gear Green Light
/// Output: 0 if light is off, 1 if light is on
pub struct NoseGearG(Led);
impl NoseGearG {
    pub const fn new() -> Self {
        Self(Led::new(30856, 16, 4))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for NoseGearG {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NoseGearG {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Nose Gear Yellow Light
/// Output: 0 if light is off, 1 if light is on
pub struct NoseGearY(Led);
impl NoseGearY {
    pub const fn new() -> Self {
        Self(Led::new(30856, 8, 3))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for NoseGearY {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NoseGearY {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Nozzle Lever Friction Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct NozzleFriction(LimitedDial);
impl NozzleFriction {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30792, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NozzleFriction {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NozzleFriction {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Nozzle Control Lever
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct NozzleLever(LimitedDial);
impl NozzleLever {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30794, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NozzleLever {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NozzleLever {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Nozzle Pointer
/// Output: gauge position
pub struct NozzlePointer(AnalogGauge);
impl NozzlePointer {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30894, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NozzlePointer {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NozzlePointer {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// NWS Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct NwsWlight(Led);
impl NwsWlight {
    pub const fn new() -> Self {
        Self(Led::new(30852, 8192, 13))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for NwsWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NwsWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Oxygen Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct O2Sw(Selector);
impl O2Sw {
    pub const fn new() -> Self {
        Self(Selector::new(30796, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for O2Sw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for O2Sw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ODU Option 1
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct OduOpt1(Selector);
impl OduOpt1 {
    pub const fn new() -> Self {
        Self(Selector::new(30732, 64, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OduOpt1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OduOpt1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ODU Option 2
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct OduOpt2(Selector);
impl OduOpt2 {
    pub const fn new() -> Self {
        Self(Selector::new(30732, 128, 7))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OduOpt2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OduOpt2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ODU Option 3
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct OduOpt3(Selector);
impl OduOpt3 {
    pub const fn new() -> Self {
        Self(Selector::new(30732, 256, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OduOpt3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OduOpt3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ODU Option 4
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct OduOpt4(Selector);
impl OduOpt4 {
    pub const fn new() -> Self {
        Self(Selector::new(30732, 512, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OduOpt4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OduOpt4 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ODU Option 5
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct OduOpt5(Selector);
impl OduOpt5 {
    pub const fn new() -> Self {
        Self(Selector::new(30732, 1024, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OduOpt5 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OduOpt5 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// OIL Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct OilWlight(Led);
impl OilWlight {
    pub const fn new() -> Self {
        Self(Led::new(30852, 1, 0))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for OilWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OilWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// OT Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct OtLight(Led);
impl OtLight {
    pub const fn new() -> Self {
        Self(Led::new(30848, 2, 1))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for OtLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OtLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// OXY Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct OxyWlight(Led);
impl OxyWlight {
    pub const fn new() -> Self {
        Self(Led::new(30850, 8, 3))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for OxyWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OxyWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// P JAM Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct PJamWlight(Led);
impl PJamWlight {
    pub const fn new() -> Self {
        Self(Led::new(30854, 4096, 12))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PJamWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PJamWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// P NOGO Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct PNogoWlight(Led);
impl PNogoWlight {
    pub const fn new() -> Self {
        Self(Led::new(30854, 32768, 15))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PNogoWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PNogoWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Parking Brake Lever
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct ParkBrakeLv(Selector);
impl ParkBrakeLv {
    pub const fn new() -> Self {
        Self(Selector::new(30788, 256, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ParkBrakeLv {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ParkBrakeLv {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// PITCH Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct PitchWlight(Led);
impl PitchWlight {
    pub const fn new() -> Self {
        Self(Led::new(30854, 4, 2))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PitchWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PitchWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Position Lights Switch OFF/DIM/BRT
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct PosLights(Selector);
impl PosLights {
    pub const fn new() -> Self {
        Self(Selector::new(30796, 3072, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PosLights {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PosLights {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Air Refueling Probe Switch IN/OUT/PRESS
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct Probe(Selector);
impl Probe {
    pub const fn new() -> Self {
        Self(Selector::new(30796, 12, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Probe {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Probe {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Probe Heat Mode Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct ProbeHeatMode(Selector);
impl ProbeHeatMode {
    pub const fn new() -> Self {
        Self(Selector::new(30786, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ProbeHeatMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ProbeHeatMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// PROP Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct PropWlight(Led);
impl PropWlight {
    pub const fn new() -> Self {
        Self(Led::new(30850, 16384, 14))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PropWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PropWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Q Feel Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct QFeelSw(Selector);
impl QFeelSw {
    pub const fn new() -> Self {
        Self(Selector::new(30786, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for QFeelSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for QFeelSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Quantity 1. Drum Digit
/// Output: gauge position
pub struct Qty1(AnalogGauge);
impl Qty1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30946, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Qty1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Qty1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Quantity 2. Drum Digit
/// Output: gauge position
pub struct Qty2(AnalogGauge);
impl Qty2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30948, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Qty2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Qty2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Quantity Selector (Units)
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct QuantSel01(Selector);
impl QuantSel01 {
    pub const fn new() -> Self {
        Self(Selector::new(30772, 3, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for QuantSel01 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for QuantSel01 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Quantity Selector (Tens)
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct QuantSel10(Selector);
impl QuantSel10 {
    pub const fn new() -> Self {
        Self(Selector::new(30768, 49152, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for QuantSel10 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for QuantSel10 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Ready Lamp Canopy
/// Output: 0 if light is off, 1 if light is on
pub struct ReadyLight(Led);
impl ReadyLight {
    pub const fn new() -> Self {
        Self(Led::new(30850, 1, 0))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ReadyLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ReadyLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Video Recorder System Display Selector Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RecordSysDisplay(Selector);
impl RecordSysDisplay {
    pub const fn new() -> Self {
        Self(Selector::new(30780, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RecordSysDisplay {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RecordSysDisplay {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Video Recorder System Mode Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RecordSysMode(Selector);
impl RecordSysMode {
    pub const fn new() -> Self {
        Self(Selector::new(30780, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RecordSysMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RecordSysMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Manual Release Control Knob NORM/N+T/N/T
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct ReleaseControl(Selector);
impl ReleaseControl {
    pub const fn new() -> Self {
        Self(Selector::new(30768, 768, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ReleaseControl {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ReleaseControl {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Release Interval Control (Units)
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct ReleaseInt001(Selector);
impl ReleaseInt001 {
    pub const fn new() -> Self {
        Self(Selector::new(30772, 768, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ReleaseInt001 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ReleaseInt001 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Release Interval Control (Tens)
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct ReleaseInt010(Selector);
impl ReleaseInt010 {
    pub const fn new() -> Self {
        Self(Selector::new(30772, 192, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ReleaseInt010 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ReleaseInt010 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Release Interval Control (Hundreds)
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct ReleaseInt100(Selector);
impl ReleaseInt100 {
    pub const fn new() -> Self {
        Self(Selector::new(30772, 48, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ReleaseInt100 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ReleaseInt100 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// REPLY Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct ReplyWlight(Led);
impl ReplyWlight {
    pub const fn new() -> Self {
        Self(Led::new(30856, 2, 1))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ReplyWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ReplyWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Right Fuel Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct RfuelLight(Led);
impl RfuelLight {
    pub const fn new() -> Self {
        Self(Led::new(30844, 32, 5))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for RfuelLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RfuelLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Right Gear Green Light
/// Output: 0 if light is off, 1 if light is on
pub struct RightGearG(Led);
impl RightGearG {
    pub const fn new() -> Self {
        Self(Led::new(30856, 256, 8))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for RightGearG {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RightGearG {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Right Gear Yellow Light
/// Output: 0 if light is off, 1 if light is on
pub struct RightGearY(Led);
impl RightGearY {
    pub const fn new() -> Self {
        Self(Led::new(30856, 128, 7))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for RightGearY {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RightGearY {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Right Lamp Canopy
/// Output: 0 if light is off, 1 if light is on
pub struct RightLight(Led);
impl RightLight {
    pub const fn new() -> Self {
        Self(Led::new(30850, 2, 1))
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

/// ROLL Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct RollWlight(Led);
impl RollWlight {
    pub const fn new() -> Self {
        Self(Led::new(30854, 64, 6))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for RollWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RollWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// RPM 1. Drum Digit
/// Output: gauge position
pub struct Rpm1(AnalogGauge);
impl Rpm1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30864, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Rpm1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Rpm1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// RPM 2. Drum Digit
/// Output: gauge position
pub struct Rpm2(AnalogGauge);
impl Rpm2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30866, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Rpm2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Rpm2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// RPM 3. Drum Digit
/// Output: gauge position
pub struct Rpm3(AnalogGauge);
impl Rpm3 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30868, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Rpm3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Rpm3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// RPM 4. Drum Digit
/// Output: gauge position
pub struct Rpm4(AnalogGauge);
impl Rpm4 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30870, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Rpm4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Rpm4 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// ENG RPM Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RpmSw(Selector);
impl RpmSw {
    pub const fn new() -> Self {
        Self(Selector::new(30800, 2, 1))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RpmSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RpmSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Right Pump Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct RpumpWlight(Led);
impl RpumpWlight {
    pub const fn new() -> Self {
        Self(Led::new(30850, 256, 8))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for RpumpWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RpumpWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Right Tank Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct RtankLight(Led);
impl RtankLight {
    pub const fn new() -> Self {
        Self(Led::new(30844, 16384, 14))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for RtankLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RtankLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// R TRANS Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct RtransWlight(Led);
impl RtransWlight {
    pub const fn new() -> Self {
        Self(Led::new(30850, 1024, 10))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for RtransWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RtransWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Rudder Needle
/// Output: gauge position
pub struct RudderNeedle(AnalogGauge);
impl RudderNeedle {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31022, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RudderNeedle {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RudderNeedle {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Rudder Trim Switch L/R
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct RudderTrim(Selector);
impl RudderTrim {
    pub const fn new() -> Self {
        Self(Selector::new(30788, 24, 3))
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

/// RWR Power/Volume Button
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct RwrVol(LimitedDial);
impl RwrVol {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30776, 65535, 0))
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

/// SAM Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct SamLight(Led);
impl SamLight {
    pub const fn new() -> Self {
        Self(Led::new(30848, 32, 5))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for SamLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SamLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// SAS Pitch Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct SasPitch(Selector);
impl SasPitch {
    pub const fn new() -> Self {
        Self(Selector::new(30788, 2, 1))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SasPitch {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SasPitch {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// SAS Roll Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct SasRoll(Selector);
impl SasRoll {
    pub const fn new() -> Self {
        Self(Selector::new(30788, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SasRoll {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SasRoll {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// SAS Yaw Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct SasYaw(Selector);
impl SasYaw {
    pub const fn new() -> Self {
        Self(Selector::new(30786, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SasYaw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SasYaw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Seat Adjustment Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct SeatAdj(Selector);
impl SeatAdj {
    pub const fn new() -> Self {
        Self(Selector::new(30800, 24, 3))
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

/// Seat Ground Safety Lever
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct SeatSafeLever(Selector);
impl SeatSafeLever {
    pub const fn new() -> Self {
        Self(Selector::new(30780, 128, 7))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SeatSafeLever {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SeatSafeLever {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// SEL Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct SelLight(Led);
impl SelLight {
    pub const fn new() -> Self {
        Self(Led::new(30848, 4096, 12))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for SelLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SelLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// SET FLAG 1 Gauge
/// Output: gauge position
pub struct SetFlag1(AnalogGauge);
impl SetFlag1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30950, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SetFlag1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SetFlag1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// SET FLAG 2 Gauge
/// Output: gauge position
pub struct SetFlag2(AnalogGauge);
impl SetFlag2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30952, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SetFlag2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SetFlag2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// SET FLAG 3 Gauge
/// Output: gauge position
pub struct SetFlag3(AnalogGauge);
impl SetFlag3 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30954, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SetFlag3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SetFlag3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// SET FLAG 4 Gauge
/// Output: gauge position
pub struct SetFlag4(AnalogGauge);
impl SetFlag4 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30956, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SetFlag4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SetFlag4 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// SET FLAG 5 Gauge
/// Output: gauge position
pub struct SetFlag5(AnalogGauge);
impl SetFlag5 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30958, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SetFlag5 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SetFlag5 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// SET FLAG 6 Gauge
/// Output: gauge position
pub struct SetFlag6(AnalogGauge);
impl SetFlag6 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30960, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SetFlag6 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SetFlag6 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// SET FLAG 7 Gauge
/// Output: gauge position
pub struct SetFlag7(AnalogGauge);
impl SetFlag7 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30962, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SetFlag7 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SetFlag7 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Sidepanel Lightning Intensity
/// Output: gauge position
pub struct SidepanelLights(AnalogGauge);
impl SidepanelLights {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31044, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SidepanelLights {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SidepanelLights {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// SKID Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct SkidWlight(Led);
impl SkidWlight {
    pub const fn new() -> Self {
        Self(Led::new(30852, 2048, 11))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for SkidWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SkidWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Slip Indicator Ball
/// Output: gauge position
pub struct SlipInd(AnalogGauge);
impl SlipInd {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31018, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SlipInd {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SlipInd {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Slip/Turn Flag
/// Output: gauge position
pub struct SlipTurnFlag(AnalogGauge);
impl SlipTurnFlag {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31052, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SlipTurnFlag {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SlipTurnFlag {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// SPD BRK Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct SpdBrkWlight(Led);
impl SpdBrkWlight {
    pub const fn new() -> Self {
        Self(Led::new(30854, 16, 4))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for SpdBrkWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SpdBrkWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// STAB 1. Drum Digit
/// Output: gauge position
pub struct Stab1(AnalogGauge);
impl Stab1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30886, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Stab1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Stab1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// STAB 2. Drum Digit
/// Output: gauge position
pub struct Stab2(AnalogGauge);
impl Stab2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30888, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Stab2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Stab2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// STAB Arrow up/down Drum Digit
/// Output: gauge position
pub struct StabArrow(AnalogGauge);
impl StabArrow {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30884, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for StabArrow {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for StabArrow {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// STBY TR Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct StbyTrWlight(Led);
impl StbyTrWlight {
    pub const fn new() -> Self {
        Self(Led::new(30852, 64, 6))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for StbyTrWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for StbyTrWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// STO Stop Lever
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct StoLever(Selector);
impl StoLever {
    pub const fn new() -> Self {
        Self(Selector::new(30788, 7680, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for StoLever {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for StoLever {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// STO Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct StoLight(Led);
impl StoLight {
    pub const fn new() -> Self {
        Self(Led::new(30848, 16384, 14))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for StoLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for StoLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Stopwatch Lap/Reset
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct StopwatchLap(Selector);
impl StopwatchLap {
    pub const fn new() -> Self {
        Self(Selector::new(30780, 64, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for StopwatchLap {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for StopwatchLap {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Stopwatch Start/Stop
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct StopwatchStart(Selector);
impl StopwatchStart {
    pub const fn new() -> Self {
        Self(Selector::new(30780, 32, 5))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for StopwatchStart {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for StopwatchStart {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Temperature Control Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct TempControl(LimitedDial);
impl TempControl {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30822, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TempControl {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TempControl {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Throttle Cutoff Lever
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct ThrottCutoffLv(Selector);
impl ThrottCutoffLv {
    pub const fn new() -> Self {
        Self(Selector::new(30788, 128, 7))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ThrottCutoffLv {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ThrottCutoffLv {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Throttle Lever Friction Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct ThrottleFriction(LimitedDial);
impl ThrottleFriction {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30790, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ThrottleFriction {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ThrottleFriction {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// TOT FUEL 1. Drum Digit
/// Output: gauge position
pub struct TotFuel1(AnalogGauge);
impl TotFuel1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30898, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TotFuel1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TotFuel1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// TOT FUEL 2. Drum Digit
/// Output: gauge position
pub struct TotFuel2(AnalogGauge);
impl TotFuel2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30900, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TotFuel2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TotFuel2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// TOT FUEL 3. Drum Digit
/// Output: gauge position
pub struct TotFuel3(AnalogGauge);
impl TotFuel3 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30902, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TotFuel3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TotFuel3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// TOT FUEL 4. Drum Digit
/// Output: gauge position
pub struct TotFuel4(AnalogGauge);
impl TotFuel4 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30904, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TotFuel4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TotFuel4 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// TOT FUEL 5. Drum Digit
/// Output: gauge position
pub struct TotFuel5(AnalogGauge);
impl TotFuel5 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(30906, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TotFuel5 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TotFuel5 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Trim Mode Switch APRCH/HVR/OFF
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct TrimMode(Selector);
impl TrimMode {
    pub const fn new() -> Self {
        Self(Selector::new(30786, 1536, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TrimMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TrimMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Turn Indicator
/// Output: gauge position
pub struct TurnIndi(AnalogGauge);
impl TurnIndi {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31050, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TurnIndi {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TurnIndi {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Altitude Function Selector
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcAlt(Selector);
impl UfcAlt {
    pub const fn new() -> Self {
        Self(Selector::new(30730, 256, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcAlt {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcAlt {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Button 0
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcB0(Selector);
impl UfcB0 {
    pub const fn new() -> Self {
        Self(Selector::new(30730, 4, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcB0 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcB0 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Button 1
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcB1(Selector);
impl UfcB1 {
    pub const fn new() -> Self {
        Self(Selector::new(30720, 512, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcB1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcB1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Button 2/N
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcB2(Selector);
impl UfcB2 {
    pub const fn new() -> Self {
        Self(Selector::new(30720, 1024, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcB2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcB2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Button 3
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcB3(Selector);
impl UfcB3 {
    pub const fn new() -> Self {
        Self(Selector::new(30720, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcB3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcB3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Button 4/W
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcB4(Selector);
impl UfcB4 {
    pub const fn new() -> Self {
        Self(Selector::new(30720, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcB4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcB4 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Button 5
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcB5(Selector);
impl UfcB5 {
    pub const fn new() -> Self {
        Self(Selector::new(30720, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcB5 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcB5 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Button 6/E
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcB6(Selector);
impl UfcB6 {
    pub const fn new() -> Self {
        Self(Selector::new(30720, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcB6 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcB6 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Button 7
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcB7(Selector);
impl UfcB7 {
    pub const fn new() -> Self {
        Self(Selector::new(30720, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcB7 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcB7 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Button 8/S
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcB8(Selector);
impl UfcB8 {
    pub const fn new() -> Self {
        Self(Selector::new(30730, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcB8 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcB8 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Button 9
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcB9(Selector);
impl UfcB9 {
    pub const fn new() -> Self {
        Self(Selector::new(30730, 2, 1))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcB9 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcB9 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Radar Beacon Function Selector
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcBeacon(Selector);
impl UfcBeacon {
    pub const fn new() -> Self {
        Self(Selector::new(30732, 2, 1))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcBeacon {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcBeacon {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Display Brightness Control
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct UfcBright(LimitedDial);
impl UfcBright {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30734, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcBright {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcBright {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Clear Entry Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcClear(Selector);
impl UfcClear {
    pub const fn new() -> Self {
        Self(Selector::new(30730, 64, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcClear {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcClear {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Comm 1 Channel Selector Pull
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcCom1Pull(Selector);
impl UfcCom1Pull {
    pub const fn new() -> Self {
        Self(Selector::new(30732, 16, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcCom1Pull {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcCom1Pull {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Comm 1 Channel Selector Turn
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct UfcCom1Sel(AnalogDial);
impl UfcCom1Sel {
    pub const fn new() -> Self {
        Self(AnalogDial::new(30740, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcCom1Sel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcCom1Sel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Comm 1 Volume Control
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct UfcCom1Vol(LimitedDial);
impl UfcCom1Vol {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30736, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcCom1Vol {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcCom1Vol {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Comm 2 Channel Selector Pull
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcCom2Pull(Selector);
impl UfcCom2Pull {
    pub const fn new() -> Self {
        Self(Selector::new(30732, 32, 5))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcCom2Pull {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcCom2Pull {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Comm 2 Channel Selector Turn
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct UfcCom2Sel(AnalogDial);
impl UfcCom2Sel {
    pub const fn new() -> Self {
        Self(AnalogDial::new(30742, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcCom2Sel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcCom2Sel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Comm 2 Volume Control
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct UfcCom2Vol(LimitedDial);
impl UfcCom2Vol {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30738, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcCom2Vol {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcCom2Vol {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Comm1 Preset Display
/// Output: UFC Comm1 Preset Display
pub struct UfcComm1Display(Display<2>);
impl UfcComm1Display {
    pub const fn new() -> Self {
        Self(Display::<2>::new(31060, 2))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for UfcComm1Display {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcComm1Display {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Comm2 Preset Display
/// Output: UFC Comm2 Preset Display
pub struct UfcComm2Display(Display<2>);
impl UfcComm2Display {
    pub const fn new() -> Self {
        Self(Display::<2>::new(31062, 2))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for UfcComm2Display {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcComm2Display {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Button -
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcDash(Selector);
impl UfcDash {
    pub const fn new() -> Self {
        Self(Selector::new(30730, 16, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcDash {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcDash {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Button .
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcDot(Selector);
impl UfcDot {
    pub const fn new() -> Self {
        Self(Selector::new(30730, 8, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcDot {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcDot {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC EMCON ON/OFF Toggle
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcEmcom(Selector);
impl UfcEmcom {
    pub const fn new() -> Self {
        Self(Selector::new(30730, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcEmcom {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcEmcom {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC ENTER Data Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcEnter(Selector);
impl UfcEnter {
    pub const fn new() -> Self {
        Self(Selector::new(30730, 32, 5))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcEnter {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcEnter {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC IFF Function Selector
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcIff(Selector);
impl UfcIff {
    pub const fn new() -> Self {
        Self(Selector::new(30730, 512, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcIff {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcIff {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC I/P Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcIp(Selector);
impl UfcIp {
    pub const fn new() -> Self {
        Self(Selector::new(30732, 4, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcIp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcIp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Radalt/TACAN/AWL/IFF ON/OFF Toggle
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcOnoff(Selector);
impl UfcOnoff {
    pub const fn new() -> Self {
        Self(Selector::new(30730, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcOnoff {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcOnoff {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Save Data Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcSave(Selector);
impl UfcSave {
    pub const fn new() -> Self {
        Self(Selector::new(30732, 8, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcSave {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcSave {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Scratchpad Display
/// Output: UFC Scratchpad Display
pub struct UfcScratchpad(Display<12>);
impl UfcScratchpad {
    pub const fn new() -> Self {
        Self(Display::<12>::new(31108, 12))
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for UfcScratchpad {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcScratchpad {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC TACAN Function Selector
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcTacan(Selector);
impl UfcTacan {
    pub const fn new() -> Self {
        Self(Selector::new(30730, 1024, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcTacan {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcTacan {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Target-Of-Opportunity Selector
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcTarget(Selector);
impl UfcTarget {
    pub const fn new() -> Self {
        Self(Selector::new(30730, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcTarget {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcTarget {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Timer Function Selector
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcTimer(Selector);
impl UfcTimer {
    pub const fn new() -> Self {
        Self(Selector::new(30730, 128, 7))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcTimer {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcTimer {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Waypoint Overfly INS Update Selector
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcWay(Selector);
impl UfcWay {
    pub const fn new() -> Self {
        Self(Selector::new(30730, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcWay {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcWay {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC Weapons Function Selector
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcWeapon(Selector);
impl UfcWeapon {
    pub const fn new() -> Self {
        Self(Selector::new(30732, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcWeapon {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcWeapon {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UFC All Weather Landing Function Selector
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UfcWeather(Selector);
impl UfcWeather {
    pub const fn new() -> Self {
        Self(Selector::new(30730, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UfcWeather {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UfcWeather {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// V/UHF RSC Ancillary Mode Pointer
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UhfAncPoint(Selector);
impl UhfAncPoint {
    pub const fn new() -> Self {
        Self(Selector::new(30800, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UhfAncPoint {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UhfAncPoint {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// V/UHF RSC Ancillary Mode Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UhfAncSw(Selector);
impl UhfAncSw {
    pub const fn new() -> Self {
        Self(Selector::new(30806, 8, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UhfAncSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UhfAncSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// V/UHF RSC Chan/Freq Knob
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct UhfFreq(AnalogDial);
impl UhfFreq {
    pub const fn new() -> Self {
        Self(AnalogDial::new(30804, 65535, 0))
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

/// V/UHF RSC LOAD/OFST Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UhfLoadSw(Selector);
impl UhfLoadSw {
    pub const fn new() -> Self {
        Self(Selector::new(30806, 128, 7))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UhfLoadSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UhfLoadSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// V/UHF RSC Operational Mode Switch ZRO/OFF/TEST/TR+G/TR/ADF/CHNG PRST
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct UhfMode(Selector);
impl UhfMode {
    pub const fn new() -> Self {
        Self(Selector::new(30806, 7, 0))
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

/// V/UHF RSC Volume Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct UhfVol(LimitedDial);
impl UhfVol {
    pub const fn new() -> Self {
        Self(LimitedDial::new(30802, 65535, 0))
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

/// Volt Meter Pointer
/// Output: gauge position
pub struct VoltMeter(AnalogGauge);
impl VoltMeter {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31026, 65535, 0))
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

/// VSTOL Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct VstolLight(Led);
impl VstolLight {
    pub const fn new() -> Self {
        Self(Led::new(30848, 1024, 10))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for VstolLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VstolLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// VSTOL Master Mode Selector
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct VstolSel(Selector);
impl VstolSel {
    pub const fn new() -> Self {
        Self(Selector::new(30720, 2, 1))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VstolSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VstolSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Vertical Velocity Indicator
/// Output: gauge position
pub struct Vvi(AnalogGauge);
impl Vvi {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(31016, 65535, 0))
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

/// WSLD Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct WsldWlight(Led);
impl WsldWlight {
    pub const fn new() -> Self {
        Self(Led::new(30850, 16, 4))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for WsldWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for WsldWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// RPS/YAW Trim Switch TEST/ON/OFF
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct YawTrimSw(Selector);
impl YawTrimSw {
    pub const fn new() -> Self {
        Self(Selector::new(30786, 384, 7))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for YawTrimSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for YawTrimSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// YAW Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct YawWlight(Led);
impl YawWlight {
    pub const fn new() -> Self {
        Self(Led::new(30854, 1024, 10))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for YawWlight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for YawWlight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

pub enum RegisteredControl<'a> {
    AaaLight {
        control: AaaLight,
        callback: LedCallback<'a>,
    },
    AcnipMode {
        control: AcnipMode,
        callback: IntegerCallback<'a>,
    },
    AcnipRadioSel {
        control: AcnipRadioSel,
        callback: IntegerCallback<'a>,
    },
    AdiBank {
        control: AdiBank,
        callback: IntegerCallback<'a>,
    },
    AdiFlag {
        control: AdiFlag,
        callback: IntegerCallback<'a>,
    },
    AdiPitch {
        control: AdiPitch,
        callback: IntegerCallback<'a>,
    },
    AfcSw {
        control: AfcSw,
        callback: IntegerCallback<'a>,
    },
    AfcWlight {
        control: AfcWlight,
        callback: LedCallback<'a>,
    },
    AftBayWlight {
        control: AftBayWlight,
        callback: LedCallback<'a>,
    },
    AgLight {
        control: AgLight,
        callback: LedCallback<'a>,
    },
    AgSel {
        control: AgSel,
        callback: IntegerCallback<'a>,
    },
    AiLight {
        control: AiLight,
        callback: LedCallback<'a>,
    },
    AileronNeedle {
        control: AileronNeedle,
        callback: IntegerCallback<'a>,
    },
    Alt10000ftCnt {
        control: Alt10000ftCnt,
        callback: IntegerCallback<'a>,
    },
    Alt1000ftCnt {
        control: Alt1000ftCnt,
        callback: IntegerCallback<'a>,
    },
    Alt100ftCnt {
        control: Alt100ftCnt,
        callback: IntegerCallback<'a>,
    },
    AltHoldSw {
        control: AltHoldSw,
        callback: IntegerCallback<'a>,
    },
    AltPointer {
        control: AltPointer,
        callback: IntegerCallback<'a>,
    },
    AltPressure1 {
        control: AltPressure1,
        callback: IntegerCallback<'a>,
    },
    AltPressure2 {
        control: AltPressure2,
        callback: IntegerCallback<'a>,
    },
    AltPressure3 {
        control: AltPressure3,
        callback: IntegerCallback<'a>,
    },
    AltPressure4 {
        control: AltPressure4,
        callback: IntegerCallback<'a>,
    },
    AnnunciatorLights {
        control: AnnunciatorLights,
        callback: IntegerCallback<'a>,
    },
    AntiCollLights {
        control: AntiCollLights,
        callback: IntegerCallback<'a>,
    },
    AntiSkid {
        control: AntiSkid,
        callback: IntegerCallback<'a>,
    },
    AoaFlag {
        control: AoaFlag,
        callback: IntegerCallback<'a>,
    },
    AoaPointer {
        control: AoaPointer,
        callback: IntegerCallback<'a>,
    },
    ApuGenSw {
        control: ApuGenSw,
        callback: IntegerCallback<'a>,
    },
    ApuGenWlight {
        control: ApuGenWlight,
        callback: LedCallback<'a>,
    },
    ApuWlight {
        control: ApuWlight,
        callback: LedCallback<'a>,
    },
    ArmControl {
        control: ArmControl,
        callback: IntegerCallback<'a>,
    },
    AutflpWlight {
        control: AutflpWlight,
        callback: LedCallback<'a>,
    },
    Av8bnaOdu1Select {
        control: Av8bnaOdu1Select,
        callback: DisplayCallback<'a>,
    },
    Av8bnaOdu1Text {
        control: Av8bnaOdu1Text,
        callback: DisplayCallback<'a>,
    },
    Av8bnaOdu2Select {
        control: Av8bnaOdu2Select,
        callback: DisplayCallback<'a>,
    },
    Av8bnaOdu2Text {
        control: Av8bnaOdu2Text,
        callback: DisplayCallback<'a>,
    },
    Av8bnaOdu3Select {
        control: Av8bnaOdu3Select,
        callback: DisplayCallback<'a>,
    },
    Av8bnaOdu3Text {
        control: Av8bnaOdu3Text,
        callback: DisplayCallback<'a>,
    },
    Av8bnaOdu4Select {
        control: Av8bnaOdu4Select,
        callback: DisplayCallback<'a>,
    },
    Av8bnaOdu4Text {
        control: Av8bnaOdu4Text,
        callback: DisplayCallback<'a>,
    },
    Av8bnaOdu5Select {
        control: Av8bnaOdu5Select,
        callback: DisplayCallback<'a>,
    },
    Av8bnaOdu5Text {
        control: Av8bnaOdu5Text,
        callback: DisplayCallback<'a>,
    },
    AvBitWlight {
        control: AvBitWlight,
        callback: LedCallback<'a>,
    },
    BakAdiCageKnob {
        control: BakAdiCageKnob,
        callback: IntegerCallback<'a>,
    },
    BakAdiCagePull {
        control: BakAdiCagePull,
        callback: IntegerCallback<'a>,
    },
    BaroPressure {
        control: BaroPressure,
        callback: IntegerCallback<'a>,
    },
    BattSw {
        control: BattSw,
        callback: IntegerCallback<'a>,
    },
    Bingo1 {
        control: Bingo1,
        callback: IntegerCallback<'a>,
    },
    Bingo2 {
        control: Bingo2,
        callback: IntegerCallback<'a>,
    },
    Bingo3 {
        control: Bingo3,
        callback: IntegerCallback<'a>,
    },
    Bingo4 {
        control: Bingo4,
        callback: IntegerCallback<'a>,
    },
    BingoLight {
        control: BingoLight,
        callback: LedCallback<'a>,
    },
    BingoSet {
        control: BingoSet,
        callback: IntegerCallback<'a>,
    },
    BombsArmSw {
        control: BombsArmSw,
        callback: IntegerCallback<'a>,
    },
    Brake1 {
        control: Brake1,
        callback: IntegerCallback<'a>,
    },
    Brake2 {
        control: Brake2,
        callback: IntegerCallback<'a>,
    },
    Brake3 {
        control: Brake3,
        callback: IntegerCallback<'a>,
    },
    BrakeAccu {
        control: BrakeAccu,
        callback: IntegerCallback<'a>,
    },
    CAut {
        control: CAut,
        callback: LedCallback<'a>,
    },
    CabinDefog {
        control: CabinDefog,
        callback: IntegerCallback<'a>,
    },
    CabinPress {
        control: CabinPress,
        callback: IntegerCallback<'a>,
    },
    CabinPressSwitch {
        control: CabinPressSwitch,
        callback: IntegerCallback<'a>,
    },
    CanopyHandL {
        control: CanopyHandL,
        callback: IntegerCallback<'a>,
    },
    CanopyHandR {
        control: CanopyHandR,
        callback: IntegerCallback<'a>,
    },
    CanopyLock {
        control: CanopyLock,
        callback: IntegerCallback<'a>,
    },
    CanopyValue {
        control: CanopyValue,
        callback: IntegerCallback<'a>,
    },
    CanopyWlight {
        control: CanopyWlight,
        callback: LedCallback<'a>,
    },
    ClockHh {
        control: ClockHh,
        callback: IntegerCallback<'a>,
    },
    ClockMm {
        control: ClockMm,
        callback: IntegerCallback<'a>,
    },
    ClockSs {
        control: ClockSs,
        callback: IntegerCallback<'a>,
    },
    Cmbt {
        control: Cmbt,
        callback: IntegerCallback<'a>,
    },
    CmbtLight {
        control: CmbtLight,
        callback: LedCallback<'a>,
    },
    Comm1StringFreq {
        control: Comm1StringFreq,
        callback: DisplayCallback<'a>,
    },
    Comm2StringFreq {
        control: Comm2StringFreq,
        callback: DisplayCallback<'a>,
    },
    CompLight {
        control: CompLight,
        callback: IntegerCallback<'a>,
    },
    CompassBank {
        control: CompassBank,
        callback: IntegerCallback<'a>,
    },
    CompassLights {
        control: CompassLights,
        callback: IntegerCallback<'a>,
    },
    CompassNs {
        control: CompassNs,
        callback: IntegerCallback<'a>,
    },
    CompassUd {
        control: CompassUd,
        callback: IntegerCallback<'a>,
    },
    ConsoleLights {
        control: ConsoleLights,
        callback: IntegerCallback<'a>,
    },
    Control15sekLight {
        control: Control15sekLight,
        callback: LedCallback<'a>,
    },
    CsCoolWlight {
        control: CsCoolWlight,
        callback: LedCallback<'a>,
    },
    CwJamWlight {
        control: CwJamWlight,
        callback: LedCallback<'a>,
    },
    CwLight {
        control: CwLight,
        callback: LedCallback<'a>,
    },
    CwNogoWlight {
        control: CwNogoWlight,
        callback: LedCallback<'a>,
    },
    DcTestSw {
        control: DcTestSw,
        callback: IntegerCallback<'a>,
    },
    DcWlight {
        control: DcWlight,
        callback: LedCallback<'a>,
    },
    DecoyControl {
        control: DecoyControl,
        callback: IntegerCallback<'a>,
    },
    DecsSw {
        control: DecsSw,
        callback: IntegerCallback<'a>,
    },
    DepResWlight {
        control: DepResWlight,
        callback: LedCallback<'a>,
    },
    Dmt {
        control: Dmt,
        callback: IntegerCallback<'a>,
    },
    DroopWlight {
        control: DroopWlight,
        callback: LedCallback<'a>,
    },
    DualCpuMode {
        control: DualCpuMode,
        callback: IntegerCallback<'a>,
    },
    Duct1 {
        control: Duct1,
        callback: IntegerCallback<'a>,
    },
    Duct2 {
        control: Duct2,
        callback: IntegerCallback<'a>,
    },
    Duct3 {
        control: Duct3,
        callback: IntegerCallback<'a>,
    },
    EcsAft {
        control: EcsAft,
        callback: IntegerCallback<'a>,
    },
    EcsFwd {
        control: EcsFwd,
        callback: IntegerCallback<'a>,
    },
    EdpBit {
        control: EdpBit,
        callback: IntegerCallback<'a>,
    },
    EdpBright {
        control: EdpBright,
        callback: IntegerCallback<'a>,
    },
    EfcLight {
        control: EfcLight,
        callback: LedCallback<'a>,
    },
    EfcSw {
        control: EfcSw,
        callback: IntegerCallback<'a>,
    },
    EfcWlight {
        control: EfcWlight,
        callback: LedCallback<'a>,
    },
    EmergJett {
        control: EmergJett,
        callback: IntegerCallback<'a>,
    },
    EmsB {
        control: EmsB,
        callback: IntegerCallback<'a>,
    },
    EngExcWlight {
        control: EngExcWlight,
        callback: LedCallback<'a>,
    },
    EngStartSw {
        control: EngStartSw,
        callback: IntegerCallback<'a>,
    },
    ExtAuxLights {
        control: ExtAuxLights,
        callback: IntegerCallback<'a>,
    },
    ExtFormationLights {
        control: ExtFormationLights,
        callback: IntegerCallback<'a>,
    },
    ExtLights {
        control: ExtLights,
        callback: IntegerCallback<'a>,
    },
    ExtPositionLights {
        control: ExtPositionLights,
        callback: IntegerCallback<'a>,
    },
    ExtSpeedBrake {
        control: ExtSpeedBrake,
        callback: IntegerCallback<'a>,
    },
    ExtStrobeTail {
        control: ExtStrobeTail,
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
    ExtWowTail {
        control: ExtWowTail,
        callback: IntegerCallback<'a>,
    },
    Ff1 {
        control: Ff1,
        callback: IntegerCallback<'a>,
    },
    Ff2 {
        control: Ff2,
        callback: IntegerCallback<'a>,
    },
    Ff3 {
        control: Ff3,
        callback: IntegerCallback<'a>,
    },
    FireLight {
        control: FireLight,
        callback: LedCallback<'a>,
    },
    FlapMode {
        control: FlapMode,
        callback: IntegerCallback<'a>,
    },
    FlapPos1 {
        control: FlapPos1,
        callback: IntegerCallback<'a>,
    },
    FlapPos2 {
        control: FlapPos2,
        callback: IntegerCallback<'a>,
    },
    FlapPower {
        control: FlapPower,
        callback: IntegerCallback<'a>,
    },
    Flaps1Wlight {
        control: Flaps1Wlight,
        callback: LedCallback<'a>,
    },
    Flaps2Wlight {
        control: Flaps2Wlight,
        callback: LedCallback<'a>,
    },
    FlapsBit {
        control: FlapsBit,
        callback: IntegerCallback<'a>,
    },
    FlapsLight {
        control: FlapsLight,
        callback: LedCallback<'a>,
    },
    Flir {
        control: Flir,
        callback: IntegerCallback<'a>,
    },
    FloodLBhAftB {
        control: FloodLBhAftB,
        callback: IntegerCallback<'a>,
    },
    FloodLBhAftF {
        control: FloodLBhAftF,
        callback: IntegerCallback<'a>,
    },
    FloodLBhFwd {
        control: FloodLBhFwd,
        callback: IntegerCallback<'a>,
    },
    FloodLCanB {
        control: FloodLCanB,
        callback: IntegerCallback<'a>,
    },
    FloodLCanT {
        control: FloodLCanT,
        callback: IntegerCallback<'a>,
    },
    FloodLights {
        control: FloodLights,
        callback: IntegerCallback<'a>,
    },
    FloodLightsDial {
        control: FloodLightsDial,
        callback: IntegerCallback<'a>,
    },
    FloodRBhAftB {
        control: FloodRBhAftB,
        callback: IntegerCallback<'a>,
    },
    FloodRBhAftF {
        control: FloodRBhAftF,
        callback: IntegerCallback<'a>,
    },
    FloodRBhFwd {
        control: FloodRBhFwd,
        callback: IntegerCallback<'a>,
    },
    FloodRCanB {
        control: FloodRCanB,
        callback: IntegerCallback<'a>,
    },
    FloodRCanT {
        control: FloodRCanT,
        callback: IntegerCallback<'a>,
    },
    FormationLights {
        control: FormationLights,
        callback: IntegerCallback<'a>,
    },
    FuelDumpL {
        control: FuelDumpL,
        callback: IntegerCallback<'a>,
    },
    FuelDumpR {
        control: FuelDumpR,
        callback: IntegerCallback<'a>,
    },
    FuelFlag {
        control: FuelFlag,
        callback: IntegerCallback<'a>,
    },
    FuelProp {
        control: FuelProp,
        callback: IntegerCallback<'a>,
    },
    FuelPumpL {
        control: FuelPumpL,
        callback: IntegerCallback<'a>,
    },
    FuelPumpR {
        control: FuelPumpR,
        callback: IntegerCallback<'a>,
    },
    FuelSel {
        control: FuelSel,
        callback: IntegerCallback<'a>,
    },
    FuelShutoff {
        control: FuelShutoff,
        callback: IntegerCallback<'a>,
    },
    Fuz1 {
        control: Fuz1,
        callback: IntegerCallback<'a>,
    },
    Fuz2 {
        control: Fuz2,
        callback: IntegerCallback<'a>,
    },
    FuzeControl {
        control: FuzeControl,
        callback: IntegerCallback<'a>,
    },
    GearLeverLight {
        control: GearLeverLight,
        callback: LedCallback<'a>,
    },
    GearLight {
        control: GearLight,
        callback: LedCallback<'a>,
    },
    GenLight {
        control: GenLight,
        callback: LedCallback<'a>,
    },
    GenSw {
        control: GenSw,
        callback: IntegerCallback<'a>,
    },
    GndIrCool {
        control: GndIrCool,
        callback: IntegerCallback<'a>,
    },
    GpsWlight {
        control: GpsWlight,
        callback: LedCallback<'a>,
    },
    H2o1 {
        control: H2o1,
        callback: IntegerCallback<'a>,
    },
    H2o2 {
        control: H2o2,
        callback: IntegerCallback<'a>,
    },
    H2oControlLight {
        control: H2oControlLight,
        callback: LedCallback<'a>,
    },
    H2oDump {
        control: H2oDump,
        callback: IntegerCallback<'a>,
    },
    H2oLight {
        control: H2oLight,
        callback: LedCallback<'a>,
    },
    H2oMode {
        control: H2oMode,
        callback: IntegerCallback<'a>,
    },
    H2oSel {
        control: H2oSel,
        callback: LedCallback<'a>,
    },
    HhydLight {
        control: HhydLight,
        callback: LedCallback<'a>,
    },
    HideStick {
        control: HideStick,
        callback: IntegerCallback<'a>,
    },
    HudAlt {
        control: HudAlt,
        callback: IntegerCallback<'a>,
    },
    HudBright {
        control: HudBright,
        callback: IntegerCallback<'a>,
    },
    HudMode {
        control: HudMode,
        callback: IntegerCallback<'a>,
    },
    HudSymbol {
        control: HudSymbol,
        callback: IntegerCallback<'a>,
    },
    HudVideoBright {
        control: HudVideoBright,
        callback: IntegerCallback<'a>,
    },
    HudVideoContr {
        control: HudVideoContr,
        callback: IntegerCallback<'a>,
    },
    Hyd11 {
        control: Hyd11,
        callback: IntegerCallback<'a>,
    },
    Hyd12 {
        control: Hyd12,
        callback: IntegerCallback<'a>,
    },
    Hyd13 {
        control: Hyd13,
        callback: IntegerCallback<'a>,
    },
    Hyd1Wlight {
        control: Hyd1Wlight,
        callback: LedCallback<'a>,
    },
    Hyd21 {
        control: Hyd21,
        callback: IntegerCallback<'a>,
    },
    Hyd22 {
        control: Hyd22,
        callback: IntegerCallback<'a>,
    },
    Hyd23 {
        control: Hyd23,
        callback: IntegerCallback<'a>,
    },
    Hyd2Wlight {
        control: Hyd2Wlight,
        callback: LedCallback<'a>,
    },
    IcsAuxVol {
        control: IcsAuxVol,
        callback: IntegerCallback<'a>,
    },
    IcsGndVol {
        control: IcsGndVol,
        callback: IntegerCallback<'a>,
    },
    IcsMicMode {
        control: IcsMicMode,
        callback: IntegerCallback<'a>,
    },
    IffCryptMode {
        control: IffCryptMode,
        callback: IntegerCallback<'a>,
    },
    IffOperMode {
        control: IffOperMode,
        callback: IntegerCallback<'a>,
    },
    IffWlight {
        control: IffWlight,
        callback: LedCallback<'a>,
    },
    InsMode {
        control: InsMode,
        callback: IntegerCallback<'a>,
    },
    InsWlight {
        control: InsWlight,
        callback: LedCallback<'a>,
    },
    InstLights {
        control: InstLights,
        callback: IntegerCallback<'a>,
    },
    Invt1 {
        control: Invt1,
        callback: IntegerCallback<'a>,
    },
    Invt2 {
        control: Invt2,
        callback: IntegerCallback<'a>,
    },
    Invt3 {
        control: Invt3,
        callback: IntegerCallback<'a>,
    },
    JamHotWlight {
        control: JamHotWlight,
        callback: LedCallback<'a>,
    },
    JammerControl {
        control: JammerControl,
        callback: IntegerCallback<'a>,
    },
    JettMode {
        control: JettMode,
        callback: IntegerCallback<'a>,
    },
    JettStation1 {
        control: JettStation1,
        callback: IntegerCallback<'a>,
    },
    JettStation1Light {
        control: JettStation1Light,
        callback: LedCallback<'a>,
    },
    JettStation2 {
        control: JettStation2,
        callback: IntegerCallback<'a>,
    },
    JettStation2Light {
        control: JettStation2Light,
        callback: LedCallback<'a>,
    },
    JettStation3 {
        control: JettStation3,
        callback: IntegerCallback<'a>,
    },
    JettStation3Light {
        control: JettStation3Light,
        callback: LedCallback<'a>,
    },
    JettStation4 {
        control: JettStation4,
        callback: IntegerCallback<'a>,
    },
    JettStation4Light {
        control: JettStation4Light,
        callback: LedCallback<'a>,
    },
    JettStation5 {
        control: JettStation5,
        callback: IntegerCallback<'a>,
    },
    JettStation5Light {
        control: JettStation5Light,
        callback: LedCallback<'a>,
    },
    JettStation6 {
        control: JettStation6,
        callback: IntegerCallback<'a>,
    },
    JettStation6Light {
        control: JettStation6Light,
        callback: LedCallback<'a>,
    },
    JettStation7 {
        control: JettStation7,
        callback: IntegerCallback<'a>,
    },
    JettStation7Light {
        control: JettStation7Light,
        callback: LedCallback<'a>,
    },
    JettStores {
        control: JettStores,
        callback: IntegerCallback<'a>,
    },
    Jpt1 {
        control: Jpt1,
        callback: IntegerCallback<'a>,
    },
    Jpt2 {
        control: Jpt2,
        callback: IntegerCallback<'a>,
    },
    Jpt3 {
        control: Jpt3,
        callback: IntegerCallback<'a>,
    },
    JptlLight {
        control: JptlLight,
        callback: LedCallback<'a>,
    },
    JptlSw {
        control: JptlSw,
        callback: IntegerCallback<'a>,
    },
    KntPointerL {
        control: KntPointerL,
        callback: IntegerCallback<'a>,
    },
    KntPointerS {
        control: KntPointerS,
        callback: IntegerCallback<'a>,
    },
    Ky1Mode {
        control: Ky1Mode,
        callback: IntegerCallback<'a>,
    },
    Ky1Type {
        control: Ky1Type,
        callback: IntegerCallback<'a>,
    },
    Ky2Mode {
        control: Ky2Mode,
        callback: IntegerCallback<'a>,
    },
    Ky2Type {
        control: Ky2Type,
        callback: IntegerCallback<'a>,
    },
    Ky58CodeClear {
        control: Ky58CodeClear,
        callback: IntegerCallback<'a>,
    },
    Ky58CodeLoad {
        control: Ky58CodeLoad,
        callback: IntegerCallback<'a>,
    },
    LandingTaxiLights {
        control: LandingTaxiLights,
        callback: IntegerCallback<'a>,
    },
    LaunchFlareSalvo {
        control: LaunchFlareSalvo,
        callback: IntegerCallback<'a>,
    },
    LawLight {
        control: LawLight,
        callback: LedCallback<'a>,
    },
    LbsL1 {
        control: LbsL1,
        callback: IntegerCallback<'a>,
    },
    LbsL2 {
        control: LbsL2,
        callback: IntegerCallback<'a>,
    },
    LbsL3 {
        control: LbsL3,
        callback: IntegerCallback<'a>,
    },
    LbsL4 {
        control: LbsL4,
        callback: IntegerCallback<'a>,
    },
    LbsR1 {
        control: LbsR1,
        callback: IntegerCallback<'a>,
    },
    LbsR2 {
        control: LbsR2,
        callback: IntegerCallback<'a>,
    },
    LbsR3 {
        control: LbsR3,
        callback: IntegerCallback<'a>,
    },
    LbsR4 {
        control: LbsR4,
        callback: IntegerCallback<'a>,
    },
    LeftGearG {
        control: LeftGearG,
        callback: LedCallback<'a>,
    },
    LeftGearY {
        control: LeftGearY,
        callback: LedCallback<'a>,
    },
    LeftLight {
        control: LeftLight,
        callback: LedCallback<'a>,
    },
    LfuelLight {
        control: LfuelLight,
        callback: LedCallback<'a>,
    },
    LgDownLock {
        control: LgDownLock,
        callback: IntegerCallback<'a>,
    },
    LgEmergLever {
        control: LgEmergLever,
        callback: IntegerCallback<'a>,
    },
    LgEmergbattLever {
        control: LgEmergbattLever,
        callback: IntegerCallback<'a>,
    },
    LgLever {
        control: LgLever,
        callback: IntegerCallback<'a>,
    },
    LidsSw {
        control: LidsSw,
        callback: IntegerCallback<'a>,
    },
    LidsWlight {
        control: LidsWlight,
        callback: LedCallback<'a>,
    },
    Line2Wlight {
        control: Line2Wlight,
        callback: LedCallback<'a>,
    },
    LineWlight {
        control: LineWlight,
        callback: LedCallback<'a>,
    },
    LoadWlight {
        control: LoadWlight,
        callback: LedCallback<'a>,
    },
    LpumpWlight {
        control: LpumpWlight,
        callback: LedCallback<'a>,
    },
    LtankLight {
        control: LtankLight,
        callback: LedCallback<'a>,
    },
    LtransWlight {
        control: LtransWlight,
        callback: LedCallback<'a>,
    },
    MCaution {
        control: MCaution,
        callback: IntegerCallback<'a>,
    },
    MWarning {
        control: MWarning,
        callback: IntegerCallback<'a>,
    },
    MainGearG {
        control: MainGearG,
        callback: LedCallback<'a>,
    },
    MainGearY {
        control: MainGearY,
        callback: LedCallback<'a>,
    },
    ManFuelSw {
        control: ManFuelSw,
        callback: IntegerCallback<'a>,
    },
    MasterArm {
        control: MasterArm,
        callback: IntegerCallback<'a>,
    },
    McLight {
        control: McLight,
        callback: LedCallback<'a>,
    },
    MfsEmergLever {
        control: MfsEmergLever,
        callback: IntegerCallback<'a>,
    },
    MfsLight {
        control: MfsLight,
        callback: LedCallback<'a>,
    },
    MipLights {
        control: MipLights,
        callback: IntegerCallback<'a>,
    },
    MirrorToggleC {
        control: MirrorToggleC,
        callback: IntegerCallback<'a>,
    },
    MirrorToggleL {
        control: MirrorToggleL,
        callback: IntegerCallback<'a>,
    },
    MirrorToggleR {
        control: MirrorToggleR,
        callback: IntegerCallback<'a>,
    },
    MissPcMode {
        control: MissPcMode,
        callback: IntegerCallback<'a>,
    },
    ModeGauge {
        control: ModeGauge,
        callback: IntegerCallback<'a>,
    },
    MpcdL1 {
        control: MpcdL1,
        callback: IntegerCallback<'a>,
    },
    MpcdL10 {
        control: MpcdL10,
        callback: IntegerCallback<'a>,
    },
    MpcdL11 {
        control: MpcdL11,
        callback: IntegerCallback<'a>,
    },
    MpcdL12 {
        control: MpcdL12,
        callback: IntegerCallback<'a>,
    },
    MpcdL13 {
        control: MpcdL13,
        callback: IntegerCallback<'a>,
    },
    MpcdL14 {
        control: MpcdL14,
        callback: IntegerCallback<'a>,
    },
    MpcdL15 {
        control: MpcdL15,
        callback: IntegerCallback<'a>,
    },
    MpcdL16 {
        control: MpcdL16,
        callback: IntegerCallback<'a>,
    },
    MpcdL17 {
        control: MpcdL17,
        callback: IntegerCallback<'a>,
    },
    MpcdL18 {
        control: MpcdL18,
        callback: IntegerCallback<'a>,
    },
    MpcdL19 {
        control: MpcdL19,
        callback: IntegerCallback<'a>,
    },
    MpcdL2 {
        control: MpcdL2,
        callback: IntegerCallback<'a>,
    },
    MpcdL20 {
        control: MpcdL20,
        callback: IntegerCallback<'a>,
    },
    MpcdL3 {
        control: MpcdL3,
        callback: IntegerCallback<'a>,
    },
    MpcdL4 {
        control: MpcdL4,
        callback: IntegerCallback<'a>,
    },
    MpcdL5 {
        control: MpcdL5,
        callback: IntegerCallback<'a>,
    },
    MpcdL6 {
        control: MpcdL6,
        callback: IntegerCallback<'a>,
    },
    MpcdL7 {
        control: MpcdL7,
        callback: IntegerCallback<'a>,
    },
    MpcdL8 {
        control: MpcdL8,
        callback: IntegerCallback<'a>,
    },
    MpcdL9 {
        control: MpcdL9,
        callback: IntegerCallback<'a>,
    },
    MpcdLBright {
        control: MpcdLBright,
        callback: IntegerCallback<'a>,
    },
    MpcdLContrast {
        control: MpcdLContrast,
        callback: IntegerCallback<'a>,
    },
    MpcdLDayNight {
        control: MpcdLDayNight,
        callback: IntegerCallback<'a>,
    },
    MpcdLGain {
        control: MpcdLGain,
        callback: IntegerCallback<'a>,
    },
    MpcdLSymbol {
        control: MpcdLSymbol,
        callback: IntegerCallback<'a>,
    },
    MpcdR1 {
        control: MpcdR1,
        callback: IntegerCallback<'a>,
    },
    MpcdR10 {
        control: MpcdR10,
        callback: IntegerCallback<'a>,
    },
    MpcdR11 {
        control: MpcdR11,
        callback: IntegerCallback<'a>,
    },
    MpcdR12 {
        control: MpcdR12,
        callback: IntegerCallback<'a>,
    },
    MpcdR13 {
        control: MpcdR13,
        callback: IntegerCallback<'a>,
    },
    MpcdR14 {
        control: MpcdR14,
        callback: IntegerCallback<'a>,
    },
    MpcdR15 {
        control: MpcdR15,
        callback: IntegerCallback<'a>,
    },
    MpcdR16 {
        control: MpcdR16,
        callback: IntegerCallback<'a>,
    },
    MpcdR17 {
        control: MpcdR17,
        callback: IntegerCallback<'a>,
    },
    MpcdR18 {
        control: MpcdR18,
        callback: IntegerCallback<'a>,
    },
    MpcdR19 {
        control: MpcdR19,
        callback: IntegerCallback<'a>,
    },
    MpcdR2 {
        control: MpcdR2,
        callback: IntegerCallback<'a>,
    },
    MpcdR20 {
        control: MpcdR20,
        callback: IntegerCallback<'a>,
    },
    MpcdR3 {
        control: MpcdR3,
        callback: IntegerCallback<'a>,
    },
    MpcdR4 {
        control: MpcdR4,
        callback: IntegerCallback<'a>,
    },
    MpcdR5 {
        control: MpcdR5,
        callback: IntegerCallback<'a>,
    },
    MpcdR6 {
        control: MpcdR6,
        callback: IntegerCallback<'a>,
    },
    MpcdR7 {
        control: MpcdR7,
        callback: IntegerCallback<'a>,
    },
    MpcdR8 {
        control: MpcdR8,
        callback: IntegerCallback<'a>,
    },
    MpcdR9 {
        control: MpcdR9,
        callback: IntegerCallback<'a>,
    },
    MpcdRBright {
        control: MpcdRBright,
        callback: IntegerCallback<'a>,
    },
    MpcdRContrast {
        control: MpcdRContrast,
        callback: IntegerCallback<'a>,
    },
    MpcdRDayNight {
        control: MpcdRDayNight,
        callback: IntegerCallback<'a>,
    },
    MpcdRGain {
        control: MpcdRGain,
        callback: IntegerCallback<'a>,
    },
    MpcdRSymbol {
        control: MpcdRSymbol,
        callback: IntegerCallback<'a>,
    },
    MultGauge {
        control: MultGauge,
        callback: IntegerCallback<'a>,
    },
    MultiReleaseControl {
        control: MultiReleaseControl,
        callback: IntegerCallback<'a>,
    },
    MwLight {
        control: MwLight,
        callback: LedCallback<'a>,
    },
    NavCrs {
        control: NavCrs,
        callback: IntegerCallback<'a>,
    },
    NavLight {
        control: NavLight,
        callback: LedCallback<'a>,
    },
    NavSel {
        control: NavSel,
        callback: IntegerCallback<'a>,
    },
    NoseGearG {
        control: NoseGearG,
        callback: LedCallback<'a>,
    },
    NoseGearY {
        control: NoseGearY,
        callback: LedCallback<'a>,
    },
    NozzleFriction {
        control: NozzleFriction,
        callback: IntegerCallback<'a>,
    },
    NozzleLever {
        control: NozzleLever,
        callback: IntegerCallback<'a>,
    },
    NozzlePointer {
        control: NozzlePointer,
        callback: IntegerCallback<'a>,
    },
    NwsWlight {
        control: NwsWlight,
        callback: LedCallback<'a>,
    },
    O2Sw {
        control: O2Sw,
        callback: IntegerCallback<'a>,
    },
    OduOpt1 {
        control: OduOpt1,
        callback: IntegerCallback<'a>,
    },
    OduOpt2 {
        control: OduOpt2,
        callback: IntegerCallback<'a>,
    },
    OduOpt3 {
        control: OduOpt3,
        callback: IntegerCallback<'a>,
    },
    OduOpt4 {
        control: OduOpt4,
        callback: IntegerCallback<'a>,
    },
    OduOpt5 {
        control: OduOpt5,
        callback: IntegerCallback<'a>,
    },
    OilWlight {
        control: OilWlight,
        callback: LedCallback<'a>,
    },
    OtLight {
        control: OtLight,
        callback: LedCallback<'a>,
    },
    OxyWlight {
        control: OxyWlight,
        callback: LedCallback<'a>,
    },
    PJamWlight {
        control: PJamWlight,
        callback: LedCallback<'a>,
    },
    PNogoWlight {
        control: PNogoWlight,
        callback: LedCallback<'a>,
    },
    ParkBrakeLv {
        control: ParkBrakeLv,
        callback: IntegerCallback<'a>,
    },
    PitchWlight {
        control: PitchWlight,
        callback: LedCallback<'a>,
    },
    PosLights {
        control: PosLights,
        callback: IntegerCallback<'a>,
    },
    Probe {
        control: Probe,
        callback: IntegerCallback<'a>,
    },
    ProbeHeatMode {
        control: ProbeHeatMode,
        callback: IntegerCallback<'a>,
    },
    PropWlight {
        control: PropWlight,
        callback: LedCallback<'a>,
    },
    QFeelSw {
        control: QFeelSw,
        callback: IntegerCallback<'a>,
    },
    Qty1 {
        control: Qty1,
        callback: IntegerCallback<'a>,
    },
    Qty2 {
        control: Qty2,
        callback: IntegerCallback<'a>,
    },
    QuantSel01 {
        control: QuantSel01,
        callback: IntegerCallback<'a>,
    },
    QuantSel10 {
        control: QuantSel10,
        callback: IntegerCallback<'a>,
    },
    ReadyLight {
        control: ReadyLight,
        callback: LedCallback<'a>,
    },
    RecordSysDisplay {
        control: RecordSysDisplay,
        callback: IntegerCallback<'a>,
    },
    RecordSysMode {
        control: RecordSysMode,
        callback: IntegerCallback<'a>,
    },
    ReleaseControl {
        control: ReleaseControl,
        callback: IntegerCallback<'a>,
    },
    ReleaseInt001 {
        control: ReleaseInt001,
        callback: IntegerCallback<'a>,
    },
    ReleaseInt010 {
        control: ReleaseInt010,
        callback: IntegerCallback<'a>,
    },
    ReleaseInt100 {
        control: ReleaseInt100,
        callback: IntegerCallback<'a>,
    },
    ReplyWlight {
        control: ReplyWlight,
        callback: LedCallback<'a>,
    },
    RfuelLight {
        control: RfuelLight,
        callback: LedCallback<'a>,
    },
    RightGearG {
        control: RightGearG,
        callback: LedCallback<'a>,
    },
    RightGearY {
        control: RightGearY,
        callback: LedCallback<'a>,
    },
    RightLight {
        control: RightLight,
        callback: LedCallback<'a>,
    },
    RollWlight {
        control: RollWlight,
        callback: LedCallback<'a>,
    },
    Rpm1 {
        control: Rpm1,
        callback: IntegerCallback<'a>,
    },
    Rpm2 {
        control: Rpm2,
        callback: IntegerCallback<'a>,
    },
    Rpm3 {
        control: Rpm3,
        callback: IntegerCallback<'a>,
    },
    Rpm4 {
        control: Rpm4,
        callback: IntegerCallback<'a>,
    },
    RpmSw {
        control: RpmSw,
        callback: IntegerCallback<'a>,
    },
    RpumpWlight {
        control: RpumpWlight,
        callback: LedCallback<'a>,
    },
    RtankLight {
        control: RtankLight,
        callback: LedCallback<'a>,
    },
    RtransWlight {
        control: RtransWlight,
        callback: LedCallback<'a>,
    },
    RudderNeedle {
        control: RudderNeedle,
        callback: IntegerCallback<'a>,
    },
    RudderTrim {
        control: RudderTrim,
        callback: IntegerCallback<'a>,
    },
    RwrVol {
        control: RwrVol,
        callback: IntegerCallback<'a>,
    },
    SamLight {
        control: SamLight,
        callback: LedCallback<'a>,
    },
    SasPitch {
        control: SasPitch,
        callback: IntegerCallback<'a>,
    },
    SasRoll {
        control: SasRoll,
        callback: IntegerCallback<'a>,
    },
    SasYaw {
        control: SasYaw,
        callback: IntegerCallback<'a>,
    },
    SeatAdj {
        control: SeatAdj,
        callback: IntegerCallback<'a>,
    },
    SeatSafeLever {
        control: SeatSafeLever,
        callback: IntegerCallback<'a>,
    },
    SelLight {
        control: SelLight,
        callback: LedCallback<'a>,
    },
    SetFlag1 {
        control: SetFlag1,
        callback: IntegerCallback<'a>,
    },
    SetFlag2 {
        control: SetFlag2,
        callback: IntegerCallback<'a>,
    },
    SetFlag3 {
        control: SetFlag3,
        callback: IntegerCallback<'a>,
    },
    SetFlag4 {
        control: SetFlag4,
        callback: IntegerCallback<'a>,
    },
    SetFlag5 {
        control: SetFlag5,
        callback: IntegerCallback<'a>,
    },
    SetFlag6 {
        control: SetFlag6,
        callback: IntegerCallback<'a>,
    },
    SetFlag7 {
        control: SetFlag7,
        callback: IntegerCallback<'a>,
    },
    SidepanelLights {
        control: SidepanelLights,
        callback: IntegerCallback<'a>,
    },
    SkidWlight {
        control: SkidWlight,
        callback: LedCallback<'a>,
    },
    SlipInd {
        control: SlipInd,
        callback: IntegerCallback<'a>,
    },
    SlipTurnFlag {
        control: SlipTurnFlag,
        callback: IntegerCallback<'a>,
    },
    SpdBrkWlight {
        control: SpdBrkWlight,
        callback: LedCallback<'a>,
    },
    Stab1 {
        control: Stab1,
        callback: IntegerCallback<'a>,
    },
    Stab2 {
        control: Stab2,
        callback: IntegerCallback<'a>,
    },
    StabArrow {
        control: StabArrow,
        callback: IntegerCallback<'a>,
    },
    StbyTrWlight {
        control: StbyTrWlight,
        callback: LedCallback<'a>,
    },
    StoLever {
        control: StoLever,
        callback: IntegerCallback<'a>,
    },
    StoLight {
        control: StoLight,
        callback: LedCallback<'a>,
    },
    StopwatchLap {
        control: StopwatchLap,
        callback: IntegerCallback<'a>,
    },
    StopwatchStart {
        control: StopwatchStart,
        callback: IntegerCallback<'a>,
    },
    TempControl {
        control: TempControl,
        callback: IntegerCallback<'a>,
    },
    ThrottCutoffLv {
        control: ThrottCutoffLv,
        callback: IntegerCallback<'a>,
    },
    ThrottleFriction {
        control: ThrottleFriction,
        callback: IntegerCallback<'a>,
    },
    TotFuel1 {
        control: TotFuel1,
        callback: IntegerCallback<'a>,
    },
    TotFuel2 {
        control: TotFuel2,
        callback: IntegerCallback<'a>,
    },
    TotFuel3 {
        control: TotFuel3,
        callback: IntegerCallback<'a>,
    },
    TotFuel4 {
        control: TotFuel4,
        callback: IntegerCallback<'a>,
    },
    TotFuel5 {
        control: TotFuel5,
        callback: IntegerCallback<'a>,
    },
    TrimMode {
        control: TrimMode,
        callback: IntegerCallback<'a>,
    },
    TurnIndi {
        control: TurnIndi,
        callback: IntegerCallback<'a>,
    },
    UfcAlt {
        control: UfcAlt,
        callback: IntegerCallback<'a>,
    },
    UfcB0 {
        control: UfcB0,
        callback: IntegerCallback<'a>,
    },
    UfcB1 {
        control: UfcB1,
        callback: IntegerCallback<'a>,
    },
    UfcB2 {
        control: UfcB2,
        callback: IntegerCallback<'a>,
    },
    UfcB3 {
        control: UfcB3,
        callback: IntegerCallback<'a>,
    },
    UfcB4 {
        control: UfcB4,
        callback: IntegerCallback<'a>,
    },
    UfcB5 {
        control: UfcB5,
        callback: IntegerCallback<'a>,
    },
    UfcB6 {
        control: UfcB6,
        callback: IntegerCallback<'a>,
    },
    UfcB7 {
        control: UfcB7,
        callback: IntegerCallback<'a>,
    },
    UfcB8 {
        control: UfcB8,
        callback: IntegerCallback<'a>,
    },
    UfcB9 {
        control: UfcB9,
        callback: IntegerCallback<'a>,
    },
    UfcBeacon {
        control: UfcBeacon,
        callback: IntegerCallback<'a>,
    },
    UfcBright {
        control: UfcBright,
        callback: IntegerCallback<'a>,
    },
    UfcClear {
        control: UfcClear,
        callback: IntegerCallback<'a>,
    },
    UfcCom1Pull {
        control: UfcCom1Pull,
        callback: IntegerCallback<'a>,
    },
    UfcCom1Sel {
        control: UfcCom1Sel,
        callback: IntegerCallback<'a>,
    },
    UfcCom1Vol {
        control: UfcCom1Vol,
        callback: IntegerCallback<'a>,
    },
    UfcCom2Pull {
        control: UfcCom2Pull,
        callback: IntegerCallback<'a>,
    },
    UfcCom2Sel {
        control: UfcCom2Sel,
        callback: IntegerCallback<'a>,
    },
    UfcCom2Vol {
        control: UfcCom2Vol,
        callback: IntegerCallback<'a>,
    },
    UfcComm1Display {
        control: UfcComm1Display,
        callback: DisplayCallback<'a>,
    },
    UfcComm2Display {
        control: UfcComm2Display,
        callback: DisplayCallback<'a>,
    },
    UfcDash {
        control: UfcDash,
        callback: IntegerCallback<'a>,
    },
    UfcDot {
        control: UfcDot,
        callback: IntegerCallback<'a>,
    },
    UfcEmcom {
        control: UfcEmcom,
        callback: IntegerCallback<'a>,
    },
    UfcEnter {
        control: UfcEnter,
        callback: IntegerCallback<'a>,
    },
    UfcIff {
        control: UfcIff,
        callback: IntegerCallback<'a>,
    },
    UfcIp {
        control: UfcIp,
        callback: IntegerCallback<'a>,
    },
    UfcOnoff {
        control: UfcOnoff,
        callback: IntegerCallback<'a>,
    },
    UfcSave {
        control: UfcSave,
        callback: IntegerCallback<'a>,
    },
    UfcScratchpad {
        control: UfcScratchpad,
        callback: DisplayCallback<'a>,
    },
    UfcTacan {
        control: UfcTacan,
        callback: IntegerCallback<'a>,
    },
    UfcTarget {
        control: UfcTarget,
        callback: IntegerCallback<'a>,
    },
    UfcTimer {
        control: UfcTimer,
        callback: IntegerCallback<'a>,
    },
    UfcWay {
        control: UfcWay,
        callback: IntegerCallback<'a>,
    },
    UfcWeapon {
        control: UfcWeapon,
        callback: IntegerCallback<'a>,
    },
    UfcWeather {
        control: UfcWeather,
        callback: IntegerCallback<'a>,
    },
    UhfAncPoint {
        control: UhfAncPoint,
        callback: IntegerCallback<'a>,
    },
    UhfAncSw {
        control: UhfAncSw,
        callback: IntegerCallback<'a>,
    },
    UhfFreq {
        control: UhfFreq,
        callback: IntegerCallback<'a>,
    },
    UhfLoadSw {
        control: UhfLoadSw,
        callback: IntegerCallback<'a>,
    },
    UhfMode {
        control: UhfMode,
        callback: IntegerCallback<'a>,
    },
    UhfVol {
        control: UhfVol,
        callback: IntegerCallback<'a>,
    },
    VoltMeter {
        control: VoltMeter,
        callback: IntegerCallback<'a>,
    },
    VstolLight {
        control: VstolLight,
        callback: LedCallback<'a>,
    },
    VstolSel {
        control: VstolSel,
        callback: IntegerCallback<'a>,
    },
    Vvi {
        control: Vvi,
        callback: IntegerCallback<'a>,
    },
    WsldWlight {
        control: WsldWlight,
        callback: LedCallback<'a>,
    },
    YawTrimSw {
        control: YawTrimSw,
        callback: IntegerCallback<'a>,
    },
    YawWlight {
        control: YawWlight,
        callback: LedCallback<'a>,
    },
}

impl RegisteredControl<'_> {
    fn accept(&mut self, packet: Packet<'_>) {
        match self {
            Self::AaaLight { control, .. } => control.accept(packet),
            Self::AcnipMode { control, .. } => control.accept(packet),
            Self::AcnipRadioSel { control, .. } => control.accept(packet),
            Self::AdiBank { control, .. } => control.accept(packet),
            Self::AdiFlag { control, .. } => control.accept(packet),
            Self::AdiPitch { control, .. } => control.accept(packet),
            Self::AfcSw { control, .. } => control.accept(packet),
            Self::AfcWlight { control, .. } => control.accept(packet),
            Self::AftBayWlight { control, .. } => control.accept(packet),
            Self::AgLight { control, .. } => control.accept(packet),
            Self::AgSel { control, .. } => control.accept(packet),
            Self::AiLight { control, .. } => control.accept(packet),
            Self::AileronNeedle { control, .. } => control.accept(packet),
            Self::Alt10000ftCnt { control, .. } => control.accept(packet),
            Self::Alt1000ftCnt { control, .. } => control.accept(packet),
            Self::Alt100ftCnt { control, .. } => control.accept(packet),
            Self::AltHoldSw { control, .. } => control.accept(packet),
            Self::AltPointer { control, .. } => control.accept(packet),
            Self::AltPressure1 { control, .. } => control.accept(packet),
            Self::AltPressure2 { control, .. } => control.accept(packet),
            Self::AltPressure3 { control, .. } => control.accept(packet),
            Self::AltPressure4 { control, .. } => control.accept(packet),
            Self::AnnunciatorLights { control, .. } => control.accept(packet),
            Self::AntiCollLights { control, .. } => control.accept(packet),
            Self::AntiSkid { control, .. } => control.accept(packet),
            Self::AoaFlag { control, .. } => control.accept(packet),
            Self::AoaPointer { control, .. } => control.accept(packet),
            Self::ApuGenSw { control, .. } => control.accept(packet),
            Self::ApuGenWlight { control, .. } => control.accept(packet),
            Self::ApuWlight { control, .. } => control.accept(packet),
            Self::ArmControl { control, .. } => control.accept(packet),
            Self::AutflpWlight { control, .. } => control.accept(packet),
            Self::Av8bnaOdu1Select { control, .. } => control.accept(packet),
            Self::Av8bnaOdu1Text { control, .. } => control.accept(packet),
            Self::Av8bnaOdu2Select { control, .. } => control.accept(packet),
            Self::Av8bnaOdu2Text { control, .. } => control.accept(packet),
            Self::Av8bnaOdu3Select { control, .. } => control.accept(packet),
            Self::Av8bnaOdu3Text { control, .. } => control.accept(packet),
            Self::Av8bnaOdu4Select { control, .. } => control.accept(packet),
            Self::Av8bnaOdu4Text { control, .. } => control.accept(packet),
            Self::Av8bnaOdu5Select { control, .. } => control.accept(packet),
            Self::Av8bnaOdu5Text { control, .. } => control.accept(packet),
            Self::AvBitWlight { control, .. } => control.accept(packet),
            Self::BakAdiCageKnob { control, .. } => control.accept(packet),
            Self::BakAdiCagePull { control, .. } => control.accept(packet),
            Self::BaroPressure { control, .. } => control.accept(packet),
            Self::BattSw { control, .. } => control.accept(packet),
            Self::Bingo1 { control, .. } => control.accept(packet),
            Self::Bingo2 { control, .. } => control.accept(packet),
            Self::Bingo3 { control, .. } => control.accept(packet),
            Self::Bingo4 { control, .. } => control.accept(packet),
            Self::BingoLight { control, .. } => control.accept(packet),
            Self::BingoSet { control, .. } => control.accept(packet),
            Self::BombsArmSw { control, .. } => control.accept(packet),
            Self::Brake1 { control, .. } => control.accept(packet),
            Self::Brake2 { control, .. } => control.accept(packet),
            Self::Brake3 { control, .. } => control.accept(packet),
            Self::BrakeAccu { control, .. } => control.accept(packet),
            Self::CAut { control, .. } => control.accept(packet),
            Self::CabinDefog { control, .. } => control.accept(packet),
            Self::CabinPress { control, .. } => control.accept(packet),
            Self::CabinPressSwitch { control, .. } => control.accept(packet),
            Self::CanopyHandL { control, .. } => control.accept(packet),
            Self::CanopyHandR { control, .. } => control.accept(packet),
            Self::CanopyLock { control, .. } => control.accept(packet),
            Self::CanopyValue { control, .. } => control.accept(packet),
            Self::CanopyWlight { control, .. } => control.accept(packet),
            Self::ClockHh { control, .. } => control.accept(packet),
            Self::ClockMm { control, .. } => control.accept(packet),
            Self::ClockSs { control, .. } => control.accept(packet),
            Self::Cmbt { control, .. } => control.accept(packet),
            Self::CmbtLight { control, .. } => control.accept(packet),
            Self::Comm1StringFreq { control, .. } => control.accept(packet),
            Self::Comm2StringFreq { control, .. } => control.accept(packet),
            Self::CompLight { control, .. } => control.accept(packet),
            Self::CompassBank { control, .. } => control.accept(packet),
            Self::CompassLights { control, .. } => control.accept(packet),
            Self::CompassNs { control, .. } => control.accept(packet),
            Self::CompassUd { control, .. } => control.accept(packet),
            Self::ConsoleLights { control, .. } => control.accept(packet),
            Self::Control15sekLight { control, .. } => control.accept(packet),
            Self::CsCoolWlight { control, .. } => control.accept(packet),
            Self::CwJamWlight { control, .. } => control.accept(packet),
            Self::CwLight { control, .. } => control.accept(packet),
            Self::CwNogoWlight { control, .. } => control.accept(packet),
            Self::DcTestSw { control, .. } => control.accept(packet),
            Self::DcWlight { control, .. } => control.accept(packet),
            Self::DecoyControl { control, .. } => control.accept(packet),
            Self::DecsSw { control, .. } => control.accept(packet),
            Self::DepResWlight { control, .. } => control.accept(packet),
            Self::Dmt { control, .. } => control.accept(packet),
            Self::DroopWlight { control, .. } => control.accept(packet),
            Self::DualCpuMode { control, .. } => control.accept(packet),
            Self::Duct1 { control, .. } => control.accept(packet),
            Self::Duct2 { control, .. } => control.accept(packet),
            Self::Duct3 { control, .. } => control.accept(packet),
            Self::EcsAft { control, .. } => control.accept(packet),
            Self::EcsFwd { control, .. } => control.accept(packet),
            Self::EdpBit { control, .. } => control.accept(packet),
            Self::EdpBright { control, .. } => control.accept(packet),
            Self::EfcLight { control, .. } => control.accept(packet),
            Self::EfcSw { control, .. } => control.accept(packet),
            Self::EfcWlight { control, .. } => control.accept(packet),
            Self::EmergJett { control, .. } => control.accept(packet),
            Self::EmsB { control, .. } => control.accept(packet),
            Self::EngExcWlight { control, .. } => control.accept(packet),
            Self::EngStartSw { control, .. } => control.accept(packet),
            Self::ExtAuxLights { control, .. } => control.accept(packet),
            Self::ExtFormationLights { control, .. } => control.accept(packet),
            Self::ExtLights { control, .. } => control.accept(packet),
            Self::ExtPositionLights { control, .. } => control.accept(packet),
            Self::ExtSpeedBrake { control, .. } => control.accept(packet),
            Self::ExtStrobeTail { control, .. } => control.accept(packet),
            Self::ExtWowLeft { control, .. } => control.accept(packet),
            Self::ExtWowNose { control, .. } => control.accept(packet),
            Self::ExtWowRight { control, .. } => control.accept(packet),
            Self::ExtWowTail { control, .. } => control.accept(packet),
            Self::Ff1 { control, .. } => control.accept(packet),
            Self::Ff2 { control, .. } => control.accept(packet),
            Self::Ff3 { control, .. } => control.accept(packet),
            Self::FireLight { control, .. } => control.accept(packet),
            Self::FlapMode { control, .. } => control.accept(packet),
            Self::FlapPos1 { control, .. } => control.accept(packet),
            Self::FlapPos2 { control, .. } => control.accept(packet),
            Self::FlapPower { control, .. } => control.accept(packet),
            Self::Flaps1Wlight { control, .. } => control.accept(packet),
            Self::Flaps2Wlight { control, .. } => control.accept(packet),
            Self::FlapsBit { control, .. } => control.accept(packet),
            Self::FlapsLight { control, .. } => control.accept(packet),
            Self::Flir { control, .. } => control.accept(packet),
            Self::FloodLBhAftB { control, .. } => control.accept(packet),
            Self::FloodLBhAftF { control, .. } => control.accept(packet),
            Self::FloodLBhFwd { control, .. } => control.accept(packet),
            Self::FloodLCanB { control, .. } => control.accept(packet),
            Self::FloodLCanT { control, .. } => control.accept(packet),
            Self::FloodLights { control, .. } => control.accept(packet),
            Self::FloodLightsDial { control, .. } => control.accept(packet),
            Self::FloodRBhAftB { control, .. } => control.accept(packet),
            Self::FloodRBhAftF { control, .. } => control.accept(packet),
            Self::FloodRBhFwd { control, .. } => control.accept(packet),
            Self::FloodRCanB { control, .. } => control.accept(packet),
            Self::FloodRCanT { control, .. } => control.accept(packet),
            Self::FormationLights { control, .. } => control.accept(packet),
            Self::FuelDumpL { control, .. } => control.accept(packet),
            Self::FuelDumpR { control, .. } => control.accept(packet),
            Self::FuelFlag { control, .. } => control.accept(packet),
            Self::FuelProp { control, .. } => control.accept(packet),
            Self::FuelPumpL { control, .. } => control.accept(packet),
            Self::FuelPumpR { control, .. } => control.accept(packet),
            Self::FuelSel { control, .. } => control.accept(packet),
            Self::FuelShutoff { control, .. } => control.accept(packet),
            Self::Fuz1 { control, .. } => control.accept(packet),
            Self::Fuz2 { control, .. } => control.accept(packet),
            Self::FuzeControl { control, .. } => control.accept(packet),
            Self::GearLeverLight { control, .. } => control.accept(packet),
            Self::GearLight { control, .. } => control.accept(packet),
            Self::GenLight { control, .. } => control.accept(packet),
            Self::GenSw { control, .. } => control.accept(packet),
            Self::GndIrCool { control, .. } => control.accept(packet),
            Self::GpsWlight { control, .. } => control.accept(packet),
            Self::H2o1 { control, .. } => control.accept(packet),
            Self::H2o2 { control, .. } => control.accept(packet),
            Self::H2oControlLight { control, .. } => control.accept(packet),
            Self::H2oDump { control, .. } => control.accept(packet),
            Self::H2oLight { control, .. } => control.accept(packet),
            Self::H2oMode { control, .. } => control.accept(packet),
            Self::H2oSel { control, .. } => control.accept(packet),
            Self::HhydLight { control, .. } => control.accept(packet),
            Self::HideStick { control, .. } => control.accept(packet),
            Self::HudAlt { control, .. } => control.accept(packet),
            Self::HudBright { control, .. } => control.accept(packet),
            Self::HudMode { control, .. } => control.accept(packet),
            Self::HudSymbol { control, .. } => control.accept(packet),
            Self::HudVideoBright { control, .. } => control.accept(packet),
            Self::HudVideoContr { control, .. } => control.accept(packet),
            Self::Hyd11 { control, .. } => control.accept(packet),
            Self::Hyd12 { control, .. } => control.accept(packet),
            Self::Hyd13 { control, .. } => control.accept(packet),
            Self::Hyd1Wlight { control, .. } => control.accept(packet),
            Self::Hyd21 { control, .. } => control.accept(packet),
            Self::Hyd22 { control, .. } => control.accept(packet),
            Self::Hyd23 { control, .. } => control.accept(packet),
            Self::Hyd2Wlight { control, .. } => control.accept(packet),
            Self::IcsAuxVol { control, .. } => control.accept(packet),
            Self::IcsGndVol { control, .. } => control.accept(packet),
            Self::IcsMicMode { control, .. } => control.accept(packet),
            Self::IffCryptMode { control, .. } => control.accept(packet),
            Self::IffOperMode { control, .. } => control.accept(packet),
            Self::IffWlight { control, .. } => control.accept(packet),
            Self::InsMode { control, .. } => control.accept(packet),
            Self::InsWlight { control, .. } => control.accept(packet),
            Self::InstLights { control, .. } => control.accept(packet),
            Self::Invt1 { control, .. } => control.accept(packet),
            Self::Invt2 { control, .. } => control.accept(packet),
            Self::Invt3 { control, .. } => control.accept(packet),
            Self::JamHotWlight { control, .. } => control.accept(packet),
            Self::JammerControl { control, .. } => control.accept(packet),
            Self::JettMode { control, .. } => control.accept(packet),
            Self::JettStation1 { control, .. } => control.accept(packet),
            Self::JettStation1Light { control, .. } => control.accept(packet),
            Self::JettStation2 { control, .. } => control.accept(packet),
            Self::JettStation2Light { control, .. } => control.accept(packet),
            Self::JettStation3 { control, .. } => control.accept(packet),
            Self::JettStation3Light { control, .. } => control.accept(packet),
            Self::JettStation4 { control, .. } => control.accept(packet),
            Self::JettStation4Light { control, .. } => control.accept(packet),
            Self::JettStation5 { control, .. } => control.accept(packet),
            Self::JettStation5Light { control, .. } => control.accept(packet),
            Self::JettStation6 { control, .. } => control.accept(packet),
            Self::JettStation6Light { control, .. } => control.accept(packet),
            Self::JettStation7 { control, .. } => control.accept(packet),
            Self::JettStation7Light { control, .. } => control.accept(packet),
            Self::JettStores { control, .. } => control.accept(packet),
            Self::Jpt1 { control, .. } => control.accept(packet),
            Self::Jpt2 { control, .. } => control.accept(packet),
            Self::Jpt3 { control, .. } => control.accept(packet),
            Self::JptlLight { control, .. } => control.accept(packet),
            Self::JptlSw { control, .. } => control.accept(packet),
            Self::KntPointerL { control, .. } => control.accept(packet),
            Self::KntPointerS { control, .. } => control.accept(packet),
            Self::Ky1Mode { control, .. } => control.accept(packet),
            Self::Ky1Type { control, .. } => control.accept(packet),
            Self::Ky2Mode { control, .. } => control.accept(packet),
            Self::Ky2Type { control, .. } => control.accept(packet),
            Self::Ky58CodeClear { control, .. } => control.accept(packet),
            Self::Ky58CodeLoad { control, .. } => control.accept(packet),
            Self::LandingTaxiLights { control, .. } => control.accept(packet),
            Self::LaunchFlareSalvo { control, .. } => control.accept(packet),
            Self::LawLight { control, .. } => control.accept(packet),
            Self::LbsL1 { control, .. } => control.accept(packet),
            Self::LbsL2 { control, .. } => control.accept(packet),
            Self::LbsL3 { control, .. } => control.accept(packet),
            Self::LbsL4 { control, .. } => control.accept(packet),
            Self::LbsR1 { control, .. } => control.accept(packet),
            Self::LbsR2 { control, .. } => control.accept(packet),
            Self::LbsR3 { control, .. } => control.accept(packet),
            Self::LbsR4 { control, .. } => control.accept(packet),
            Self::LeftGearG { control, .. } => control.accept(packet),
            Self::LeftGearY { control, .. } => control.accept(packet),
            Self::LeftLight { control, .. } => control.accept(packet),
            Self::LfuelLight { control, .. } => control.accept(packet),
            Self::LgDownLock { control, .. } => control.accept(packet),
            Self::LgEmergLever { control, .. } => control.accept(packet),
            Self::LgEmergbattLever { control, .. } => control.accept(packet),
            Self::LgLever { control, .. } => control.accept(packet),
            Self::LidsSw { control, .. } => control.accept(packet),
            Self::LidsWlight { control, .. } => control.accept(packet),
            Self::Line2Wlight { control, .. } => control.accept(packet),
            Self::LineWlight { control, .. } => control.accept(packet),
            Self::LoadWlight { control, .. } => control.accept(packet),
            Self::LpumpWlight { control, .. } => control.accept(packet),
            Self::LtankLight { control, .. } => control.accept(packet),
            Self::LtransWlight { control, .. } => control.accept(packet),
            Self::MCaution { control, .. } => control.accept(packet),
            Self::MWarning { control, .. } => control.accept(packet),
            Self::MainGearG { control, .. } => control.accept(packet),
            Self::MainGearY { control, .. } => control.accept(packet),
            Self::ManFuelSw { control, .. } => control.accept(packet),
            Self::MasterArm { control, .. } => control.accept(packet),
            Self::McLight { control, .. } => control.accept(packet),
            Self::MfsEmergLever { control, .. } => control.accept(packet),
            Self::MfsLight { control, .. } => control.accept(packet),
            Self::MipLights { control, .. } => control.accept(packet),
            Self::MirrorToggleC { control, .. } => control.accept(packet),
            Self::MirrorToggleL { control, .. } => control.accept(packet),
            Self::MirrorToggleR { control, .. } => control.accept(packet),
            Self::MissPcMode { control, .. } => control.accept(packet),
            Self::ModeGauge { control, .. } => control.accept(packet),
            Self::MpcdL1 { control, .. } => control.accept(packet),
            Self::MpcdL10 { control, .. } => control.accept(packet),
            Self::MpcdL11 { control, .. } => control.accept(packet),
            Self::MpcdL12 { control, .. } => control.accept(packet),
            Self::MpcdL13 { control, .. } => control.accept(packet),
            Self::MpcdL14 { control, .. } => control.accept(packet),
            Self::MpcdL15 { control, .. } => control.accept(packet),
            Self::MpcdL16 { control, .. } => control.accept(packet),
            Self::MpcdL17 { control, .. } => control.accept(packet),
            Self::MpcdL18 { control, .. } => control.accept(packet),
            Self::MpcdL19 { control, .. } => control.accept(packet),
            Self::MpcdL2 { control, .. } => control.accept(packet),
            Self::MpcdL20 { control, .. } => control.accept(packet),
            Self::MpcdL3 { control, .. } => control.accept(packet),
            Self::MpcdL4 { control, .. } => control.accept(packet),
            Self::MpcdL5 { control, .. } => control.accept(packet),
            Self::MpcdL6 { control, .. } => control.accept(packet),
            Self::MpcdL7 { control, .. } => control.accept(packet),
            Self::MpcdL8 { control, .. } => control.accept(packet),
            Self::MpcdL9 { control, .. } => control.accept(packet),
            Self::MpcdLBright { control, .. } => control.accept(packet),
            Self::MpcdLContrast { control, .. } => control.accept(packet),
            Self::MpcdLDayNight { control, .. } => control.accept(packet),
            Self::MpcdLGain { control, .. } => control.accept(packet),
            Self::MpcdLSymbol { control, .. } => control.accept(packet),
            Self::MpcdR1 { control, .. } => control.accept(packet),
            Self::MpcdR10 { control, .. } => control.accept(packet),
            Self::MpcdR11 { control, .. } => control.accept(packet),
            Self::MpcdR12 { control, .. } => control.accept(packet),
            Self::MpcdR13 { control, .. } => control.accept(packet),
            Self::MpcdR14 { control, .. } => control.accept(packet),
            Self::MpcdR15 { control, .. } => control.accept(packet),
            Self::MpcdR16 { control, .. } => control.accept(packet),
            Self::MpcdR17 { control, .. } => control.accept(packet),
            Self::MpcdR18 { control, .. } => control.accept(packet),
            Self::MpcdR19 { control, .. } => control.accept(packet),
            Self::MpcdR2 { control, .. } => control.accept(packet),
            Self::MpcdR20 { control, .. } => control.accept(packet),
            Self::MpcdR3 { control, .. } => control.accept(packet),
            Self::MpcdR4 { control, .. } => control.accept(packet),
            Self::MpcdR5 { control, .. } => control.accept(packet),
            Self::MpcdR6 { control, .. } => control.accept(packet),
            Self::MpcdR7 { control, .. } => control.accept(packet),
            Self::MpcdR8 { control, .. } => control.accept(packet),
            Self::MpcdR9 { control, .. } => control.accept(packet),
            Self::MpcdRBright { control, .. } => control.accept(packet),
            Self::MpcdRContrast { control, .. } => control.accept(packet),
            Self::MpcdRDayNight { control, .. } => control.accept(packet),
            Self::MpcdRGain { control, .. } => control.accept(packet),
            Self::MpcdRSymbol { control, .. } => control.accept(packet),
            Self::MultGauge { control, .. } => control.accept(packet),
            Self::MultiReleaseControl { control, .. } => control.accept(packet),
            Self::MwLight { control, .. } => control.accept(packet),
            Self::NavCrs { control, .. } => control.accept(packet),
            Self::NavLight { control, .. } => control.accept(packet),
            Self::NavSel { control, .. } => control.accept(packet),
            Self::NoseGearG { control, .. } => control.accept(packet),
            Self::NoseGearY { control, .. } => control.accept(packet),
            Self::NozzleFriction { control, .. } => control.accept(packet),
            Self::NozzleLever { control, .. } => control.accept(packet),
            Self::NozzlePointer { control, .. } => control.accept(packet),
            Self::NwsWlight { control, .. } => control.accept(packet),
            Self::O2Sw { control, .. } => control.accept(packet),
            Self::OduOpt1 { control, .. } => control.accept(packet),
            Self::OduOpt2 { control, .. } => control.accept(packet),
            Self::OduOpt3 { control, .. } => control.accept(packet),
            Self::OduOpt4 { control, .. } => control.accept(packet),
            Self::OduOpt5 { control, .. } => control.accept(packet),
            Self::OilWlight { control, .. } => control.accept(packet),
            Self::OtLight { control, .. } => control.accept(packet),
            Self::OxyWlight { control, .. } => control.accept(packet),
            Self::PJamWlight { control, .. } => control.accept(packet),
            Self::PNogoWlight { control, .. } => control.accept(packet),
            Self::ParkBrakeLv { control, .. } => control.accept(packet),
            Self::PitchWlight { control, .. } => control.accept(packet),
            Self::PosLights { control, .. } => control.accept(packet),
            Self::Probe { control, .. } => control.accept(packet),
            Self::ProbeHeatMode { control, .. } => control.accept(packet),
            Self::PropWlight { control, .. } => control.accept(packet),
            Self::QFeelSw { control, .. } => control.accept(packet),
            Self::Qty1 { control, .. } => control.accept(packet),
            Self::Qty2 { control, .. } => control.accept(packet),
            Self::QuantSel01 { control, .. } => control.accept(packet),
            Self::QuantSel10 { control, .. } => control.accept(packet),
            Self::ReadyLight { control, .. } => control.accept(packet),
            Self::RecordSysDisplay { control, .. } => control.accept(packet),
            Self::RecordSysMode { control, .. } => control.accept(packet),
            Self::ReleaseControl { control, .. } => control.accept(packet),
            Self::ReleaseInt001 { control, .. } => control.accept(packet),
            Self::ReleaseInt010 { control, .. } => control.accept(packet),
            Self::ReleaseInt100 { control, .. } => control.accept(packet),
            Self::ReplyWlight { control, .. } => control.accept(packet),
            Self::RfuelLight { control, .. } => control.accept(packet),
            Self::RightGearG { control, .. } => control.accept(packet),
            Self::RightGearY { control, .. } => control.accept(packet),
            Self::RightLight { control, .. } => control.accept(packet),
            Self::RollWlight { control, .. } => control.accept(packet),
            Self::Rpm1 { control, .. } => control.accept(packet),
            Self::Rpm2 { control, .. } => control.accept(packet),
            Self::Rpm3 { control, .. } => control.accept(packet),
            Self::Rpm4 { control, .. } => control.accept(packet),
            Self::RpmSw { control, .. } => control.accept(packet),
            Self::RpumpWlight { control, .. } => control.accept(packet),
            Self::RtankLight { control, .. } => control.accept(packet),
            Self::RtransWlight { control, .. } => control.accept(packet),
            Self::RudderNeedle { control, .. } => control.accept(packet),
            Self::RudderTrim { control, .. } => control.accept(packet),
            Self::RwrVol { control, .. } => control.accept(packet),
            Self::SamLight { control, .. } => control.accept(packet),
            Self::SasPitch { control, .. } => control.accept(packet),
            Self::SasRoll { control, .. } => control.accept(packet),
            Self::SasYaw { control, .. } => control.accept(packet),
            Self::SeatAdj { control, .. } => control.accept(packet),
            Self::SeatSafeLever { control, .. } => control.accept(packet),
            Self::SelLight { control, .. } => control.accept(packet),
            Self::SetFlag1 { control, .. } => control.accept(packet),
            Self::SetFlag2 { control, .. } => control.accept(packet),
            Self::SetFlag3 { control, .. } => control.accept(packet),
            Self::SetFlag4 { control, .. } => control.accept(packet),
            Self::SetFlag5 { control, .. } => control.accept(packet),
            Self::SetFlag6 { control, .. } => control.accept(packet),
            Self::SetFlag7 { control, .. } => control.accept(packet),
            Self::SidepanelLights { control, .. } => control.accept(packet),
            Self::SkidWlight { control, .. } => control.accept(packet),
            Self::SlipInd { control, .. } => control.accept(packet),
            Self::SlipTurnFlag { control, .. } => control.accept(packet),
            Self::SpdBrkWlight { control, .. } => control.accept(packet),
            Self::Stab1 { control, .. } => control.accept(packet),
            Self::Stab2 { control, .. } => control.accept(packet),
            Self::StabArrow { control, .. } => control.accept(packet),
            Self::StbyTrWlight { control, .. } => control.accept(packet),
            Self::StoLever { control, .. } => control.accept(packet),
            Self::StoLight { control, .. } => control.accept(packet),
            Self::StopwatchLap { control, .. } => control.accept(packet),
            Self::StopwatchStart { control, .. } => control.accept(packet),
            Self::TempControl { control, .. } => control.accept(packet),
            Self::ThrottCutoffLv { control, .. } => control.accept(packet),
            Self::ThrottleFriction { control, .. } => control.accept(packet),
            Self::TotFuel1 { control, .. } => control.accept(packet),
            Self::TotFuel2 { control, .. } => control.accept(packet),
            Self::TotFuel3 { control, .. } => control.accept(packet),
            Self::TotFuel4 { control, .. } => control.accept(packet),
            Self::TotFuel5 { control, .. } => control.accept(packet),
            Self::TrimMode { control, .. } => control.accept(packet),
            Self::TurnIndi { control, .. } => control.accept(packet),
            Self::UfcAlt { control, .. } => control.accept(packet),
            Self::UfcB0 { control, .. } => control.accept(packet),
            Self::UfcB1 { control, .. } => control.accept(packet),
            Self::UfcB2 { control, .. } => control.accept(packet),
            Self::UfcB3 { control, .. } => control.accept(packet),
            Self::UfcB4 { control, .. } => control.accept(packet),
            Self::UfcB5 { control, .. } => control.accept(packet),
            Self::UfcB6 { control, .. } => control.accept(packet),
            Self::UfcB7 { control, .. } => control.accept(packet),
            Self::UfcB8 { control, .. } => control.accept(packet),
            Self::UfcB9 { control, .. } => control.accept(packet),
            Self::UfcBeacon { control, .. } => control.accept(packet),
            Self::UfcBright { control, .. } => control.accept(packet),
            Self::UfcClear { control, .. } => control.accept(packet),
            Self::UfcCom1Pull { control, .. } => control.accept(packet),
            Self::UfcCom1Sel { control, .. } => control.accept(packet),
            Self::UfcCom1Vol { control, .. } => control.accept(packet),
            Self::UfcCom2Pull { control, .. } => control.accept(packet),
            Self::UfcCom2Sel { control, .. } => control.accept(packet),
            Self::UfcCom2Vol { control, .. } => control.accept(packet),
            Self::UfcComm1Display { control, .. } => control.accept(packet),
            Self::UfcComm2Display { control, .. } => control.accept(packet),
            Self::UfcDash { control, .. } => control.accept(packet),
            Self::UfcDot { control, .. } => control.accept(packet),
            Self::UfcEmcom { control, .. } => control.accept(packet),
            Self::UfcEnter { control, .. } => control.accept(packet),
            Self::UfcIff { control, .. } => control.accept(packet),
            Self::UfcIp { control, .. } => control.accept(packet),
            Self::UfcOnoff { control, .. } => control.accept(packet),
            Self::UfcSave { control, .. } => control.accept(packet),
            Self::UfcScratchpad { control, .. } => control.accept(packet),
            Self::UfcTacan { control, .. } => control.accept(packet),
            Self::UfcTarget { control, .. } => control.accept(packet),
            Self::UfcTimer { control, .. } => control.accept(packet),
            Self::UfcWay { control, .. } => control.accept(packet),
            Self::UfcWeapon { control, .. } => control.accept(packet),
            Self::UfcWeather { control, .. } => control.accept(packet),
            Self::UhfAncPoint { control, .. } => control.accept(packet),
            Self::UhfAncSw { control, .. } => control.accept(packet),
            Self::UhfFreq { control, .. } => control.accept(packet),
            Self::UhfLoadSw { control, .. } => control.accept(packet),
            Self::UhfMode { control, .. } => control.accept(packet),
            Self::UhfVol { control, .. } => control.accept(packet),
            Self::VoltMeter { control, .. } => control.accept(packet),
            Self::VstolLight { control, .. } => control.accept(packet),
            Self::VstolSel { control, .. } => control.accept(packet),
            Self::Vvi { control, .. } => control.accept(packet),
            Self::WsldWlight { control, .. } => control.accept(packet),
            Self::YawTrimSw { control, .. } => control.accept(packet),
            Self::YawWlight { control, .. } => control.accept(packet),
        }
    }

    fn commit(&mut self) {
        match self {
            Self::AaaLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::AcnipMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AcnipRadioSel { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiBank { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiFlag { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiPitch { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AfcSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AfcWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::AftBayWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::AgLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::AgSel { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AiLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::AileronNeedle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Alt10000ftCnt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Alt1000ftCnt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Alt100ftCnt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltHoldSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltPointer { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltPressure1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltPressure2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltPressure3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltPressure4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AnnunciatorLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AntiCollLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AntiSkid { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AoaFlag { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AoaPointer { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ApuGenSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ApuGenWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ApuWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ArmControl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AutflpWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Av8bnaOdu1Select { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Av8bnaOdu1Text { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Av8bnaOdu2Select { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Av8bnaOdu2Text { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Av8bnaOdu3Select { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Av8bnaOdu3Text { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Av8bnaOdu4Select { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Av8bnaOdu4Text { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Av8bnaOdu5Select { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Av8bnaOdu5Text { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AvBitWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::BakAdiCageKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BakAdiCagePull { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BaroPressure { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BattSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Bingo1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Bingo2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Bingo3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Bingo4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BingoLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::BingoSet { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BombsArmSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Brake1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Brake2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Brake3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BrakeAccu { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CAut { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::CabinDefog { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CabinPress { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CabinPressSwitch { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CanopyHandL { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CanopyHandR { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CanopyLock { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CanopyValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CanopyWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClockHh { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockMm { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockSs { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Cmbt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CmbtLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Comm1StringFreq { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Comm2StringFreq { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CompLight { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CompassBank { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CompassLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CompassNs { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CompassUd { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ConsoleLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Control15sekLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::CsCoolWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::CwJamWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::CwLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::CwNogoWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::DcTestSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DcWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::DecoyControl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DecsSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DepResWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Dmt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DroopWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::DualCpuMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Duct1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Duct2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Duct3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EcsAft { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EcsFwd { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EdpBit { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EdpBright { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EfcLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::EfcSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EfcWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::EmergJett { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EmsB { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EngExcWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::EngStartSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtAuxLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtFormationLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtPositionLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtSpeedBrake { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtStrobeTail { control, callback } => {
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
            Self::ExtWowTail { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Ff1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Ff2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Ff3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FireLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FlapMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FlapPos1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FlapPos2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FlapPower { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Flaps1Wlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Flaps2Wlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FlapsBit { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FlapsLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Flir { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FloodLBhAftB { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FloodLBhAftF { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FloodLBhFwd { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FloodLCanB { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FloodLCanT { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FloodLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FloodLightsDial { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FloodRBhAftB { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FloodRBhAftF { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FloodRBhFwd { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FloodRCanB { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FloodRCanT { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FormationLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelDumpL { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelDumpR { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelFlag { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelProp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelPumpL { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelPumpR { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelSel { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelShutoff { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fuz1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fuz2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuzeControl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GearLeverLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::GearLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::GenLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::GenSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GndIrCool { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GpsWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::H2o1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::H2o2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::H2oControlLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::H2oDump { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::H2oLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::H2oMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::H2oSel { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::HhydLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::HideStick { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HudAlt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HudBright { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HudMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HudSymbol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HudVideoBright { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HudVideoContr { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Hyd11 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Hyd12 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Hyd13 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Hyd1Wlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Hyd21 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Hyd22 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Hyd23 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Hyd2Wlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::IcsAuxVol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IcsGndVol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IcsMicMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IffCryptMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IffOperMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IffWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::InsMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::InsWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::InstLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Invt1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Invt2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Invt3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::JamHotWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::JammerControl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::JettMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::JettStation1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::JettStation1Light { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::JettStation2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::JettStation2Light { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::JettStation3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::JettStation3Light { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::JettStation4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::JettStation4Light { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::JettStation5 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::JettStation5Light { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::JettStation6 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::JettStation6Light { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::JettStation7 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::JettStation7Light { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::JettStores { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Jpt1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Jpt2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Jpt3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::JptlLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::JptlSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::KntPointerL { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::KntPointerS { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Ky1Mode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Ky1Type { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Ky2Mode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Ky2Type { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Ky58CodeClear { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Ky58CodeLoad { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LandingTaxiLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LaunchFlareSalvo { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LawLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LbsL1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LbsL2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LbsL3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LbsL4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LbsR1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LbsR2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LbsR3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LbsR4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LeftGearG { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LeftGearY { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LeftLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LfuelLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LgDownLock { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LgEmergLever { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LgEmergbattLever { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LgLever { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LidsSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LidsWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Line2Wlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LineWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LoadWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LpumpWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LtankLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LtransWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::MCaution { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MWarning { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MainGearG { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::MainGearY { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ManFuelSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MasterArm { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::McLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::MfsEmergLever { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MfsLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::MipLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MirrorToggleC { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MirrorToggleL { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MirrorToggleR { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MissPcMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ModeGauge { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdL1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdL10 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdL11 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdL12 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdL13 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdL14 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdL15 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdL16 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdL17 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdL18 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdL19 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdL2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdL20 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdL3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdL4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdL5 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdL6 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdL7 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdL8 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdL9 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdLBright { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdLContrast { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdLDayNight { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdLGain { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdLSymbol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdR1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdR10 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdR11 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdR12 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdR13 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdR14 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdR15 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdR16 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdR17 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdR18 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdR19 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdR2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdR20 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdR3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdR4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdR5 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdR6 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdR7 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdR8 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdR9 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdRBright { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdRContrast { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdRDayNight { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdRGain { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MpcdRSymbol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MultGauge { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MultiReleaseControl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MwLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::NavCrs { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NavLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::NavSel { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NoseGearG { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::NoseGearY { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::NozzleFriction { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NozzleLever { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NozzlePointer { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NwsWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::O2Sw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OduOpt1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OduOpt2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OduOpt3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OduOpt4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OduOpt5 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OilWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::OtLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::OxyWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PJamWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PNogoWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ParkBrakeLv { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PitchWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PosLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Probe { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ProbeHeatMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PropWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::QFeelSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Qty1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Qty2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::QuantSel01 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::QuantSel10 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ReadyLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RecordSysDisplay { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RecordSysMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ReleaseControl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ReleaseInt001 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ReleaseInt010 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ReleaseInt100 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ReplyWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RfuelLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RightGearG { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RightGearY { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RightLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RollWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Rpm1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Rpm2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Rpm3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Rpm4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RpmSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RpumpWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RtankLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RtransWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RudderNeedle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RudderTrim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RwrVol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SamLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::SasPitch { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SasRoll { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SasYaw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SeatAdj { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SeatSafeLever { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SelLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::SetFlag1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SetFlag2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SetFlag3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SetFlag4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SetFlag5 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SetFlag6 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SetFlag7 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SidepanelLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SkidWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::SlipInd { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SlipTurnFlag { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SpdBrkWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Stab1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Stab2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StabArrow { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StbyTrWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::StoLever { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StoLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::StopwatchLap { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StopwatchStart { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TempControl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ThrottCutoffLv { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ThrottleFriction { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TotFuel1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TotFuel2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TotFuel3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TotFuel4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TotFuel5 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TrimMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TurnIndi { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcAlt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcB0 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcB1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcB2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcB3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcB4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcB5 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcB6 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcB7 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcB8 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcB9 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcBeacon { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcBright { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcClear { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcCom1Pull { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcCom1Sel { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcCom1Vol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcCom2Pull { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcCom2Sel { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcCom2Vol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcComm1Display { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcComm2Display { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcDash { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcDot { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcEmcom { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcEnter { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcIff { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcIp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcOnoff { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcSave { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcScratchpad { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcTacan { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcTarget { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcTimer { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcWay { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcWeapon { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcWeather { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfAncPoint { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfAncSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfFreq { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfLoadSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfVol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VoltMeter { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VstolLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::VstolSel { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Vvi { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::WsldWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::YawTrimSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::YawWlight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
        }
    }
}

pub struct Av8bnaBuilder<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    _packet_capacity: PhantomData<ProtocolParser<MAX_PACKET_DATA>>,
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    Av8bnaBuilder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
{
    pub fn new() -> Self {
        Self {
            controls: Vec::new(),
            _packet_capacity: PhantomData,
        }
    }

    pub fn register_aaa_light<F>(
        &mut self,
        control: AaaLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AaaLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_acnip_mode<F>(
        &mut self,
        control: AcnipMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AcnipMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_acnip_radio_sel<F>(
        &mut self,
        control: AcnipRadioSel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AcnipRadioSel { control, callback })
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

    pub fn register_adi_flag<F>(
        &mut self,
        control: AdiFlag,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiFlag { control, callback })
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

    pub fn register_afc_sw<F>(
        &mut self,
        control: AfcSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AfcSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_afc_wlight<F>(
        &mut self,
        control: AfcWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AfcWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aft_bay_wlight<F>(
        &mut self,
        control: AftBayWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AftBayWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ag_light<F>(
        &mut self,
        control: AgLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AgLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ag_sel<F>(
        &mut self,
        control: AgSel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AgSel { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ai_light<F>(
        &mut self,
        control: AiLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AiLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aileron_needle<F>(
        &mut self,
        control: AileronNeedle,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AileronNeedle { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt10000ft_cnt<F>(
        &mut self,
        control: Alt10000ftCnt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Alt10000ftCnt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt1000ft_cnt<F>(
        &mut self,
        control: Alt1000ftCnt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Alt1000ftCnt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt100ft_cnt<F>(
        &mut self,
        control: Alt100ftCnt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Alt100ftCnt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_hold_sw<F>(
        &mut self,
        control: AltHoldSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltHoldSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_pointer<F>(
        &mut self,
        control: AltPointer,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltPointer { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_pressure1<F>(
        &mut self,
        control: AltPressure1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltPressure1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_pressure2<F>(
        &mut self,
        control: AltPressure2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltPressure2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_pressure3<F>(
        &mut self,
        control: AltPressure3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltPressure3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_pressure4<F>(
        &mut self,
        control: AltPressure4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltPressure4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_annunciator_lights<F>(
        &mut self,
        control: AnnunciatorLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AnnunciatorLights { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_anti_coll_lights<F>(
        &mut self,
        control: AntiCollLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AntiCollLights { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_anti_skid<F>(
        &mut self,
        control: AntiSkid,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AntiSkid { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aoa_flag<F>(
        &mut self,
        control: AoaFlag,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AoaFlag { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aoa_pointer<F>(
        &mut self,
        control: AoaPointer,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AoaPointer { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_apu_gen_sw<F>(
        &mut self,
        control: ApuGenSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApuGenSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_apu_gen_wlight<F>(
        &mut self,
        control: ApuGenWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApuGenWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_apu_wlight<F>(
        &mut self,
        control: ApuWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApuWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_arm_control<F>(
        &mut self,
        control: ArmControl,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ArmControl { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_autflp_wlight<F>(
        &mut self,
        control: AutflpWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AutflpWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_av8bna_odu1_select<F>(
        &mut self,
        control: Av8bnaOdu1Select,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Av8bnaOdu1Select { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_av8bna_odu1_text<F>(
        &mut self,
        control: Av8bnaOdu1Text,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Av8bnaOdu1Text { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_av8bna_odu2_select<F>(
        &mut self,
        control: Av8bnaOdu2Select,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Av8bnaOdu2Select { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_av8bna_odu2_text<F>(
        &mut self,
        control: Av8bnaOdu2Text,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Av8bnaOdu2Text { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_av8bna_odu3_select<F>(
        &mut self,
        control: Av8bnaOdu3Select,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Av8bnaOdu3Select { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_av8bna_odu3_text<F>(
        &mut self,
        control: Av8bnaOdu3Text,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Av8bnaOdu3Text { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_av8bna_odu4_select<F>(
        &mut self,
        control: Av8bnaOdu4Select,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Av8bnaOdu4Select { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_av8bna_odu4_text<F>(
        &mut self,
        control: Av8bnaOdu4Text,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Av8bnaOdu4Text { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_av8bna_odu5_select<F>(
        &mut self,
        control: Av8bnaOdu5Select,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Av8bnaOdu5Select { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_av8bna_odu5_text<F>(
        &mut self,
        control: Av8bnaOdu5Text,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Av8bnaOdu5Text { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_av_bit_wlight<F>(
        &mut self,
        control: AvBitWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AvBitWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bak_adi_cage_knob<F>(
        &mut self,
        control: BakAdiCageKnob,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BakAdiCageKnob { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bak_adi_cage_pull<F>(
        &mut self,
        control: BakAdiCagePull,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BakAdiCagePull { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_baro_pressure<F>(
        &mut self,
        control: BaroPressure,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BaroPressure { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_batt_sw<F>(
        &mut self,
        control: BattSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BattSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bingo1<F>(
        &mut self,
        control: Bingo1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Bingo1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bingo2<F>(
        &mut self,
        control: Bingo2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Bingo2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bingo3<F>(
        &mut self,
        control: Bingo3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Bingo3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bingo4<F>(
        &mut self,
        control: Bingo4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Bingo4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bingo_light<F>(
        &mut self,
        control: BingoLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::BingoLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bingo_set<F>(
        &mut self,
        control: BingoSet,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BingoSet { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bombs_arm_sw<F>(
        &mut self,
        control: BombsArmSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BombsArmSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brake1<F>(
        &mut self,
        control: Brake1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Brake1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brake2<F>(
        &mut self,
        control: Brake2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Brake2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brake3<F>(
        &mut self,
        control: Brake3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Brake3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brake_accu<F>(
        &mut self,
        control: BrakeAccu,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BrakeAccu { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_c_aut<F>(
        &mut self,
        control: CAut,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::CAut { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cabin_defog<F>(
        &mut self,
        control: CabinDefog,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CabinDefog { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cabin_press<F>(
        &mut self,
        control: CabinPress,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CabinPress { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cabin_press_switch<F>(
        &mut self,
        control: CabinPressSwitch,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CabinPressSwitch { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_hand_l<F>(
        &mut self,
        control: CanopyHandL,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyHandL { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_hand_r<F>(
        &mut self,
        control: CanopyHandR,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyHandR { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_lock<F>(
        &mut self,
        control: CanopyLock,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyLock { control, callback })
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

    pub fn register_canopy_wlight<F>(
        &mut self,
        control: CanopyWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_hh<F>(
        &mut self,
        control: ClockHh,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockHh { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_mm<F>(
        &mut self,
        control: ClockMm,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockMm { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_ss<F>(
        &mut self,
        control: ClockSs,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockSs { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmbt<F>(
        &mut self,
        control: Cmbt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Cmbt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmbt_light<F>(
        &mut self,
        control: CmbtLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmbtLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_comm1_string_freq<F>(
        &mut self,
        control: Comm1StringFreq,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Comm1StringFreq { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_comm2_string_freq<F>(
        &mut self,
        control: Comm2StringFreq,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Comm2StringFreq { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_comp_light<F>(
        &mut self,
        control: CompLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CompLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_compass_bank<F>(
        &mut self,
        control: CompassBank,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CompassBank { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_compass_lights<F>(
        &mut self,
        control: CompassLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CompassLights { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_compass_ns<F>(
        &mut self,
        control: CompassNs,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CompassNs { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_compass_ud<F>(
        &mut self,
        control: CompassUd,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CompassUd { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_console_lights<F>(
        &mut self,
        control: ConsoleLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ConsoleLights { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_control15sek_light<F>(
        &mut self,
        control: Control15sekLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Control15sekLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cs_cool_wlight<F>(
        &mut self,
        control: CsCoolWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::CsCoolWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cw_jam_wlight<F>(
        &mut self,
        control: CwJamWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::CwJamWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cw_light<F>(
        &mut self,
        control: CwLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::CwLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cw_nogo_wlight<F>(
        &mut self,
        control: CwNogoWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::CwNogoWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dc_test_sw<F>(
        &mut self,
        control: DcTestSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DcTestSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dc_wlight<F>(
        &mut self,
        control: DcWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::DcWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_decoy_control<F>(
        &mut self,
        control: DecoyControl,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DecoyControl { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_decs_sw<F>(
        &mut self,
        control: DecsSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DecsSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dep_res_wlight<F>(
        &mut self,
        control: DepResWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::DepResWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dmt<F>(
        &mut self,
        control: Dmt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Dmt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_droop_wlight<F>(
        &mut self,
        control: DroopWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::DroopWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dual_cpu_mode<F>(
        &mut self,
        control: DualCpuMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DualCpuMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_duct1<F>(
        &mut self,
        control: Duct1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Duct1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_duct2<F>(
        &mut self,
        control: Duct2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Duct2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_duct3<F>(
        &mut self,
        control: Duct3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Duct3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecs_aft<F>(
        &mut self,
        control: EcsAft,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcsAft { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecs_fwd<F>(
        &mut self,
        control: EcsFwd,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcsFwd { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_edp_bit<F>(
        &mut self,
        control: EdpBit,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EdpBit { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_edp_bright<F>(
        &mut self,
        control: EdpBright,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EdpBright { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_efc_light<F>(
        &mut self,
        control: EfcLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::EfcLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_efc_sw<F>(
        &mut self,
        control: EfcSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EfcSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_efc_wlight<F>(
        &mut self,
        control: EfcWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::EfcWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emerg_jett<F>(
        &mut self,
        control: EmergJett,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmergJett { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ems_b<F>(
        &mut self,
        control: EmsB,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmsB { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_exc_wlight<F>(
        &mut self,
        control: EngExcWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngExcWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_start_sw<F>(
        &mut self,
        control: EngStartSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngStartSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_aux_lights<F>(
        &mut self,
        control: ExtAuxLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtAuxLights { control, callback })
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

    pub fn register_ext_lights<F>(
        &mut self,
        control: ExtLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtLights { control, callback })
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

    pub fn register_ext_speed_brake<F>(
        &mut self,
        control: ExtSpeedBrake,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtSpeedBrake { control, callback })
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

    pub fn register_ext_wow_tail<F>(
        &mut self,
        control: ExtWowTail,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtWowTail { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ff1<F>(
        &mut self,
        control: Ff1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ff1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ff2<F>(
        &mut self,
        control: Ff2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ff2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ff3<F>(
        &mut self,
        control: Ff3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ff3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fire_light<F>(
        &mut self,
        control: FireLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FireLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flap_mode<F>(
        &mut self,
        control: FlapMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlapMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flap_pos1<F>(
        &mut self,
        control: FlapPos1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlapPos1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flap_pos2<F>(
        &mut self,
        control: FlapPos2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlapPos2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flap_power<F>(
        &mut self,
        control: FlapPower,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlapPower { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flaps1_wlight<F>(
        &mut self,
        control: Flaps1Wlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Flaps1Wlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flaps2_wlight<F>(
        &mut self,
        control: Flaps2Wlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Flaps2Wlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flaps_bit<F>(
        &mut self,
        control: FlapsBit,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlapsBit { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flaps_light<F>(
        &mut self,
        control: FlapsLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlapsLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flir<F>(
        &mut self,
        control: Flir,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Flir { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flood_l_bh_aft_b<F>(
        &mut self,
        control: FloodLBhAftB,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FloodLBhAftB { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flood_l_bh_aft_f<F>(
        &mut self,
        control: FloodLBhAftF,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FloodLBhAftF { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flood_l_bh_fwd<F>(
        &mut self,
        control: FloodLBhFwd,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FloodLBhFwd { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flood_l_can_b<F>(
        &mut self,
        control: FloodLCanB,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FloodLCanB { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flood_l_can_t<F>(
        &mut self,
        control: FloodLCanT,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FloodLCanT { control, callback })
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

    pub fn register_flood_lights_dial<F>(
        &mut self,
        control: FloodLightsDial,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FloodLightsDial { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flood_r_bh_aft_b<F>(
        &mut self,
        control: FloodRBhAftB,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FloodRBhAftB { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flood_r_bh_aft_f<F>(
        &mut self,
        control: FloodRBhAftF,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FloodRBhAftF { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flood_r_bh_fwd<F>(
        &mut self,
        control: FloodRBhFwd,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FloodRBhFwd { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flood_r_can_b<F>(
        &mut self,
        control: FloodRCanB,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FloodRCanB { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flood_r_can_t<F>(
        &mut self,
        control: FloodRCanT,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FloodRCanT { control, callback })
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

    pub fn register_fuel_dump_l<F>(
        &mut self,
        control: FuelDumpL,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelDumpL { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_dump_r<F>(
        &mut self,
        control: FuelDumpR,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelDumpR { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_flag<F>(
        &mut self,
        control: FuelFlag,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelFlag { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_prop<F>(
        &mut self,
        control: FuelProp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelProp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_pump_l<F>(
        &mut self,
        control: FuelPumpL,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelPumpL { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_pump_r<F>(
        &mut self,
        control: FuelPumpR,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelPumpR { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_sel<F>(
        &mut self,
        control: FuelSel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelSel { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_shutoff<F>(
        &mut self,
        control: FuelShutoff,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelShutoff { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuz1<F>(
        &mut self,
        control: Fuz1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fuz1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuz2<F>(
        &mut self,
        control: Fuz2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fuz2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuze_control<F>(
        &mut self,
        control: FuzeControl,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuzeControl { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gear_lever_light<F>(
        &mut self,
        control: GearLeverLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::GearLeverLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gear_light<F>(
        &mut self,
        control: GearLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::GearLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gen_light<F>(
        &mut self,
        control: GenLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::GenLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gen_sw<F>(
        &mut self,
        control: GenSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GenSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gnd_ir_cool<F>(
        &mut self,
        control: GndIrCool,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GndIrCool { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gps_wlight<F>(
        &mut self,
        control: GpsWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::GpsWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_h2o1<F>(
        &mut self,
        control: H2o1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::H2o1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_h2o2<F>(
        &mut self,
        control: H2o2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::H2o2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_h2o_control_light<F>(
        &mut self,
        control: H2oControlLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::H2oControlLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_h2o_dump<F>(
        &mut self,
        control: H2oDump,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::H2oDump { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_h2o_light<F>(
        &mut self,
        control: H2oLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::H2oLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_h2o_mode<F>(
        &mut self,
        control: H2oMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::H2oMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_h2o_sel<F>(
        &mut self,
        control: H2oSel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::H2oSel { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hhyd_light<F>(
        &mut self,
        control: HhydLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::HhydLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hide_stick<F>(
        &mut self,
        control: HideStick,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HideStick { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_alt<F>(
        &mut self,
        control: HudAlt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudAlt { control, callback })
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

    pub fn register_hud_mode<F>(
        &mut self,
        control: HudMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_symbol<F>(
        &mut self,
        control: HudSymbol,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudSymbol { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_video_bright<F>(
        &mut self,
        control: HudVideoBright,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudVideoBright { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_video_contr<F>(
        &mut self,
        control: HudVideoContr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudVideoContr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hyd11<F>(
        &mut self,
        control: Hyd11,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Hyd11 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hyd12<F>(
        &mut self,
        control: Hyd12,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Hyd12 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hyd13<F>(
        &mut self,
        control: Hyd13,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Hyd13 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hyd1_wlight<F>(
        &mut self,
        control: Hyd1Wlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Hyd1Wlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hyd21<F>(
        &mut self,
        control: Hyd21,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Hyd21 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hyd22<F>(
        &mut self,
        control: Hyd22,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Hyd22 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hyd23<F>(
        &mut self,
        control: Hyd23,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Hyd23 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hyd2_wlight<F>(
        &mut self,
        control: Hyd2Wlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Hyd2Wlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ics_aux_vol<F>(
        &mut self,
        control: IcsAuxVol,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcsAuxVol { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ics_gnd_vol<F>(
        &mut self,
        control: IcsGndVol,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcsGndVol { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ics_mic_mode<F>(
        &mut self,
        control: IcsMicMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcsMicMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_crypt_mode<F>(
        &mut self,
        control: IffCryptMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffCryptMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_oper_mode<F>(
        &mut self,
        control: IffOperMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffOperMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_wlight<F>(
        &mut self,
        control: IffWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_mode<F>(
        &mut self,
        control: InsMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_wlight<F>(
        &mut self,
        control: InsWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_inst_lights<F>(
        &mut self,
        control: InstLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::InstLights { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_invt1<F>(
        &mut self,
        control: Invt1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Invt1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_invt2<F>(
        &mut self,
        control: Invt2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Invt2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_invt3<F>(
        &mut self,
        control: Invt3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Invt3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jam_hot_wlight<F>(
        &mut self,
        control: JamHotWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::JamHotWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jammer_control<F>(
        &mut self,
        control: JammerControl,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::JammerControl { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jett_mode<F>(
        &mut self,
        control: JettMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::JettMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jett_station1<F>(
        &mut self,
        control: JettStation1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::JettStation1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jett_station1_light<F>(
        &mut self,
        control: JettStation1Light,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::JettStation1Light { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jett_station2<F>(
        &mut self,
        control: JettStation2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::JettStation2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jett_station2_light<F>(
        &mut self,
        control: JettStation2Light,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::JettStation2Light { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jett_station3<F>(
        &mut self,
        control: JettStation3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::JettStation3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jett_station3_light<F>(
        &mut self,
        control: JettStation3Light,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::JettStation3Light { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jett_station4<F>(
        &mut self,
        control: JettStation4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::JettStation4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jett_station4_light<F>(
        &mut self,
        control: JettStation4Light,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::JettStation4Light { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jett_station5<F>(
        &mut self,
        control: JettStation5,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::JettStation5 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jett_station5_light<F>(
        &mut self,
        control: JettStation5Light,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::JettStation5Light { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jett_station6<F>(
        &mut self,
        control: JettStation6,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::JettStation6 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jett_station6_light<F>(
        &mut self,
        control: JettStation6Light,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::JettStation6Light { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jett_station7<F>(
        &mut self,
        control: JettStation7,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::JettStation7 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jett_station7_light<F>(
        &mut self,
        control: JettStation7Light,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::JettStation7Light { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jett_stores<F>(
        &mut self,
        control: JettStores,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::JettStores { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jpt1<F>(
        &mut self,
        control: Jpt1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Jpt1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jpt2<F>(
        &mut self,
        control: Jpt2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Jpt2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jpt3<F>(
        &mut self,
        control: Jpt3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Jpt3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jptl_light<F>(
        &mut self,
        control: JptlLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::JptlLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jptl_sw<F>(
        &mut self,
        control: JptlSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::JptlSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_knt_pointer_l<F>(
        &mut self,
        control: KntPointerL,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::KntPointerL { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_knt_pointer_s<F>(
        &mut self,
        control: KntPointerS,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::KntPointerS { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ky1_mode<F>(
        &mut self,
        control: Ky1Mode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ky1Mode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ky1_type<F>(
        &mut self,
        control: Ky1Type,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ky1Type { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ky2_mode<F>(
        &mut self,
        control: Ky2Mode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ky2Mode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ky2_type<F>(
        &mut self,
        control: Ky2Type,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ky2Type { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ky58_code_clear<F>(
        &mut self,
        control: Ky58CodeClear,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ky58CodeClear { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ky58_code_load<F>(
        &mut self,
        control: Ky58CodeLoad,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ky58CodeLoad { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_landing_taxi_lights<F>(
        &mut self,
        control: LandingTaxiLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LandingTaxiLights { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_launch_flare_salvo<F>(
        &mut self,
        control: LaunchFlareSalvo,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LaunchFlareSalvo { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_law_light<F>(
        &mut self,
        control: LawLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LawLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lbs_l1<F>(
        &mut self,
        control: LbsL1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LbsL1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lbs_l2<F>(
        &mut self,
        control: LbsL2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LbsL2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lbs_l3<F>(
        &mut self,
        control: LbsL3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LbsL3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lbs_l4<F>(
        &mut self,
        control: LbsL4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LbsL4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lbs_r1<F>(
        &mut self,
        control: LbsR1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LbsR1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lbs_r2<F>(
        &mut self,
        control: LbsR2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LbsR2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lbs_r3<F>(
        &mut self,
        control: LbsR3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LbsR3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lbs_r4<F>(
        &mut self,
        control: LbsR4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LbsR4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_gear_g<F>(
        &mut self,
        control: LeftGearG,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftGearG { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_gear_y<F>(
        &mut self,
        control: LeftGearY,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftGearY { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_light<F>(
        &mut self,
        control: LeftLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lfuel_light<F>(
        &mut self,
        control: LfuelLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LfuelLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lg_down_lock<F>(
        &mut self,
        control: LgDownLock,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LgDownLock { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lg_emerg_lever<F>(
        &mut self,
        control: LgEmergLever,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LgEmergLever { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lg_emergbatt_lever<F>(
        &mut self,
        control: LgEmergbattLever,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LgEmergbattLever { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lg_lever<F>(
        &mut self,
        control: LgLever,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LgLever { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lids_sw<F>(
        &mut self,
        control: LidsSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LidsSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lids_wlight<F>(
        &mut self,
        control: LidsWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LidsWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_line2_wlight<F>(
        &mut self,
        control: Line2Wlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Line2Wlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_line_wlight<F>(
        &mut self,
        control: LineWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LineWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_load_wlight<F>(
        &mut self,
        control: LoadWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LoadWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lpump_wlight<F>(
        &mut self,
        control: LpumpWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LpumpWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ltank_light<F>(
        &mut self,
        control: LtankLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LtankLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ltrans_wlight<F>(
        &mut self,
        control: LtransWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LtransWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_m_caution<F>(
        &mut self,
        control: MCaution,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MCaution { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_m_warning<F>(
        &mut self,
        control: MWarning,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MWarning { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_main_gear_g<F>(
        &mut self,
        control: MainGearG,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::MainGearG { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_main_gear_y<F>(
        &mut self,
        control: MainGearY,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::MainGearY { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_man_fuel_sw<F>(
        &mut self,
        control: ManFuelSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ManFuelSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_arm<F>(
        &mut self,
        control: MasterArm,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterArm { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mc_light<F>(
        &mut self,
        control: McLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::McLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfs_emerg_lever<F>(
        &mut self,
        control: MfsEmergLever,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfsEmergLever { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfs_light<F>(
        &mut self,
        control: MfsLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfsLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mip_lights<F>(
        &mut self,
        control: MipLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MipLights { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mirror_toggle_c<F>(
        &mut self,
        control: MirrorToggleC,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MirrorToggleC { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mirror_toggle_l<F>(
        &mut self,
        control: MirrorToggleL,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MirrorToggleL { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mirror_toggle_r<F>(
        &mut self,
        control: MirrorToggleR,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MirrorToggleR { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_miss_pc_mode<F>(
        &mut self,
        control: MissPcMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MissPcMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mode_gauge<F>(
        &mut self,
        control: ModeGauge,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ModeGauge { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l1<F>(
        &mut self,
        control: MpcdL1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdL1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l10<F>(
        &mut self,
        control: MpcdL10,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdL10 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l11<F>(
        &mut self,
        control: MpcdL11,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdL11 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l12<F>(
        &mut self,
        control: MpcdL12,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdL12 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l13<F>(
        &mut self,
        control: MpcdL13,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdL13 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l14<F>(
        &mut self,
        control: MpcdL14,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdL14 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l15<F>(
        &mut self,
        control: MpcdL15,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdL15 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l16<F>(
        &mut self,
        control: MpcdL16,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdL16 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l17<F>(
        &mut self,
        control: MpcdL17,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdL17 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l18<F>(
        &mut self,
        control: MpcdL18,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdL18 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l19<F>(
        &mut self,
        control: MpcdL19,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdL19 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l2<F>(
        &mut self,
        control: MpcdL2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdL2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l20<F>(
        &mut self,
        control: MpcdL20,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdL20 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l3<F>(
        &mut self,
        control: MpcdL3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdL3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l4<F>(
        &mut self,
        control: MpcdL4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdL4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l5<F>(
        &mut self,
        control: MpcdL5,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdL5 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l6<F>(
        &mut self,
        control: MpcdL6,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdL6 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l7<F>(
        &mut self,
        control: MpcdL7,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdL7 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l8<F>(
        &mut self,
        control: MpcdL8,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdL8 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l9<F>(
        &mut self,
        control: MpcdL9,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdL9 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l_bright<F>(
        &mut self,
        control: MpcdLBright,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdLBright { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l_contrast<F>(
        &mut self,
        control: MpcdLContrast,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdLContrast { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l_day_night<F>(
        &mut self,
        control: MpcdLDayNight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdLDayNight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l_gain<F>(
        &mut self,
        control: MpcdLGain,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdLGain { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_l_symbol<F>(
        &mut self,
        control: MpcdLSymbol,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdLSymbol { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r1<F>(
        &mut self,
        control: MpcdR1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdR1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r10<F>(
        &mut self,
        control: MpcdR10,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdR10 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r11<F>(
        &mut self,
        control: MpcdR11,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdR11 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r12<F>(
        &mut self,
        control: MpcdR12,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdR12 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r13<F>(
        &mut self,
        control: MpcdR13,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdR13 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r14<F>(
        &mut self,
        control: MpcdR14,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdR14 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r15<F>(
        &mut self,
        control: MpcdR15,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdR15 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r16<F>(
        &mut self,
        control: MpcdR16,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdR16 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r17<F>(
        &mut self,
        control: MpcdR17,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdR17 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r18<F>(
        &mut self,
        control: MpcdR18,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdR18 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r19<F>(
        &mut self,
        control: MpcdR19,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdR19 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r2<F>(
        &mut self,
        control: MpcdR2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdR2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r20<F>(
        &mut self,
        control: MpcdR20,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdR20 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r3<F>(
        &mut self,
        control: MpcdR3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdR3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r4<F>(
        &mut self,
        control: MpcdR4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdR4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r5<F>(
        &mut self,
        control: MpcdR5,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdR5 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r6<F>(
        &mut self,
        control: MpcdR6,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdR6 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r7<F>(
        &mut self,
        control: MpcdR7,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdR7 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r8<F>(
        &mut self,
        control: MpcdR8,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdR8 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r9<F>(
        &mut self,
        control: MpcdR9,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdR9 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r_bright<F>(
        &mut self,
        control: MpcdRBright,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdRBright { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r_contrast<F>(
        &mut self,
        control: MpcdRContrast,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdRContrast { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r_day_night<F>(
        &mut self,
        control: MpcdRDayNight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdRDayNight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r_gain<F>(
        &mut self,
        control: MpcdRGain,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdRGain { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mpcd_r_symbol<F>(
        &mut self,
        control: MpcdRSymbol,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MpcdRSymbol { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mult_gauge<F>(
        &mut self,
        control: MultGauge,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MultGauge { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_multi_release_control<F>(
        &mut self,
        control: MultiReleaseControl,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MultiReleaseControl { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mw_light<F>(
        &mut self,
        control: MwLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::MwLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_crs<F>(
        &mut self,
        control: NavCrs,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavCrs { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_light<F>(
        &mut self,
        control: NavLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_sel<F>(
        &mut self,
        control: NavSel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSel { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nose_gear_g<F>(
        &mut self,
        control: NoseGearG,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::NoseGearG { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nose_gear_y<F>(
        &mut self,
        control: NoseGearY,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::NoseGearY { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nozzle_friction<F>(
        &mut self,
        control: NozzleFriction,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NozzleFriction { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nozzle_lever<F>(
        &mut self,
        control: NozzleLever,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NozzleLever { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nozzle_pointer<F>(
        &mut self,
        control: NozzlePointer,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NozzlePointer { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nws_wlight<F>(
        &mut self,
        control: NwsWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::NwsWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_o2_sw<F>(
        &mut self,
        control: O2Sw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::O2Sw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_odu_opt1<F>(
        &mut self,
        control: OduOpt1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OduOpt1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_odu_opt2<F>(
        &mut self,
        control: OduOpt2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OduOpt2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_odu_opt3<F>(
        &mut self,
        control: OduOpt3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OduOpt3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_odu_opt4<F>(
        &mut self,
        control: OduOpt4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OduOpt4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_odu_opt5<F>(
        &mut self,
        control: OduOpt5,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OduOpt5 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oil_wlight<F>(
        &mut self,
        control: OilWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::OilWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ot_light<F>(
        &mut self,
        control: OtLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::OtLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxy_wlight<F>(
        &mut self,
        control: OxyWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxyWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_p_jam_wlight<F>(
        &mut self,
        control: PJamWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PJamWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_p_nogo_wlight<F>(
        &mut self,
        control: PNogoWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PNogoWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_park_brake_lv<F>(
        &mut self,
        control: ParkBrakeLv,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ParkBrakeLv { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pitch_wlight<F>(
        &mut self,
        control: PitchWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PitchWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pos_lights<F>(
        &mut self,
        control: PosLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PosLights { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_probe<F>(
        &mut self,
        control: Probe,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Probe { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_probe_heat_mode<F>(
        &mut self,
        control: ProbeHeatMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ProbeHeatMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_prop_wlight<F>(
        &mut self,
        control: PropWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PropWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_q_feel_sw<F>(
        &mut self,
        control: QFeelSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::QFeelSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_qty1<F>(
        &mut self,
        control: Qty1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Qty1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_qty2<F>(
        &mut self,
        control: Qty2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Qty2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_quant_sel01<F>(
        &mut self,
        control: QuantSel01,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::QuantSel01 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_quant_sel10<F>(
        &mut self,
        control: QuantSel10,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::QuantSel10 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ready_light<F>(
        &mut self,
        control: ReadyLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ReadyLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_record_sys_display<F>(
        &mut self,
        control: RecordSysDisplay,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RecordSysDisplay { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_record_sys_mode<F>(
        &mut self,
        control: RecordSysMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RecordSysMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_release_control<F>(
        &mut self,
        control: ReleaseControl,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ReleaseControl { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_release_int001<F>(
        &mut self,
        control: ReleaseInt001,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ReleaseInt001 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_release_int010<F>(
        &mut self,
        control: ReleaseInt010,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ReleaseInt010 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_release_int100<F>(
        &mut self,
        control: ReleaseInt100,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ReleaseInt100 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_reply_wlight<F>(
        &mut self,
        control: ReplyWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ReplyWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rfuel_light<F>(
        &mut self,
        control: RfuelLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RfuelLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_gear_g<F>(
        &mut self,
        control: RightGearG,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightGearG { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_gear_y<F>(
        &mut self,
        control: RightGearY,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightGearY { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_light<F>(
        &mut self,
        control: RightLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_roll_wlight<F>(
        &mut self,
        control: RollWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RollWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rpm1<F>(
        &mut self,
        control: Rpm1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Rpm1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rpm2<F>(
        &mut self,
        control: Rpm2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Rpm2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rpm3<F>(
        &mut self,
        control: Rpm3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Rpm3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rpm4<F>(
        &mut self,
        control: Rpm4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Rpm4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rpm_sw<F>(
        &mut self,
        control: RpmSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RpmSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rpump_wlight<F>(
        &mut self,
        control: RpumpWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RpumpWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rtank_light<F>(
        &mut self,
        control: RtankLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RtankLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rtrans_wlight<F>(
        &mut self,
        control: RtransWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RtransWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rudder_needle<F>(
        &mut self,
        control: RudderNeedle,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RudderNeedle { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rudder_trim<F>(
        &mut self,
        control: RudderTrim,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RudderTrim { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_vol<F>(
        &mut self,
        control: RwrVol,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrVol { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sam_light<F>(
        &mut self,
        control: SamLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::SamLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sas_pitch<F>(
        &mut self,
        control: SasPitch,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SasPitch { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sas_roll<F>(
        &mut self,
        control: SasRoll,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SasRoll { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sas_yaw<F>(
        &mut self,
        control: SasYaw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SasYaw { control, callback })
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

    pub fn register_seat_safe_lever<F>(
        &mut self,
        control: SeatSafeLever,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SeatSafeLever { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sel_light<F>(
        &mut self,
        control: SelLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::SelLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_set_flag1<F>(
        &mut self,
        control: SetFlag1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SetFlag1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_set_flag2<F>(
        &mut self,
        control: SetFlag2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SetFlag2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_set_flag3<F>(
        &mut self,
        control: SetFlag3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SetFlag3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_set_flag4<F>(
        &mut self,
        control: SetFlag4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SetFlag4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_set_flag5<F>(
        &mut self,
        control: SetFlag5,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SetFlag5 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_set_flag6<F>(
        &mut self,
        control: SetFlag6,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SetFlag6 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_set_flag7<F>(
        &mut self,
        control: SetFlag7,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SetFlag7 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sidepanel_lights<F>(
        &mut self,
        control: SidepanelLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SidepanelLights { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_skid_wlight<F>(
        &mut self,
        control: SkidWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::SkidWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_slip_ind<F>(
        &mut self,
        control: SlipInd,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SlipInd { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_slip_turn_flag<F>(
        &mut self,
        control: SlipTurnFlag,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SlipTurnFlag { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_spd_brk_wlight<F>(
        &mut self,
        control: SpdBrkWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::SpdBrkWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stab1<F>(
        &mut self,
        control: Stab1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Stab1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stab2<F>(
        &mut self,
        control: Stab2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Stab2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stab_arrow<F>(
        &mut self,
        control: StabArrow,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StabArrow { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stby_tr_wlight<F>(
        &mut self,
        control: StbyTrWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::StbyTrWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sto_lever<F>(
        &mut self,
        control: StoLever,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StoLever { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sto_light<F>(
        &mut self,
        control: StoLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::StoLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stopwatch_lap<F>(
        &mut self,
        control: StopwatchLap,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StopwatchLap { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stopwatch_start<F>(
        &mut self,
        control: StopwatchStart,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StopwatchStart { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_temp_control<F>(
        &mut self,
        control: TempControl,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TempControl { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_thrott_cutoff_lv<F>(
        &mut self,
        control: ThrottCutoffLv,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ThrottCutoffLv { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_throttle_friction<F>(
        &mut self,
        control: ThrottleFriction,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ThrottleFriction { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tot_fuel1<F>(
        &mut self,
        control: TotFuel1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TotFuel1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tot_fuel2<F>(
        &mut self,
        control: TotFuel2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TotFuel2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tot_fuel3<F>(
        &mut self,
        control: TotFuel3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TotFuel3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tot_fuel4<F>(
        &mut self,
        control: TotFuel4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TotFuel4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tot_fuel5<F>(
        &mut self,
        control: TotFuel5,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TotFuel5 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_trim_mode<F>(
        &mut self,
        control: TrimMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TrimMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_turn_indi<F>(
        &mut self,
        control: TurnIndi,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TurnIndi { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_alt<F>(
        &mut self,
        control: UfcAlt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcAlt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_b0<F>(
        &mut self,
        control: UfcB0,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcB0 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_b1<F>(
        &mut self,
        control: UfcB1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcB1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_b2<F>(
        &mut self,
        control: UfcB2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcB2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_b3<F>(
        &mut self,
        control: UfcB3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcB3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_b4<F>(
        &mut self,
        control: UfcB4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcB4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_b5<F>(
        &mut self,
        control: UfcB5,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcB5 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_b6<F>(
        &mut self,
        control: UfcB6,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcB6 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_b7<F>(
        &mut self,
        control: UfcB7,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcB7 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_b8<F>(
        &mut self,
        control: UfcB8,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcB8 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_b9<F>(
        &mut self,
        control: UfcB9,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcB9 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_beacon<F>(
        &mut self,
        control: UfcBeacon,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcBeacon { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_bright<F>(
        &mut self,
        control: UfcBright,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcBright { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_clear<F>(
        &mut self,
        control: UfcClear,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcClear { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_com1_pull<F>(
        &mut self,
        control: UfcCom1Pull,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcCom1Pull { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_com1_sel<F>(
        &mut self,
        control: UfcCom1Sel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcCom1Sel { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_com1_vol<F>(
        &mut self,
        control: UfcCom1Vol,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcCom1Vol { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_com2_pull<F>(
        &mut self,
        control: UfcCom2Pull,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcCom2Pull { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_com2_sel<F>(
        &mut self,
        control: UfcCom2Sel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcCom2Sel { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_com2_vol<F>(
        &mut self,
        control: UfcCom2Vol,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcCom2Vol { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_comm1_display<F>(
        &mut self,
        control: UfcComm1Display,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcComm1Display { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_comm2_display<F>(
        &mut self,
        control: UfcComm2Display,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcComm2Display { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_dash<F>(
        &mut self,
        control: UfcDash,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcDash { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_dot<F>(
        &mut self,
        control: UfcDot,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcDot { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_emcom<F>(
        &mut self,
        control: UfcEmcom,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcEmcom { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_enter<F>(
        &mut self,
        control: UfcEnter,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcEnter { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_iff<F>(
        &mut self,
        control: UfcIff,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcIff { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_ip<F>(
        &mut self,
        control: UfcIp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcIp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_onoff<F>(
        &mut self,
        control: UfcOnoff,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcOnoff { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_save<F>(
        &mut self,
        control: UfcSave,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcSave { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_scratchpad<F>(
        &mut self,
        control: UfcScratchpad,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcScratchpad { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_tacan<F>(
        &mut self,
        control: UfcTacan,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcTacan { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_target<F>(
        &mut self,
        control: UfcTarget,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcTarget { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_timer<F>(
        &mut self,
        control: UfcTimer,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcTimer { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_way<F>(
        &mut self,
        control: UfcWay,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcWay { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_weapon<F>(
        &mut self,
        control: UfcWeapon,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcWeapon { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_weather<F>(
        &mut self,
        control: UfcWeather,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcWeather { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_anc_point<F>(
        &mut self,
        control: UhfAncPoint,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfAncPoint { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_anc_sw<F>(
        &mut self,
        control: UhfAncSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfAncSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_freq<F>(
        &mut self,
        control: UhfFreq,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFreq { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_load_sw<F>(
        &mut self,
        control: UhfLoadSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfLoadSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_mode<F>(
        &mut self,
        control: UhfMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfMode { control, callback })
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

    pub fn register_volt_meter<F>(
        &mut self,
        control: VoltMeter,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VoltMeter { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vstol_light<F>(
        &mut self,
        control: VstolLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::VstolLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vstol_sel<F>(
        &mut self,
        control: VstolSel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VstolSel { control, callback })
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

    pub fn register_wsld_wlight<F>(
        &mut self,
        control: WsldWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::WsldWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_yaw_trim_sw<F>(
        &mut self,
        control: YawTrimSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::YawTrimSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_yaw_wlight<F>(
        &mut self,
        control: YawWlight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::YawWlight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn build(self) -> Av8bna<'a, MAX_CONTROLS, MAX_PACKET_DATA> {
        Av8bna {
            parser: ProtocolParser::new(),
            controls: self.controls,
            saw_frame: false,
        }
    }
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> Default
    for Av8bnaBuilder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
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

pub struct Av8bna<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    parser: ProtocolParser<MAX_PACKET_DATA>,
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    saw_frame: bool,
}

impl<const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    Av8bna<'_, MAX_CONTROLS, MAX_PACKET_DATA>
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
