//! Generated from model-definitions/P-51D.json.
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

/// Accelerometer Main
/// Output: gauge position
pub struct AccelerometerMain(AnalogGauge);
impl AccelerometerMain {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20588, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AccelerometerMain {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AccelerometerMain {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Accelerometer Max
/// Output: gauge position
pub struct AccelerometerMax(AnalogGauge);
impl AccelerometerMax {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20592, 65535, 0))
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
pub struct AccelerometerMin(AnalogGauge);
impl AccelerometerMin {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20590, 65535, 0))
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

/// G-meter Reset
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AccelerometerRst(Selector);
impl AccelerometerRst {
    pub const fn new() -> Self {
        Self(Selector::new(20586, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AccelerometerRst {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AccelerometerRst {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Accelerometer
/// Output: Accelerometer
pub struct AccelerometerValue(Metadata);
impl AccelerometerValue {
    pub const fn new() -> Self {
        Self(Metadata::new(20668, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AccelerometerValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AccelerometerValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Artificial Horizon - Bank
/// Output: gauge position
pub struct AhorizonBank(AnalogGauge);
impl AhorizonBank {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20542, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AhorizonBank {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AhorizonBank {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Artificial Horizon - Caged
/// Output: gauge position
pub struct AhorizonCaged(AnalogGauge);
impl AhorizonCaged {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20548, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AhorizonCaged {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AhorizonCaged {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Artificial Horizon - Pitch
/// Output: gauge position
pub struct AhorizonPitch(AnalogGauge);
impl AhorizonPitch {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20544, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AhorizonPitch {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AhorizonPitch {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Artificial Horizon - Pitch Shift
/// Output: gauge position
pub struct AhorizonPitchShift(AnalogGauge);
impl AhorizonPitchShift {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20546, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AhorizonPitchShift {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AhorizonPitchShift {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Aileron Trim
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct AileronTrim(AnalogDial);
impl AileronTrim {
    pub const fn new() -> Self {
        Self(AnalogDial::new(20516, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AileronTrim {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AileronTrim {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Airspeed MPH
/// Output: Airspeed MPH
pub struct AirspeedMphValue(Metadata);
impl AirspeedMphValue {
    pub const fn new() -> Self {
        Self(Metadata::new(20632, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AirspeedMphValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AirspeedMphValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Airspeed Needle
/// Output: gauge position
pub struct AirspeedNeedle(AnalogGauge);
impl AirspeedNeedle {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20528, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AirspeedNeedle {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AirspeedNeedle {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altimeter 10000 Foot
/// Output: gauge position
pub struct Altimeter10000Foot(AnalogGauge);
impl Altimeter10000Foot {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20538, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Altimeter10000Foot {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Altimeter10000Foot {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altimeter 1000 Foot
/// Output: gauge position
pub struct Altimeter1000Foot(AnalogGauge);
impl Altimeter1000Foot {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20536, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Altimeter1000Foot {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Altimeter1000Foot {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altimeter 100 Foot
/// Output: gauge position
pub struct Altimeter100Foot(AnalogGauge);
impl Altimeter100Foot {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20534, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Altimeter100Foot {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Altimeter100Foot {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altimeter Pressure
/// Output: gauge position
pub struct AltimeterPressure(AnalogGauge);
impl AltimeterPressure {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20532, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltimeterPressure {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltimeterPressure {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Altimeter
/// Output: Altimeter
pub struct AltimeterValue(Metadata);
impl AltimeterValue {
    pub const fn new() -> Self {
        Self(Metadata::new(20634, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltimeterValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltimeterValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Amber Recognition Light Key/Off/Steady
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct AmbrRecLt(Selector);
impl AmbrRecLt {
    pub const fn new() -> Self {
        Self(Selector::new(20480, 12288, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AmbrRecLt {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AmbrRecLt {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Ammeter
/// Output: gauge position
pub struct Ammeter(AnalogGauge);
impl Ammeter {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20614, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Ammeter {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Ammeter {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Ammeter
/// Output: Ammeter
pub struct AmmeterValue(Metadata);
impl AmmeterValue {
    pub const fn new() -> Self {
        Self(Metadata::new(20656, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AmmeterValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AmmeterValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Arm rest
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct ArmRest(Selector);
impl ArmRest {
    pub const fn new() -> Self {
        Self(Selector::new(20494, 8, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ArmRest {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ArmRest {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Barometric Pressure
/// Output: Barometric Pressure
pub struct BarometricPressureValue(Metadata);
impl BarometricPressureValue {
    pub const fn new() -> Self {
        Self(Metadata::new(20666, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BarometricPressureValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BarometricPressureValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Battery
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Bat(Selector);
impl Bat {
    pub const fn new() -> Self {
        Self(Selector::new(20480, 2, 1))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Bat {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Bat {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Artificial Horizon Cage
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct CageButton(Selector);
impl CageButton {
    pub const fn new() -> Self {
        Self(Selector::new(20526, 512, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CageButton {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CageButton {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Artificial Horizon Cage Rotary
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct CageRotary(LimitedDial);
impl CageRotary {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20552, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CageRotary {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CageRotary {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Canopy Emergency Release Handle
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct CanopyEmergencyReleaseHandle(Selector);
impl CanopyEmergencyReleaseHandle {
    pub const fn new() -> Self {
        Self(Selector::new(20494, 16, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CanopyEmergencyReleaseHandle {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CanopyEmergencyReleaseHandle {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Canopy Hand Crank
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct CanopyHandCrank(LimitedDial);
impl CanopyHandCrank {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20504, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CanopyHandCrank {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CanopyHandCrank {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Carburetor Cold Air Control
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct CarbColdAir(LimitedDial);
impl CarbColdAir {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20490, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CarbColdAir {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CarbColdAir {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Carburetor Temperature
/// Output: gauge position
pub struct CarbTemp(AnalogGauge);
impl CarbTemp {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20606, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CarbTemp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CarbTemp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Carburetor Warm Air Control
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct CarbWarmAir(LimitedDial);
impl CarbWarmAir {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20492, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CarbWarmAir {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CarbWarmAir {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Circuit Protectors Reset
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct CircProRst(Selector);
impl CircProRst {
    pub const fn new() -> Self {
        Self(Selector::new(20480, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CircProRst {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CircProRst {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Clock Adjust
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct ClockAdjust(AnalogDial);
impl ClockAdjust {
    pub const fn new() -> Self {
        Self(AnalogDial::new(20560, 65535, 0))
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

/// Clock Adjust Button
/// Input: switch to previous or next state
/// Output: selector position
pub struct ClockAdjustButton(Selector);
impl ClockAdjustButton {
    pub const fn new() -> Self {
        Self(Selector::new(20526, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ClockAdjustButton {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClockAdjustButton {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Clock Hours
/// Output: gauge position
pub struct ClockHours(AnalogGauge);
impl ClockHours {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20562, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ClockHours {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClockHours {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Clock Minutes
/// Output: gauge position
pub struct ClockMinutes(AnalogGauge);
impl ClockMinutes {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20564, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ClockMinutes {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClockMinutes {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Clock Seconds
/// Output: gauge position
pub struct ClockSeconds(AnalogGauge);
impl ClockSeconds {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20566, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ClockSeconds {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClockSeconds {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Cockpit Lights
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct CockpitLights(LimitedDial);
impl CockpitLights {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20524, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CockpitLights {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CockpitLights {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Cold Air
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct ColdAir(LimitedDial);
impl ColdAir {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20510, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ColdAir {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ColdAir {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Coolant Control
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct CoolantControl(Selector);
impl CoolantControl {
    pub const fn new() -> Self {
        Self(Selector::new(20484, 6144, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CoolantControl {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CoolantControl {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Coolant Control Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct CoolantControlCover(Selector);
impl CoolantControlCover {
    pub const fn new() -> Self {
        Self(Selector::new(20484, 1024, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CoolantControlCover {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CoolantControlCover {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Coolant Temperature
/// Output: gauge position
pub struct CoolantTemp(AnalogGauge);
impl CoolantTemp {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20608, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CoolantTemp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CoolantTemp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Remote Compass Course
/// Output: Remote Compass Course
pub struct CourseValue(Metadata);
impl CourseValue {
    pub const fn new() -> Self {
        Self(Metadata::new(20642, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CourseValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CourseValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Defroster
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct Defroster(LimitedDial);
impl Defroster {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20508, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Defroster {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Defroster {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Detrola Frequency Selector
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct DetrolaFrequency(LimitedDial);
impl DetrolaFrequency {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20582, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DetrolaFrequency {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DetrolaFrequency {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Detrola Volume
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct DetrolaVolume(LimitedDial);
impl DetrolaVolume {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20584, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DetrolaVolume {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DetrolaVolume {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Directional Gyro Heading
/// Output: gauge position
pub struct DirectionalGyro(AnalogGauge);
impl DirectionalGyro {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20554, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DirectionalGyro {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DirectionalGyro {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Directional Gyro
/// Output: Directional Gyro
pub struct DirectionalGyroValue(Metadata);
impl DirectionalGyroValue {
    pub const fn new() -> Self {
        Self(Metadata::new(20638, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DirectionalGyroValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DirectionalGyroValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Elevator Trim
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct ElevatorTrim(AnalogDial);
impl ElevatorTrim {
    pub const fn new() -> Self {
        Self(AnalogDial::new(20518, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ElevatorTrim {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ElevatorTrim {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Engine RPM
/// Output: gauge position
pub struct EngineRpm(AnalogGauge);
impl EngineRpm {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20594, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EngineRpm {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EngineRpm {
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
        Self(Metadata::new(20636, 65535, 0))
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

/// Formation Lights
/// Output: Formation Lights
pub struct ExtFormationLights(Metadata);
impl ExtFormationLights {
    pub const fn new() -> Self {
        Self(Metadata::new(20630, 65535, 0))
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

/// Left Position Light (red)
/// Output: Left Position Light (red)
pub struct ExtPositionLightLeft(Metadata);
impl ExtPositionLightLeft {
    pub const fn new() -> Self {
        Self(Metadata::new(20586, 8192, 13))
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
        Self(Metadata::new(20586, 16384, 14))
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

/// Flaps Control Handle
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct FlapsControlHandle(LimitedDial);
impl FlapsControlHandle {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20522, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FlapsControlHandle {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FlapsControlHandle {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuel Booster On/Off
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FuelBooster(Selector);
impl FuelBooster {
    pub const fn new() -> Self {
        Self(Selector::new(20526, 16, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelBooster {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelBooster {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuel Pressure
/// Output: gauge position
pub struct FuelPressure(AnalogGauge);
impl FuelPressure {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20596, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelPressure {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelPressure {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuel Pressure
/// Output: Fuel Pressure
pub struct FuelPressureValue(Metadata);
impl FuelPressureValue {
    pub const fn new() -> Self {
        Self(Metadata::new(20644, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelPressureValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelPressureValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuel Selector Valve
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct FuelSelectorValve(Selector);
impl FuelSelectorValve {
    pub const fn new() -> Self {
        Self(Selector::new(20494, 1792, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelSelectorValve {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelSelectorValve {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuel Shut-Off Valve
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FuelShutOffValve(Selector);
impl FuelShutOffValve {
    pub const fn new() -> Self {
        Self(Selector::new(20494, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelShutOffValve {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelShutOffValve {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuel Tank Fuselage
/// Output: gauge position
pub struct FuelTankFuselage(AnalogGauge);
impl FuelTankFuselage {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20628, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelTankFuselage {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelTankFuselage {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuel Tank Left
/// Output: gauge position
pub struct FuelTankLeft(AnalogGauge);
impl FuelTankLeft {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20624, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelTankLeft {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelTankLeft {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuel Tank Right
/// Output: gauge position
pub struct FuelTankRight(AnalogGauge);
impl FuelTankRight {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20626, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelTankRight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelTankRight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fuselage Fuel Tank Gallons
/// Output: Fuselage Fuel Tank Gallons
pub struct FuselageFuelTankValue(Metadata);
impl FuselageFuelTankValue {
    pub const fn new() -> Self {
        Self(Metadata::new(20664, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuselageFuelTankValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuselageFuelTankValue {
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
pub struct Gen(Selector);
impl Gen {
    pub const fn new() -> Self {
        Self(Selector::new(20480, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Gen {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Gen {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Green Recognition Light Key/Off/Steady
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct GrnRecLt(Selector);
impl GrnRecLt {
    pub const fn new() -> Self {
        Self(Selector::new(20480, 3072, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for GrnRecLt {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for GrnRecLt {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Gun/Camera Control
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct GunControl(Selector);
impl GunControl {
    pub const fn new() -> Self {
        Self(Selector::new(20514, 192, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for GunControl {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for GunControl {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Gun Heating
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct GunHeat(Selector);
impl GunHeat {
    pub const fn new() -> Self {
        Self(Selector::new(20480, 4, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for GunHeat {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for GunHeat {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Gun Sight Brightness
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct GunsightBrt(LimitedDial);
impl GunsightBrt {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20486, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for GunsightBrt {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for GunsightBrt {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fixed Fixed-Gyro Gyro
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct GunsightFixedGyro(Selector);
impl GunsightFixedGyro {
    pub const fn new() -> Self {
        Self(Selector::new(20484, 12, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for GunsightFixedGyro {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for GunsightFixedGyro {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Gunsight On/Off
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct GunsightOnOff(Selector);
impl GunsightOnOff {
    pub const fn new() -> Self {
        Self(Selector::new(20484, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for GunsightOnOff {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for GunsightOnOff {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Directional Gyro Cage
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct HdgCageButton(Selector);
impl HdgCageButton {
    pub const fn new() -> Self {
        Self(Selector::new(20526, 1024, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HdgCageButton {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HdgCageButton {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Directional Gyro Heading Set
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct HdgCageRotary(AnalogDial);
impl HdgCageRotary {
    pub const fn new() -> Self {
        Self(AnalogDial::new(20556, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HdgCageRotary {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HdgCageRotary {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Remote Compass Heading
/// Output: Remote Compass Heading
pub struct HeadingValue(Metadata);
impl HeadingValue {
    pub const fn new() -> Self {
        Self(Metadata::new(20640, 65535, 0))
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

/// High Blower Lamp
/// Output: 0 if light is off, 1 if light is on
pub struct HighBlowerLamp(Led);
impl HighBlowerLamp {
    pub const fn new() -> Self {
        Self(Led::new(20526, 8, 3))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for HighBlowerLamp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HighBlowerLamp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// High Blower Lamp Test
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct HighBlowerTestLamp(Selector);
impl HighBlowerTestLamp {
    pub const fn new() -> Self {
        Self(Selector::new(20526, 4, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HighBlowerTestLamp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HighBlowerTestLamp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Homing Adapter Circuit Breaker
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct HomingAdapterCb(Selector);
impl HomingAdapterCb {
    pub const fn new() -> Self {
        Self(Selector::new(20586, 512, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HomingAdapterCb {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HomingAdapterCb {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Homing Adapter Mode
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct HomingAdapterMode(Selector);
impl HomingAdapterMode {
    pub const fn new() -> Self {
        Self(Selector::new(20586, 3072, 10))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HomingAdapterMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HomingAdapterMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Homing Adapter On/Off
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct HomingAdapterPower(Selector);
impl HomingAdapterPower {
    pub const fn new() -> Self {
        Self(Selector::new(20586, 256, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HomingAdapterPower {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HomingAdapterPower {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Hot Air
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct HotAir(LimitedDial);
impl HotAir {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20512, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HotAir {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HotAir {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Hydraulic Release Knob
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct HydRel(Selector);
impl HydRel {
    pub const fn new() -> Self {
        Self(Selector::new(20484, 16, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HydRel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HydRel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Hydraulic Pressure
/// Output: gauge position
pub struct HydraulicPressure(AnalogGauge);
impl HydraulicPressure {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20598, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HydraulicPressure {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HydraulicPressure {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Hydraulic Pressure
/// Output: Hydraulic Pressure
pub struct HydraulicPressureValue(Metadata);
impl HydraulicPressureValue {
    pub const fn new() -> Self {
        Self(Metadata::new(20646, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HydraulicPressureValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HydraulicPressureValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// IFF Code 1-6
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct IffCode(Selector);
impl IffCode {
    pub const fn new() -> Self {
        Self(Selector::new(20586, 224, 5))
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

/// IFF Detonator Circuit On/Off
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct IffDetCircuit(Selector);
impl IffDetCircuit {
    pub const fn new() -> Self {
        Self(Selector::new(20586, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for IffDetCircuit {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IffDetCircuit {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// IFF Detonator Left
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct IffDetLeft(Selector);
impl IffDetLeft {
    pub const fn new() -> Self {
        Self(Selector::new(20586, 2, 1))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for IffDetLeft {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IffDetLeft {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// IFF Detonator Right
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct IffDetRight(Selector);
impl IffDetRight {
    pub const fn new() -> Self {
        Self(Selector::new(20586, 4, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for IffDetRight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IffDetRight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// IFF Distress Signal On/Off
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct IffDistress(Selector);
impl IffDistress {
    pub const fn new() -> Self {
        Self(Selector::new(20574, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for IffDistress {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IffDistress {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// IFF Time/Off/On
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct IffTimeOffOn(Selector);
impl IffTimeOffOn {
    pub const fn new() -> Self {
        Self(Selector::new(20586, 24, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for IffTimeOffOn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IffTimeOffOn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Ignition Off/Right/Left/Both
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct Ignition(Selector);
impl Ignition {
    pub const fn new() -> Self {
        Self(Selector::new(20514, 48, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Ignition {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Ignition {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Landing Gear Control Handle
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct LandingGearContr0lHandle(Selector);
impl LandingGearContr0lHandle {
    pub const fn new() -> Self {
        Self(Selector::new(20514, 4, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LandingGearContr0lHandle {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LandingGearContr0lHandle {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Landing Gear Green Light
/// Output: 0 if light is off, 1 if light is on
pub struct LandingGearGreen(Led);
impl LandingGearGreen {
    pub const fn new() -> Self {
        Self(Led::new(20494, 8192, 13))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for LandingGearGreen {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LandingGearGreen {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Landing Gear Red Light
/// Output: 0 if light is off, 1 if light is on
pub struct LandingGearRed(Led);
impl LandingGearRed {
    pub const fn new() -> Self {
        Self(Led::new(20494, 16384, 14))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for LandingGearRed {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LandingGearRed {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Landing Light On/Off
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct LandingLight(Selector);
impl LandingLight {
    pub const fn new() -> Self {
        Self(Selector::new(20494, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LandingLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LandingLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Left Bomb Arm/Chemical Release
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct LeftBombArmChem(Selector);
impl LeftBombArmChem {
    pub const fn new() -> Self {
        Self(Selector::new(20514, 1536, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LeftBombArmChem {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LeftBombArmChem {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Left Fluorescent Light
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct LeftFluorescentLight(LimitedDial);
impl LeftFluorescentLight {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20506, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LeftFluorescentLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LeftFluorescentLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Left Fuel Tank Gallons
/// Output: Left Fuel Tank Gallons
pub struct LeftFuelTankValue(Metadata);
impl LeftFuelTankValue {
    pub const fn new() -> Self {
        Self(Metadata::new(20660, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LeftFuelTankValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LeftFuelTankValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Left Payload Salvo
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct LftPylodSlvo(Selector);
impl LftPylodSlvo {
    pub const fn new() -> Self {
        Self(Selector::new(20484, 256, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LftPylodSlvo {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LftPylodSlvo {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Lock Propeller & Mixture
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct LockPropellerMixture(LimitedDial);
impl LockPropellerMixture {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20502, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LockPropellerMixture {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LockPropellerMixture {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Lock Stick Forward/Neutral
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct LockStickFrwdNeut(Selector);
impl LockStickFrwdNeut {
    pub const fn new() -> Self {
        Self(Selector::new(20514, 8, 3))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LockStickFrwdNeut {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LockStickFrwdNeut {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Lock Throttle
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct LockThrottle(LimitedDial);
impl LockThrottle {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20500, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LockThrottle {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LockThrottle {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Manifold Pressure
/// Output: gauge position
pub struct ManifoldPressure(AnalogGauge);
impl ManifoldPressure {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20600, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ManifoldPressure {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ManifoldPressure {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Manifold Pressure
/// Output: Manifold Pressure
pub struct ManifoldPressureValue(Metadata);
impl ManifoldPressureValue {
    pub const fn new() -> Self {
        Self(Metadata::new(20648, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ManifoldPressureValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ManifoldPressureValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Microphone On
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MicrophoneOn(Selector);
impl MicrophoneOn {
    pub const fn new() -> Self {
        Self(Selector::new(20494, 4, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MicrophoneOn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MicrophoneOn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Mixture Control
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct MixtureControl(Selector);
impl MixtureControl {
    pub const fn new() -> Self {
        Self(Selector::new(20494, 3, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MixtureControl {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MixtureControl {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// IFF Power On/Off
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct OffPower(Selector);
impl OffPower {
    pub const fn new() -> Self {
        Self(Selector::new(20574, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OffPower {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OffPower {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Oil Control
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct OilControl(Selector);
impl OilControl {
    pub const fn new() -> Self {
        Self(Selector::new(20484, 49152, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OilControl {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OilControl {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Oil Control Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct OilControlCover(Selector);
impl OilControlCover {
    pub const fn new() -> Self {
        Self(Selector::new(20484, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OilControlCover {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OilControlCover {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Oil Dilute Activate
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct OilDilute(Selector);
impl OilDilute {
    pub const fn new() -> Self {
        Self(Selector::new(20526, 32, 5))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OilDilute {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OilDilute {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Oil Pressure
/// Output: gauge position
pub struct OilPres(AnalogGauge);
impl OilPres {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20612, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OilPres {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OilPres {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Oil Pressure
/// Output: Oil Pressure
pub struct OilPressureValue(Metadata);
impl OilPressureValue {
    pub const fn new() -> Self {
        Self(Metadata::new(20654, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OilPressureValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OilPressureValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Oil Temperature
/// Output: gauge position
pub struct OilTemp(AnalogGauge);
impl OilTemp {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20610, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OilTemp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OilTemp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Oil Temperature
/// Output: Oil Temperature
pub struct OilTemperatureValue(Metadata);
impl OilTemperatureValue {
    pub const fn new() -> Self {
        Self(Metadata::new(20652, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OilTemperatureValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OilTemperatureValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Auto-Mix On-Off
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct OxygenAutoMix(Selector);
impl OxygenAutoMix {
    pub const fn new() -> Self {
        Self(Selector::new(20494, 32, 5))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OxygenAutoMix {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OxygenAutoMix {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Oxygen Emergency By-pass
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct OxygenByPass(Selector);
impl OxygenByPass {
    pub const fn new() -> Self {
        Self(Selector::new(20494, 64, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OxygenByPass {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OxygenByPass {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Oxygen Flow Blinker
/// Output: gauge position
pub struct OxygenFlow(AnalogGauge);
impl OxygenFlow {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20620, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OxygenFlow {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OxygenFlow {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Oxygen Pressure
/// Output: gauge position
pub struct OxygenPres(AnalogGauge);
impl OxygenPres {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20618, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OxygenPres {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OxygenPres {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Oxygen Pressure
/// Output: Oxygen Pressure
pub struct OxygenPressureValue(Metadata);
impl OxygenPressureValue {
    pub const fn new() -> Self {
        Self(Metadata::new(20658, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OxygenPressureValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OxygenPressureValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Oxygen Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct OxygenWarning(Led);
impl OxygenWarning {
    pub const fn new() -> Self {
        Self(Led::new(20494, 128, 7))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for OxygenWarning {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OxygenWarning {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Parking Brake Handle
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct ParkBrakeHandle(Selector);
impl ParkBrakeHandle {
    pub const fn new() -> Self {
        Self(Selector::new(20514, 2, 1))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ParkBrakeHandle {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ParkBrakeHandle {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Pitch Adjustment
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct PitchAdjust(AnalogDial);
impl PitchAdjust {
    pub const fn new() -> Self {
        Self(AnalogDial::new(20550, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PitchAdjust {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PitchAdjust {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Pitot Heating
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Pitot(Selector);
impl Pitot {
    pub const fn new() -> Self {
        Self(Selector::new(20480, 8, 3))
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

/// Primer Activate
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Primer(Selector);
impl Primer {
    pub const fn new() -> Self {
        Self(Selector::new(20526, 256, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Primer {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Primer {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Propeller RPM
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct PropellerRpm(LimitedDial);
impl PropellerRpm {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20498, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PropellerRpm {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PropellerRpm {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Radar Warning Light
/// Output: 0 if light is off, 1 if light is on
pub struct RadarWarningLight(Led);
impl RadarWarningLight {
    pub const fn new() -> Self {
        Self(Led::new(20574, 8192, 13))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for RadarWarningLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadarWarningLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Radio Lights Dimmer
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct RadioLightsDimmer(LimitedDial);
impl RadioLightsDimmer {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20576, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadioLightsDimmer {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadioLightsDimmer {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Radio Mode2
/// Input: switch to previous or next state
/// Output: selector position
pub struct RadioMode2(Selector);
impl RadioMode2 {
    pub const fn new() -> Self {
        Self(Selector::new(20574, 12, 2))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadioMode2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadioMode2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Radio Mode3
/// Input: switch to previous or next state
/// Output: selector position
pub struct RadioMode3(Selector);
impl RadioMode3 {
    pub const fn new() -> Self {
        Self(Selector::new(20574, 48, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadioMode3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadioMode3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Radio Audio Volume
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct RadioVolume(LimitedDial);
impl RadioVolume {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20578, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadioVolume {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadioVolume {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Recognition Lights Key
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RecLtsKey(Selector);
impl RecLtsKey {
    pub const fn new() -> Self {
        Self(Selector::new(20480, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RecLtsKey {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RecLtsKey {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Red Recognition Light Key/Off/Steady
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct RedRecLt(Selector);
impl RedRecLt {
    pub const fn new() -> Self {
        Self(Selector::new(20480, 768, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RedRecLt {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RedRecLt {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Remote Compass Course
/// Output: gauge position
pub struct RemoteCompassCrs(AnalogGauge);
impl RemoteCompassCrs {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20572, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RemoteCompassCrs {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RemoteCompassCrs {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Remote Compass Heading
/// Output: gauge position
pub struct RemoteCompassHdg(AnalogGauge);
impl RemoteCompassHdg {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20570, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RemoteCompassHdg {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RemoteCompassHdg {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Course Set
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct RemoteCompassSet(AnalogDial);
impl RemoteCompassSet {
    pub const fn new() -> Self {
        Self(AnalogDial::new(20568, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RemoteCompassSet {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RemoteCompassSet {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Fixed Reticle Mask Lever
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RetMaskLvr(Selector);
impl RetMaskLvr {
    pub const fn new() -> Self {
        Self(Selector::new(20484, 2, 1))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RetMaskLvr {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RetMaskLvr {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Right Bomb Arm/Chemical Release
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct RightBombArmChem(Selector);
impl RightBombArmChem {
    pub const fn new() -> Self {
        Self(Selector::new(20514, 6144, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RightBombArmChem {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RightBombArmChem {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Right Fuel Tank Gallons
/// Output: Right Fuel Tank Gallons
pub struct RightFuelTankValue(Metadata);
impl RightFuelTankValue {
    pub const fn new() -> Self {
        Self(Metadata::new(20662, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RightFuelTankValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RightFuelTankValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Rockets Delay Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RktDelSwitch(Selector);
impl RktDelSwitch {
    pub const fn new() -> Self {
        Self(Selector::new(20484, 128, 7))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RktDelSwitch {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RktDelSwitch {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Rockets Release Mode
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct RktRelMode(Selector);
impl RktRelMode {
    pub const fn new() -> Self {
        Self(Selector::new(20484, 96, 5))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RktRelMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RktRelMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Rocket Counter
/// Output: gauge position
pub struct RocketCounter(AnalogGauge);
impl RocketCounter {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20602, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RocketCounter {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RocketCounter {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Rockets/Bombs Modes
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct RocketsBombsModes(Selector);
impl RocketsBombsModes {
    pub const fn new() -> Self {
        Self(Selector::new(20514, 24576, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RocketsBombsModes {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RocketsBombsModes {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Right Fluorescent Light
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct RtFlresLt(LimitedDial);
impl RtFlresLt {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20482, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RtFlresLt {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RtFlresLt {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Right Payload Salvo
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RtPylodSlvo(Selector);
impl RtPylodSlvo {
    pub const fn new() -> Self {
        Self(Selector::new(20484, 512, 9))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RtPylodSlvo {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RtPylodSlvo {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Rudder Trim
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct RudderTrim(AnalogDial);
impl RudderTrim {
    pub const fn new() -> Self {
        Self(AnalogDial::new(20520, 65535, 0))
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

/// Safe Landing Gear Light Test
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct SafeLndGearLtTest(Selector);
impl SafeLndGearLtTest {
    pub const fn new() -> Self {
        Self(Selector::new(20494, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SafeLndGearLtTest {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SafeLndGearLtTest {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Set Pressure
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct SetPressure(AnalogDial);
impl SetPressure {
    pub const fn new() -> Self {
        Self(AnalogDial::new(20530, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SetPressure {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SetPressure {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Silence Landing Gear Warning Horn Cut Off
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct SilenceGearHorn(Selector);
impl SilenceGearHorn {
    pub const fn new() -> Self {
        Self(Selector::new(20514, 256, 8))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SilenceGearHorn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SilenceGearHorn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Slipball
/// Output: gauge position
pub struct Slipball(AnalogGauge);
impl Slipball {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20622, 65535, 0))
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

/// Starter Activate
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Starter(Selector);
impl Starter {
    pub const fn new() -> Self {
        Self(Selector::new(20526, 64, 6))
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

/// Starter Switch Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct StarterCover(Selector);
impl StarterCover {
    pub const fn new() -> Self {
        Self(Selector::new(20526, 128, 7))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for StarterCover {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for StarterCover {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Supercharger Auto/Low/High
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct SuperchargerAutoLowHigh(Selector);
impl SuperchargerAutoLowHigh {
    pub const fn new() -> Self {
        Self(Selector::new(20526, 3, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SuperchargerAutoLowHigh {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SuperchargerAutoLowHigh {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Supercharger Switch Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct SuperchargerSwitchCover(Selector);
impl SuperchargerSwitchCover {
    pub const fn new() -> Self {
        Self(Selector::new(20514, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SuperchargerSwitchCover {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SuperchargerSwitchCover {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Tail Position Lights Bright/Off/Dim
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct TailLts(Selector);
impl TailLts {
    pub const fn new() -> Self {
        Self(Selector::new(20480, 192, 6))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TailLts {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TailLts {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Throttle
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct Throttle(LimitedDial);
impl Throttle {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20496, 65535, 0))
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

/// Turn Indicator
/// Output: gauge position
pub struct TurnIndicator(AnalogGauge);
impl TurnIndicator {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20558, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TurnIndicator {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TurnIndicator {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Unsafe Landing Gear Light Test
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UnsafeLndGearLtTest(Selector);
impl UnsafeLndGearLtTest {
    pub const fn new() -> Self {
        Self(Selector::new(20514, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UnsafeLndGearLtTest {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UnsafeLndGearLtTest {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Vacuum System
/// Output: gauge position
pub struct VacuumSuction(AnalogGauge);
impl VacuumSuction {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20604, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VacuumSuction {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VacuumSuction {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Vacuum Suction read as X.X or XX.X
/// Output: Vacuum Suction read as X.X or XX.X
pub struct VacuumSuctionValue(Metadata);
impl VacuumSuctionValue {
    pub const fn new() -> Self {
        Self(Metadata::new(20650, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VacuumSuctionValue {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VacuumSuctionValue {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Variometer-VVI
/// Output: gauge position
pub struct VariometerVvi(AnalogGauge);
impl VariometerVvi {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20540, 65535, 0))
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

/// Switch Locking Lever
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct VhfLockingLever(Selector);
impl VhfLockingLever {
    pub const fn new() -> Self {
        Self(Selector::new(20574, 2, 1))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VhfLockingLever {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VhfLockingLever {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// VHF Radio A Light
/// Output: 0 if light is off, 1 if light is on
pub struct VhfRadioALight(Led);
impl VhfRadioALight {
    pub const fn new() -> Self {
        Self(Led::new(20574, 64, 6))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for VhfRadioALight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VhfRadioALight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// VHF Radio B Light
/// Output: 0 if light is off, 1 if light is on
pub struct VhfRadioBLight(Led);
impl VhfRadioBLight {
    pub const fn new() -> Self {
        Self(Led::new(20574, 128, 7))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for VhfRadioBLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VhfRadioBLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// VHF Radio C Light
/// Output: 0 if light is off, 1 if light is on
pub struct VhfRadioCLight(Led);
impl VhfRadioCLight {
    pub const fn new() -> Self {
        Self(Led::new(20574, 256, 8))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for VhfRadioCLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VhfRadioCLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// VHF Radio Channel A
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct VhfRadioChanA(Selector);
impl VhfRadioChanA {
    pub const fn new() -> Self {
        Self(Selector::new(20526, 8192, 13))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VhfRadioChanA {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VhfRadioChanA {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// VHF Radio Channel B
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct VhfRadioChanB(Selector);
impl VhfRadioChanB {
    pub const fn new() -> Self {
        Self(Selector::new(20526, 16384, 14))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VhfRadioChanB {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VhfRadioChanB {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// VHF Radio Channel C
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct VhfRadioChanC(Selector);
impl VhfRadioChanC {
    pub const fn new() -> Self {
        Self(Selector::new(20526, 32768, 15))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VhfRadioChanC {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VhfRadioChanC {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// VHF Radio Channel D
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct VhfRadioChanD(Selector);
impl VhfRadioChanD {
    pub const fn new() -> Self {
        Self(Selector::new(20574, 1, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VhfRadioChanD {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VhfRadioChanD {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// VHF Radio D Light
/// Output: 0 if light is off, 1 if light is on
pub struct VhfRadioDLight(Led);
impl VhfRadioDLight {
    pub const fn new() -> Self {
        Self(Led::new(20574, 512, 9))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for VhfRadioDLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VhfRadioDLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// VHF Radio On/Off
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct VhfRadioOnOff(Selector);
impl VhfRadioOnOff {
    pub const fn new() -> Self {
        Self(Selector::new(20526, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VhfRadioOnOff {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VhfRadioOnOff {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// VHF Radio TX Light
/// Output: 0 if light is off, 1 if light is on
pub struct VhfRadioTxLight(Led);
impl VhfRadioTxLight {
    pub const fn new() -> Self {
        Self(Led::new(20574, 1024, 10))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for VhfRadioTxLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VhfRadioTxLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Tail Warning Radar Light Control
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct WarningRadarLight(LimitedDial);
impl WarningRadarLight {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20580, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for WarningRadarLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for WarningRadarLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Tail Warning Radar Power
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct WarningRadarPower(Selector);
impl WarningRadarPower {
    pub const fn new() -> Self {
        Self(Selector::new(20574, 2048, 11))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for WarningRadarPower {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for WarningRadarPower {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Tail Warning Radar Test
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct WarningRadarTest(Selector);
impl WarningRadarTest {
    pub const fn new() -> Self {
        Self(Selector::new(20574, 4096, 12))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for WarningRadarTest {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for WarningRadarTest {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Windshield Oil
/// Output: gauge position
pub struct WindshieldOil(AnalogGauge);
impl WindshieldOil {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20616, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for WindshieldOil {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for WindshieldOil {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Windshield Crack Holes
/// Output: gauge position
pub struct Windshieldcracks(AnalogGauge);
impl Windshieldcracks {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(20670, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Windshieldcracks {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Windshieldcracks {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Windshield Oil Splashes
/// Output: 0 if light is off, 1 if light is on
pub struct Windshieldoil(Led);
impl Windshieldoil {
    pub const fn new() -> Self {
        Self(Led::new(20586, 32768, 15))
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Windshieldoil {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Windshieldoil {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Wing Position Lights Bright/Off/Dim
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct WingLts(Selector);
impl WingLts {
    pub const fn new() -> Self {
        Self(Selector::new(20480, 48, 4))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for WingLts {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for WingLts {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Wing Span Selector
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct WingSpanSel(LimitedDial);
impl WingSpanSel {
    pub const fn new() -> Self {
        Self(LimitedDial::new(20488, 65535, 0))
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for WingSpanSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for WingSpanSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

pub enum RegisteredControl<'a> {
    AccelerometerMain {
        control: AccelerometerMain,
        callback: IntegerCallback<'a>,
    },
    AccelerometerMax {
        control: AccelerometerMax,
        callback: IntegerCallback<'a>,
    },
    AccelerometerMin {
        control: AccelerometerMin,
        callback: IntegerCallback<'a>,
    },
    AccelerometerRst {
        control: AccelerometerRst,
        callback: IntegerCallback<'a>,
    },
    AccelerometerValue {
        control: AccelerometerValue,
        callback: IntegerCallback<'a>,
    },
    AhorizonBank {
        control: AhorizonBank,
        callback: IntegerCallback<'a>,
    },
    AhorizonCaged {
        control: AhorizonCaged,
        callback: IntegerCallback<'a>,
    },
    AhorizonPitch {
        control: AhorizonPitch,
        callback: IntegerCallback<'a>,
    },
    AhorizonPitchShift {
        control: AhorizonPitchShift,
        callback: IntegerCallback<'a>,
    },
    AileronTrim {
        control: AileronTrim,
        callback: IntegerCallback<'a>,
    },
    AirspeedMphValue {
        control: AirspeedMphValue,
        callback: IntegerCallback<'a>,
    },
    AirspeedNeedle {
        control: AirspeedNeedle,
        callback: IntegerCallback<'a>,
    },
    Altimeter10000Foot {
        control: Altimeter10000Foot,
        callback: IntegerCallback<'a>,
    },
    Altimeter1000Foot {
        control: Altimeter1000Foot,
        callback: IntegerCallback<'a>,
    },
    Altimeter100Foot {
        control: Altimeter100Foot,
        callback: IntegerCallback<'a>,
    },
    AltimeterPressure {
        control: AltimeterPressure,
        callback: IntegerCallback<'a>,
    },
    AltimeterValue {
        control: AltimeterValue,
        callback: IntegerCallback<'a>,
    },
    AmbrRecLt {
        control: AmbrRecLt,
        callback: IntegerCallback<'a>,
    },
    Ammeter {
        control: Ammeter,
        callback: IntegerCallback<'a>,
    },
    AmmeterValue {
        control: AmmeterValue,
        callback: IntegerCallback<'a>,
    },
    ArmRest {
        control: ArmRest,
        callback: IntegerCallback<'a>,
    },
    BarometricPressureValue {
        control: BarometricPressureValue,
        callback: IntegerCallback<'a>,
    },
    Bat {
        control: Bat,
        callback: IntegerCallback<'a>,
    },
    CageButton {
        control: CageButton,
        callback: IntegerCallback<'a>,
    },
    CageRotary {
        control: CageRotary,
        callback: IntegerCallback<'a>,
    },
    CanopyEmergencyReleaseHandle {
        control: CanopyEmergencyReleaseHandle,
        callback: IntegerCallback<'a>,
    },
    CanopyHandCrank {
        control: CanopyHandCrank,
        callback: IntegerCallback<'a>,
    },
    CarbColdAir {
        control: CarbColdAir,
        callback: IntegerCallback<'a>,
    },
    CarbTemp {
        control: CarbTemp,
        callback: IntegerCallback<'a>,
    },
    CarbWarmAir {
        control: CarbWarmAir,
        callback: IntegerCallback<'a>,
    },
    CircProRst {
        control: CircProRst,
        callback: IntegerCallback<'a>,
    },
    ClockAdjust {
        control: ClockAdjust,
        callback: IntegerCallback<'a>,
    },
    ClockAdjustButton {
        control: ClockAdjustButton,
        callback: IntegerCallback<'a>,
    },
    ClockHours {
        control: ClockHours,
        callback: IntegerCallback<'a>,
    },
    ClockMinutes {
        control: ClockMinutes,
        callback: IntegerCallback<'a>,
    },
    ClockSeconds {
        control: ClockSeconds,
        callback: IntegerCallback<'a>,
    },
    CockpitLights {
        control: CockpitLights,
        callback: IntegerCallback<'a>,
    },
    ColdAir {
        control: ColdAir,
        callback: IntegerCallback<'a>,
    },
    CoolantControl {
        control: CoolantControl,
        callback: IntegerCallback<'a>,
    },
    CoolantControlCover {
        control: CoolantControlCover,
        callback: IntegerCallback<'a>,
    },
    CoolantTemp {
        control: CoolantTemp,
        callback: IntegerCallback<'a>,
    },
    CourseValue {
        control: CourseValue,
        callback: IntegerCallback<'a>,
    },
    Defroster {
        control: Defroster,
        callback: IntegerCallback<'a>,
    },
    DetrolaFrequency {
        control: DetrolaFrequency,
        callback: IntegerCallback<'a>,
    },
    DetrolaVolume {
        control: DetrolaVolume,
        callback: IntegerCallback<'a>,
    },
    DirectionalGyro {
        control: DirectionalGyro,
        callback: IntegerCallback<'a>,
    },
    DirectionalGyroValue {
        control: DirectionalGyroValue,
        callback: IntegerCallback<'a>,
    },
    ElevatorTrim {
        control: ElevatorTrim,
        callback: IntegerCallback<'a>,
    },
    EngineRpm {
        control: EngineRpm,
        callback: IntegerCallback<'a>,
    },
    EngineRpmValue {
        control: EngineRpmValue,
        callback: IntegerCallback<'a>,
    },
    ExtFormationLights {
        control: ExtFormationLights,
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
    FlapsControlHandle {
        control: FlapsControlHandle,
        callback: IntegerCallback<'a>,
    },
    FuelBooster {
        control: FuelBooster,
        callback: IntegerCallback<'a>,
    },
    FuelPressure {
        control: FuelPressure,
        callback: IntegerCallback<'a>,
    },
    FuelPressureValue {
        control: FuelPressureValue,
        callback: IntegerCallback<'a>,
    },
    FuelSelectorValve {
        control: FuelSelectorValve,
        callback: IntegerCallback<'a>,
    },
    FuelShutOffValve {
        control: FuelShutOffValve,
        callback: IntegerCallback<'a>,
    },
    FuelTankFuselage {
        control: FuelTankFuselage,
        callback: IntegerCallback<'a>,
    },
    FuelTankLeft {
        control: FuelTankLeft,
        callback: IntegerCallback<'a>,
    },
    FuelTankRight {
        control: FuelTankRight,
        callback: IntegerCallback<'a>,
    },
    FuselageFuelTankValue {
        control: FuselageFuelTankValue,
        callback: IntegerCallback<'a>,
    },
    Gen {
        control: Gen,
        callback: IntegerCallback<'a>,
    },
    GrnRecLt {
        control: GrnRecLt,
        callback: IntegerCallback<'a>,
    },
    GunControl {
        control: GunControl,
        callback: IntegerCallback<'a>,
    },
    GunHeat {
        control: GunHeat,
        callback: IntegerCallback<'a>,
    },
    GunsightBrt {
        control: GunsightBrt,
        callback: IntegerCallback<'a>,
    },
    GunsightFixedGyro {
        control: GunsightFixedGyro,
        callback: IntegerCallback<'a>,
    },
    GunsightOnOff {
        control: GunsightOnOff,
        callback: IntegerCallback<'a>,
    },
    HdgCageButton {
        control: HdgCageButton,
        callback: IntegerCallback<'a>,
    },
    HdgCageRotary {
        control: HdgCageRotary,
        callback: IntegerCallback<'a>,
    },
    HeadingValue {
        control: HeadingValue,
        callback: IntegerCallback<'a>,
    },
    HighBlowerLamp {
        control: HighBlowerLamp,
        callback: LedCallback<'a>,
    },
    HighBlowerTestLamp {
        control: HighBlowerTestLamp,
        callback: IntegerCallback<'a>,
    },
    HomingAdapterCb {
        control: HomingAdapterCb,
        callback: IntegerCallback<'a>,
    },
    HomingAdapterMode {
        control: HomingAdapterMode,
        callback: IntegerCallback<'a>,
    },
    HomingAdapterPower {
        control: HomingAdapterPower,
        callback: IntegerCallback<'a>,
    },
    HotAir {
        control: HotAir,
        callback: IntegerCallback<'a>,
    },
    HydRel {
        control: HydRel,
        callback: IntegerCallback<'a>,
    },
    HydraulicPressure {
        control: HydraulicPressure,
        callback: IntegerCallback<'a>,
    },
    HydraulicPressureValue {
        control: HydraulicPressureValue,
        callback: IntegerCallback<'a>,
    },
    IffCode {
        control: IffCode,
        callback: IntegerCallback<'a>,
    },
    IffDetCircuit {
        control: IffDetCircuit,
        callback: IntegerCallback<'a>,
    },
    IffDetLeft {
        control: IffDetLeft,
        callback: IntegerCallback<'a>,
    },
    IffDetRight {
        control: IffDetRight,
        callback: IntegerCallback<'a>,
    },
    IffDistress {
        control: IffDistress,
        callback: IntegerCallback<'a>,
    },
    IffTimeOffOn {
        control: IffTimeOffOn,
        callback: IntegerCallback<'a>,
    },
    Ignition {
        control: Ignition,
        callback: IntegerCallback<'a>,
    },
    LandingGearContr0lHandle {
        control: LandingGearContr0lHandle,
        callback: IntegerCallback<'a>,
    },
    LandingGearGreen {
        control: LandingGearGreen,
        callback: LedCallback<'a>,
    },
    LandingGearRed {
        control: LandingGearRed,
        callback: LedCallback<'a>,
    },
    LandingLight {
        control: LandingLight,
        callback: IntegerCallback<'a>,
    },
    LeftBombArmChem {
        control: LeftBombArmChem,
        callback: IntegerCallback<'a>,
    },
    LeftFluorescentLight {
        control: LeftFluorescentLight,
        callback: IntegerCallback<'a>,
    },
    LeftFuelTankValue {
        control: LeftFuelTankValue,
        callback: IntegerCallback<'a>,
    },
    LftPylodSlvo {
        control: LftPylodSlvo,
        callback: IntegerCallback<'a>,
    },
    LockPropellerMixture {
        control: LockPropellerMixture,
        callback: IntegerCallback<'a>,
    },
    LockStickFrwdNeut {
        control: LockStickFrwdNeut,
        callback: IntegerCallback<'a>,
    },
    LockThrottle {
        control: LockThrottle,
        callback: IntegerCallback<'a>,
    },
    ManifoldPressure {
        control: ManifoldPressure,
        callback: IntegerCallback<'a>,
    },
    ManifoldPressureValue {
        control: ManifoldPressureValue,
        callback: IntegerCallback<'a>,
    },
    MicrophoneOn {
        control: MicrophoneOn,
        callback: IntegerCallback<'a>,
    },
    MixtureControl {
        control: MixtureControl,
        callback: IntegerCallback<'a>,
    },
    OffPower {
        control: OffPower,
        callback: IntegerCallback<'a>,
    },
    OilControl {
        control: OilControl,
        callback: IntegerCallback<'a>,
    },
    OilControlCover {
        control: OilControlCover,
        callback: IntegerCallback<'a>,
    },
    OilDilute {
        control: OilDilute,
        callback: IntegerCallback<'a>,
    },
    OilPres {
        control: OilPres,
        callback: IntegerCallback<'a>,
    },
    OilPressureValue {
        control: OilPressureValue,
        callback: IntegerCallback<'a>,
    },
    OilTemp {
        control: OilTemp,
        callback: IntegerCallback<'a>,
    },
    OilTemperatureValue {
        control: OilTemperatureValue,
        callback: IntegerCallback<'a>,
    },
    OxygenAutoMix {
        control: OxygenAutoMix,
        callback: IntegerCallback<'a>,
    },
    OxygenByPass {
        control: OxygenByPass,
        callback: IntegerCallback<'a>,
    },
    OxygenFlow {
        control: OxygenFlow,
        callback: IntegerCallback<'a>,
    },
    OxygenPres {
        control: OxygenPres,
        callback: IntegerCallback<'a>,
    },
    OxygenPressureValue {
        control: OxygenPressureValue,
        callback: IntegerCallback<'a>,
    },
    OxygenWarning {
        control: OxygenWarning,
        callback: LedCallback<'a>,
    },
    ParkBrakeHandle {
        control: ParkBrakeHandle,
        callback: IntegerCallback<'a>,
    },
    PitchAdjust {
        control: PitchAdjust,
        callback: IntegerCallback<'a>,
    },
    Pitot {
        control: Pitot,
        callback: IntegerCallback<'a>,
    },
    Primer {
        control: Primer,
        callback: IntegerCallback<'a>,
    },
    PropellerRpm {
        control: PropellerRpm,
        callback: IntegerCallback<'a>,
    },
    RadarWarningLight {
        control: RadarWarningLight,
        callback: LedCallback<'a>,
    },
    RadioLightsDimmer {
        control: RadioLightsDimmer,
        callback: IntegerCallback<'a>,
    },
    RadioMode2 {
        control: RadioMode2,
        callback: IntegerCallback<'a>,
    },
    RadioMode3 {
        control: RadioMode3,
        callback: IntegerCallback<'a>,
    },
    RadioVolume {
        control: RadioVolume,
        callback: IntegerCallback<'a>,
    },
    RecLtsKey {
        control: RecLtsKey,
        callback: IntegerCallback<'a>,
    },
    RedRecLt {
        control: RedRecLt,
        callback: IntegerCallback<'a>,
    },
    RemoteCompassCrs {
        control: RemoteCompassCrs,
        callback: IntegerCallback<'a>,
    },
    RemoteCompassHdg {
        control: RemoteCompassHdg,
        callback: IntegerCallback<'a>,
    },
    RemoteCompassSet {
        control: RemoteCompassSet,
        callback: IntegerCallback<'a>,
    },
    RetMaskLvr {
        control: RetMaskLvr,
        callback: IntegerCallback<'a>,
    },
    RightBombArmChem {
        control: RightBombArmChem,
        callback: IntegerCallback<'a>,
    },
    RightFuelTankValue {
        control: RightFuelTankValue,
        callback: IntegerCallback<'a>,
    },
    RktDelSwitch {
        control: RktDelSwitch,
        callback: IntegerCallback<'a>,
    },
    RktRelMode {
        control: RktRelMode,
        callback: IntegerCallback<'a>,
    },
    RocketCounter {
        control: RocketCounter,
        callback: IntegerCallback<'a>,
    },
    RocketsBombsModes {
        control: RocketsBombsModes,
        callback: IntegerCallback<'a>,
    },
    RtFlresLt {
        control: RtFlresLt,
        callback: IntegerCallback<'a>,
    },
    RtPylodSlvo {
        control: RtPylodSlvo,
        callback: IntegerCallback<'a>,
    },
    RudderTrim {
        control: RudderTrim,
        callback: IntegerCallback<'a>,
    },
    SafeLndGearLtTest {
        control: SafeLndGearLtTest,
        callback: IntegerCallback<'a>,
    },
    SetPressure {
        control: SetPressure,
        callback: IntegerCallback<'a>,
    },
    SilenceGearHorn {
        control: SilenceGearHorn,
        callback: IntegerCallback<'a>,
    },
    Slipball {
        control: Slipball,
        callback: IntegerCallback<'a>,
    },
    Starter {
        control: Starter,
        callback: IntegerCallback<'a>,
    },
    StarterCover {
        control: StarterCover,
        callback: IntegerCallback<'a>,
    },
    SuperchargerAutoLowHigh {
        control: SuperchargerAutoLowHigh,
        callback: IntegerCallback<'a>,
    },
    SuperchargerSwitchCover {
        control: SuperchargerSwitchCover,
        callback: IntegerCallback<'a>,
    },
    TailLts {
        control: TailLts,
        callback: IntegerCallback<'a>,
    },
    Throttle {
        control: Throttle,
        callback: IntegerCallback<'a>,
    },
    TurnIndicator {
        control: TurnIndicator,
        callback: IntegerCallback<'a>,
    },
    UnsafeLndGearLtTest {
        control: UnsafeLndGearLtTest,
        callback: IntegerCallback<'a>,
    },
    VacuumSuction {
        control: VacuumSuction,
        callback: IntegerCallback<'a>,
    },
    VacuumSuctionValue {
        control: VacuumSuctionValue,
        callback: IntegerCallback<'a>,
    },
    VariometerVvi {
        control: VariometerVvi,
        callback: IntegerCallback<'a>,
    },
    VhfLockingLever {
        control: VhfLockingLever,
        callback: IntegerCallback<'a>,
    },
    VhfRadioALight {
        control: VhfRadioALight,
        callback: LedCallback<'a>,
    },
    VhfRadioBLight {
        control: VhfRadioBLight,
        callback: LedCallback<'a>,
    },
    VhfRadioCLight {
        control: VhfRadioCLight,
        callback: LedCallback<'a>,
    },
    VhfRadioChanA {
        control: VhfRadioChanA,
        callback: IntegerCallback<'a>,
    },
    VhfRadioChanB {
        control: VhfRadioChanB,
        callback: IntegerCallback<'a>,
    },
    VhfRadioChanC {
        control: VhfRadioChanC,
        callback: IntegerCallback<'a>,
    },
    VhfRadioChanD {
        control: VhfRadioChanD,
        callback: IntegerCallback<'a>,
    },
    VhfRadioDLight {
        control: VhfRadioDLight,
        callback: LedCallback<'a>,
    },
    VhfRadioOnOff {
        control: VhfRadioOnOff,
        callback: IntegerCallback<'a>,
    },
    VhfRadioTxLight {
        control: VhfRadioTxLight,
        callback: LedCallback<'a>,
    },
    WarningRadarLight {
        control: WarningRadarLight,
        callback: IntegerCallback<'a>,
    },
    WarningRadarPower {
        control: WarningRadarPower,
        callback: IntegerCallback<'a>,
    },
    WarningRadarTest {
        control: WarningRadarTest,
        callback: IntegerCallback<'a>,
    },
    WindshieldOil {
        control: WindshieldOil,
        callback: IntegerCallback<'a>,
    },
    Windshieldcracks {
        control: Windshieldcracks,
        callback: IntegerCallback<'a>,
    },
    Windshieldoil {
        control: Windshieldoil,
        callback: LedCallback<'a>,
    },
    WingLts {
        control: WingLts,
        callback: IntegerCallback<'a>,
    },
    WingSpanSel {
        control: WingSpanSel,
        callback: IntegerCallback<'a>,
    },
}

impl RegisteredControl<'_> {
    fn accept(&mut self, packet: Packet<'_>) {
        match self {
            Self::AccelerometerMain { control, .. } => control.accept(packet),
            Self::AccelerometerMax { control, .. } => control.accept(packet),
            Self::AccelerometerMin { control, .. } => control.accept(packet),
            Self::AccelerometerRst { control, .. } => control.accept(packet),
            Self::AccelerometerValue { control, .. } => control.accept(packet),
            Self::AhorizonBank { control, .. } => control.accept(packet),
            Self::AhorizonCaged { control, .. } => control.accept(packet),
            Self::AhorizonPitch { control, .. } => control.accept(packet),
            Self::AhorizonPitchShift { control, .. } => control.accept(packet),
            Self::AileronTrim { control, .. } => control.accept(packet),
            Self::AirspeedMphValue { control, .. } => control.accept(packet),
            Self::AirspeedNeedle { control, .. } => control.accept(packet),
            Self::Altimeter10000Foot { control, .. } => control.accept(packet),
            Self::Altimeter1000Foot { control, .. } => control.accept(packet),
            Self::Altimeter100Foot { control, .. } => control.accept(packet),
            Self::AltimeterPressure { control, .. } => control.accept(packet),
            Self::AltimeterValue { control, .. } => control.accept(packet),
            Self::AmbrRecLt { control, .. } => control.accept(packet),
            Self::Ammeter { control, .. } => control.accept(packet),
            Self::AmmeterValue { control, .. } => control.accept(packet),
            Self::ArmRest { control, .. } => control.accept(packet),
            Self::BarometricPressureValue { control, .. } => control.accept(packet),
            Self::Bat { control, .. } => control.accept(packet),
            Self::CageButton { control, .. } => control.accept(packet),
            Self::CageRotary { control, .. } => control.accept(packet),
            Self::CanopyEmergencyReleaseHandle { control, .. } => control.accept(packet),
            Self::CanopyHandCrank { control, .. } => control.accept(packet),
            Self::CarbColdAir { control, .. } => control.accept(packet),
            Self::CarbTemp { control, .. } => control.accept(packet),
            Self::CarbWarmAir { control, .. } => control.accept(packet),
            Self::CircProRst { control, .. } => control.accept(packet),
            Self::ClockAdjust { control, .. } => control.accept(packet),
            Self::ClockAdjustButton { control, .. } => control.accept(packet),
            Self::ClockHours { control, .. } => control.accept(packet),
            Self::ClockMinutes { control, .. } => control.accept(packet),
            Self::ClockSeconds { control, .. } => control.accept(packet),
            Self::CockpitLights { control, .. } => control.accept(packet),
            Self::ColdAir { control, .. } => control.accept(packet),
            Self::CoolantControl { control, .. } => control.accept(packet),
            Self::CoolantControlCover { control, .. } => control.accept(packet),
            Self::CoolantTemp { control, .. } => control.accept(packet),
            Self::CourseValue { control, .. } => control.accept(packet),
            Self::Defroster { control, .. } => control.accept(packet),
            Self::DetrolaFrequency { control, .. } => control.accept(packet),
            Self::DetrolaVolume { control, .. } => control.accept(packet),
            Self::DirectionalGyro { control, .. } => control.accept(packet),
            Self::DirectionalGyroValue { control, .. } => control.accept(packet),
            Self::ElevatorTrim { control, .. } => control.accept(packet),
            Self::EngineRpm { control, .. } => control.accept(packet),
            Self::EngineRpmValue { control, .. } => control.accept(packet),
            Self::ExtFormationLights { control, .. } => control.accept(packet),
            Self::ExtPositionLightLeft { control, .. } => control.accept(packet),
            Self::ExtPositionLightRight { control, .. } => control.accept(packet),
            Self::FlapsControlHandle { control, .. } => control.accept(packet),
            Self::FuelBooster { control, .. } => control.accept(packet),
            Self::FuelPressure { control, .. } => control.accept(packet),
            Self::FuelPressureValue { control, .. } => control.accept(packet),
            Self::FuelSelectorValve { control, .. } => control.accept(packet),
            Self::FuelShutOffValve { control, .. } => control.accept(packet),
            Self::FuelTankFuselage { control, .. } => control.accept(packet),
            Self::FuelTankLeft { control, .. } => control.accept(packet),
            Self::FuelTankRight { control, .. } => control.accept(packet),
            Self::FuselageFuelTankValue { control, .. } => control.accept(packet),
            Self::Gen { control, .. } => control.accept(packet),
            Self::GrnRecLt { control, .. } => control.accept(packet),
            Self::GunControl { control, .. } => control.accept(packet),
            Self::GunHeat { control, .. } => control.accept(packet),
            Self::GunsightBrt { control, .. } => control.accept(packet),
            Self::GunsightFixedGyro { control, .. } => control.accept(packet),
            Self::GunsightOnOff { control, .. } => control.accept(packet),
            Self::HdgCageButton { control, .. } => control.accept(packet),
            Self::HdgCageRotary { control, .. } => control.accept(packet),
            Self::HeadingValue { control, .. } => control.accept(packet),
            Self::HighBlowerLamp { control, .. } => control.accept(packet),
            Self::HighBlowerTestLamp { control, .. } => control.accept(packet),
            Self::HomingAdapterCb { control, .. } => control.accept(packet),
            Self::HomingAdapterMode { control, .. } => control.accept(packet),
            Self::HomingAdapterPower { control, .. } => control.accept(packet),
            Self::HotAir { control, .. } => control.accept(packet),
            Self::HydRel { control, .. } => control.accept(packet),
            Self::HydraulicPressure { control, .. } => control.accept(packet),
            Self::HydraulicPressureValue { control, .. } => control.accept(packet),
            Self::IffCode { control, .. } => control.accept(packet),
            Self::IffDetCircuit { control, .. } => control.accept(packet),
            Self::IffDetLeft { control, .. } => control.accept(packet),
            Self::IffDetRight { control, .. } => control.accept(packet),
            Self::IffDistress { control, .. } => control.accept(packet),
            Self::IffTimeOffOn { control, .. } => control.accept(packet),
            Self::Ignition { control, .. } => control.accept(packet),
            Self::LandingGearContr0lHandle { control, .. } => control.accept(packet),
            Self::LandingGearGreen { control, .. } => control.accept(packet),
            Self::LandingGearRed { control, .. } => control.accept(packet),
            Self::LandingLight { control, .. } => control.accept(packet),
            Self::LeftBombArmChem { control, .. } => control.accept(packet),
            Self::LeftFluorescentLight { control, .. } => control.accept(packet),
            Self::LeftFuelTankValue { control, .. } => control.accept(packet),
            Self::LftPylodSlvo { control, .. } => control.accept(packet),
            Self::LockPropellerMixture { control, .. } => control.accept(packet),
            Self::LockStickFrwdNeut { control, .. } => control.accept(packet),
            Self::LockThrottle { control, .. } => control.accept(packet),
            Self::ManifoldPressure { control, .. } => control.accept(packet),
            Self::ManifoldPressureValue { control, .. } => control.accept(packet),
            Self::MicrophoneOn { control, .. } => control.accept(packet),
            Self::MixtureControl { control, .. } => control.accept(packet),
            Self::OffPower { control, .. } => control.accept(packet),
            Self::OilControl { control, .. } => control.accept(packet),
            Self::OilControlCover { control, .. } => control.accept(packet),
            Self::OilDilute { control, .. } => control.accept(packet),
            Self::OilPres { control, .. } => control.accept(packet),
            Self::OilPressureValue { control, .. } => control.accept(packet),
            Self::OilTemp { control, .. } => control.accept(packet),
            Self::OilTemperatureValue { control, .. } => control.accept(packet),
            Self::OxygenAutoMix { control, .. } => control.accept(packet),
            Self::OxygenByPass { control, .. } => control.accept(packet),
            Self::OxygenFlow { control, .. } => control.accept(packet),
            Self::OxygenPres { control, .. } => control.accept(packet),
            Self::OxygenPressureValue { control, .. } => control.accept(packet),
            Self::OxygenWarning { control, .. } => control.accept(packet),
            Self::ParkBrakeHandle { control, .. } => control.accept(packet),
            Self::PitchAdjust { control, .. } => control.accept(packet),
            Self::Pitot { control, .. } => control.accept(packet),
            Self::Primer { control, .. } => control.accept(packet),
            Self::PropellerRpm { control, .. } => control.accept(packet),
            Self::RadarWarningLight { control, .. } => control.accept(packet),
            Self::RadioLightsDimmer { control, .. } => control.accept(packet),
            Self::RadioMode2 { control, .. } => control.accept(packet),
            Self::RadioMode3 { control, .. } => control.accept(packet),
            Self::RadioVolume { control, .. } => control.accept(packet),
            Self::RecLtsKey { control, .. } => control.accept(packet),
            Self::RedRecLt { control, .. } => control.accept(packet),
            Self::RemoteCompassCrs { control, .. } => control.accept(packet),
            Self::RemoteCompassHdg { control, .. } => control.accept(packet),
            Self::RemoteCompassSet { control, .. } => control.accept(packet),
            Self::RetMaskLvr { control, .. } => control.accept(packet),
            Self::RightBombArmChem { control, .. } => control.accept(packet),
            Self::RightFuelTankValue { control, .. } => control.accept(packet),
            Self::RktDelSwitch { control, .. } => control.accept(packet),
            Self::RktRelMode { control, .. } => control.accept(packet),
            Self::RocketCounter { control, .. } => control.accept(packet),
            Self::RocketsBombsModes { control, .. } => control.accept(packet),
            Self::RtFlresLt { control, .. } => control.accept(packet),
            Self::RtPylodSlvo { control, .. } => control.accept(packet),
            Self::RudderTrim { control, .. } => control.accept(packet),
            Self::SafeLndGearLtTest { control, .. } => control.accept(packet),
            Self::SetPressure { control, .. } => control.accept(packet),
            Self::SilenceGearHorn { control, .. } => control.accept(packet),
            Self::Slipball { control, .. } => control.accept(packet),
            Self::Starter { control, .. } => control.accept(packet),
            Self::StarterCover { control, .. } => control.accept(packet),
            Self::SuperchargerAutoLowHigh { control, .. } => control.accept(packet),
            Self::SuperchargerSwitchCover { control, .. } => control.accept(packet),
            Self::TailLts { control, .. } => control.accept(packet),
            Self::Throttle { control, .. } => control.accept(packet),
            Self::TurnIndicator { control, .. } => control.accept(packet),
            Self::UnsafeLndGearLtTest { control, .. } => control.accept(packet),
            Self::VacuumSuction { control, .. } => control.accept(packet),
            Self::VacuumSuctionValue { control, .. } => control.accept(packet),
            Self::VariometerVvi { control, .. } => control.accept(packet),
            Self::VhfLockingLever { control, .. } => control.accept(packet),
            Self::VhfRadioALight { control, .. } => control.accept(packet),
            Self::VhfRadioBLight { control, .. } => control.accept(packet),
            Self::VhfRadioCLight { control, .. } => control.accept(packet),
            Self::VhfRadioChanA { control, .. } => control.accept(packet),
            Self::VhfRadioChanB { control, .. } => control.accept(packet),
            Self::VhfRadioChanC { control, .. } => control.accept(packet),
            Self::VhfRadioChanD { control, .. } => control.accept(packet),
            Self::VhfRadioDLight { control, .. } => control.accept(packet),
            Self::VhfRadioOnOff { control, .. } => control.accept(packet),
            Self::VhfRadioTxLight { control, .. } => control.accept(packet),
            Self::WarningRadarLight { control, .. } => control.accept(packet),
            Self::WarningRadarPower { control, .. } => control.accept(packet),
            Self::WarningRadarTest { control, .. } => control.accept(packet),
            Self::WindshieldOil { control, .. } => control.accept(packet),
            Self::Windshieldcracks { control, .. } => control.accept(packet),
            Self::Windshieldoil { control, .. } => control.accept(packet),
            Self::WingLts { control, .. } => control.accept(packet),
            Self::WingSpanSel { control, .. } => control.accept(packet),
        }
    }

    fn commit(&mut self) {
        match self {
            Self::AccelerometerMain { control, callback } => {
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
            Self::AccelerometerRst { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AccelerometerValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AhorizonBank { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AhorizonCaged { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AhorizonPitch { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AhorizonPitchShift { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AileronTrim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AirspeedMphValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AirspeedNeedle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Altimeter10000Foot { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Altimeter1000Foot { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Altimeter100Foot { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltimeterPressure { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltimeterValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AmbrRecLt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Ammeter { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AmmeterValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ArmRest { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BarometricPressureValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Bat { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CageButton { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CageRotary { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CanopyEmergencyReleaseHandle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CanopyHandCrank { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CarbColdAir { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CarbTemp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CarbWarmAir { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CircProRst { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockAdjust { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockAdjustButton { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockHours { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockMinutes { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockSeconds { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CockpitLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ColdAir { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CoolantControl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CoolantControlCover { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CoolantTemp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CourseValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Defroster { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DetrolaFrequency { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DetrolaVolume { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DirectionalGyro { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DirectionalGyroValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ElevatorTrim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EngineRpm { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EngineRpmValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtFormationLights { control, callback } => {
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
            Self::FlapsControlHandle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelBooster { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelPressure { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelPressureValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelSelectorValve { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelShutOffValve { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelTankFuselage { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelTankLeft { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelTankRight { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuselageFuelTankValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Gen { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GrnRecLt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GunControl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GunHeat { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GunsightBrt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GunsightFixedGyro { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GunsightOnOff { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HdgCageButton { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HdgCageRotary { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HeadingValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HighBlowerLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::HighBlowerTestLamp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HomingAdapterCb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HomingAdapterMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HomingAdapterPower { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HotAir { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HydRel { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HydraulicPressure { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HydraulicPressureValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IffCode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IffDetCircuit { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IffDetLeft { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IffDetRight { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IffDistress { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IffTimeOffOn { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Ignition { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LandingGearContr0lHandle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LandingGearGreen { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LandingGearRed { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LandingLight { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LeftBombArmChem { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LeftFluorescentLight { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LeftFuelTankValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LftPylodSlvo { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LockPropellerMixture { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LockStickFrwdNeut { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LockThrottle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ManifoldPressure { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ManifoldPressureValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MicrophoneOn { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MixtureControl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OffPower { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OilControl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OilControlCover { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OilDilute { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OilPres { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OilPressureValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OilTemp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OilTemperatureValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OxygenAutoMix { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OxygenByPass { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OxygenFlow { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OxygenPres { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OxygenPressureValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OxygenWarning { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ParkBrakeHandle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PitchAdjust { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Pitot { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Primer { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PropellerRpm { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarWarningLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RadioLightsDimmer { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadioMode2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadioMode3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadioVolume { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RecLtsKey { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RedRecLt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RemoteCompassCrs { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RemoteCompassHdg { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RemoteCompassSet { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RetMaskLvr { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RightBombArmChem { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RightFuelTankValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RktDelSwitch { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RktRelMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RocketCounter { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RocketsBombsModes { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RtFlresLt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RtPylodSlvo { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RudderTrim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SafeLndGearLtTest { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SetPressure { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SilenceGearHorn { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Slipball { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Starter { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StarterCover { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SuperchargerAutoLowHigh { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SuperchargerSwitchCover { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TailLts { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Throttle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TurnIndicator { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UnsafeLndGearLtTest { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VacuumSuction { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VacuumSuctionValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VariometerVvi { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhfLockingLever { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhfRadioALight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::VhfRadioBLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::VhfRadioCLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::VhfRadioChanA { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhfRadioChanB { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhfRadioChanC { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhfRadioChanD { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhfRadioDLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::VhfRadioOnOff { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhfRadioTxLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::WarningRadarLight { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::WarningRadarPower { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::WarningRadarTest { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::WindshieldOil { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Windshieldcracks { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Windshieldoil { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::WingLts { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::WingSpanSel { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
        }
    }
}

pub struct P51dBuilder<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    _packet_capacity: PhantomData<ProtocolParser<MAX_PACKET_DATA>>,
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    P51dBuilder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
{
    pub fn new() -> Self {
        Self {
            controls: Vec::new(),
            _packet_capacity: PhantomData,
        }
    }

    pub fn register_accelerometer_main<F>(
        &mut self,
        control: AccelerometerMain,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AccelerometerMain { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_accelerometer_max<F>(
        &mut self,
        control: AccelerometerMax,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AccelerometerMax { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_accelerometer_min<F>(
        &mut self,
        control: AccelerometerMin,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AccelerometerMin { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_accelerometer_rst<F>(
        &mut self,
        control: AccelerometerRst,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AccelerometerRst { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_accelerometer_value<F>(
        &mut self,
        control: AccelerometerValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AccelerometerValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ahorizon_bank<F>(
        &mut self,
        control: AhorizonBank,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AhorizonBank { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ahorizon_caged<F>(
        &mut self,
        control: AhorizonCaged,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AhorizonCaged { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ahorizon_pitch<F>(
        &mut self,
        control: AhorizonPitch,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AhorizonPitch { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ahorizon_pitch_shift<F>(
        &mut self,
        control: AhorizonPitchShift,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AhorizonPitchShift { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aileron_trim<F>(
        &mut self,
        control: AileronTrim,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AileronTrim { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_airspeed_mph_value<F>(
        &mut self,
        control: AirspeedMphValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AirspeedMphValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_airspeed_needle<F>(
        &mut self,
        control: AirspeedNeedle,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AirspeedNeedle { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter10000_foot<F>(
        &mut self,
        control: Altimeter10000Foot,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Altimeter10000Foot { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter1000_foot<F>(
        &mut self,
        control: Altimeter1000Foot,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Altimeter1000Foot { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter100_foot<F>(
        &mut self,
        control: Altimeter100Foot,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Altimeter100Foot { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter_pressure<F>(
        &mut self,
        control: AltimeterPressure,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltimeterPressure { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter_value<F>(
        &mut self,
        control: AltimeterValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltimeterValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ambr_rec_lt<F>(
        &mut self,
        control: AmbrRecLt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmbrRecLt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ammeter<F>(
        &mut self,
        control: Ammeter,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ammeter { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ammeter_value<F>(
        &mut self,
        control: AmmeterValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmmeterValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_arm_rest<F>(
        &mut self,
        control: ArmRest,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ArmRest { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_barometric_pressure_value<F>(
        &mut self,
        control: BarometricPressureValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BarometricPressureValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bat<F>(
        &mut self,
        control: Bat,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Bat { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cage_button<F>(
        &mut self,
        control: CageButton,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CageButton { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cage_rotary<F>(
        &mut self,
        control: CageRotary,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CageRotary { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_emergency_release_handle<F>(
        &mut self,
        control: CanopyEmergencyReleaseHandle,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyEmergencyReleaseHandle { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_hand_crank<F>(
        &mut self,
        control: CanopyHandCrank,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyHandCrank { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_carb_cold_air<F>(
        &mut self,
        control: CarbColdAir,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CarbColdAir { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_carb_temp<F>(
        &mut self,
        control: CarbTemp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CarbTemp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_carb_warm_air<F>(
        &mut self,
        control: CarbWarmAir,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CarbWarmAir { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_circ_pro_rst<F>(
        &mut self,
        control: CircProRst,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CircProRst { control, callback })
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

    pub fn register_clock_adjust_button<F>(
        &mut self,
        control: ClockAdjustButton,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockAdjustButton { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_hours<F>(
        &mut self,
        control: ClockHours,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockHours { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_minutes<F>(
        &mut self,
        control: ClockMinutes,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockMinutes { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_seconds<F>(
        &mut self,
        control: ClockSeconds,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockSeconds { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cockpit_lights<F>(
        &mut self,
        control: CockpitLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CockpitLights { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cold_air<F>(
        &mut self,
        control: ColdAir,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ColdAir { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_coolant_control<F>(
        &mut self,
        control: CoolantControl,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CoolantControl { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_coolant_control_cover<F>(
        &mut self,
        control: CoolantControlCover,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CoolantControlCover { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_coolant_temp<F>(
        &mut self,
        control: CoolantTemp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CoolantTemp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_course_value<F>(
        &mut self,
        control: CourseValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CourseValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_defroster<F>(
        &mut self,
        control: Defroster,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Defroster { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_detrola_frequency<F>(
        &mut self,
        control: DetrolaFrequency,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DetrolaFrequency { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_detrola_volume<F>(
        &mut self,
        control: DetrolaVolume,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DetrolaVolume { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_directional_gyro<F>(
        &mut self,
        control: DirectionalGyro,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DirectionalGyro { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_directional_gyro_value<F>(
        &mut self,
        control: DirectionalGyroValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DirectionalGyroValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_elevator_trim<F>(
        &mut self,
        control: ElevatorTrim,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ElevatorTrim { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_rpm<F>(
        &mut self,
        control: EngineRpm,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineRpm { control, callback })
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

    pub fn register_flaps_control_handle<F>(
        &mut self,
        control: FlapsControlHandle,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlapsControlHandle { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_booster<F>(
        &mut self,
        control: FuelBooster,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelBooster { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_pressure<F>(
        &mut self,
        control: FuelPressure,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelPressure { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_pressure_value<F>(
        &mut self,
        control: FuelPressureValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelPressureValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_selector_valve<F>(
        &mut self,
        control: FuelSelectorValve,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelSelectorValve { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_shut_off_valve<F>(
        &mut self,
        control: FuelShutOffValve,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelShutOffValve { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_tank_fuselage<F>(
        &mut self,
        control: FuelTankFuselage,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelTankFuselage { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_tank_left<F>(
        &mut self,
        control: FuelTankLeft,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelTankLeft { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_tank_right<F>(
        &mut self,
        control: FuelTankRight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelTankRight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuselage_fuel_tank_value<F>(
        &mut self,
        control: FuselageFuelTankValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuselageFuelTankValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gen<F>(
        &mut self,
        control: Gen,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Gen { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_grn_rec_lt<F>(
        &mut self,
        control: GrnRecLt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GrnRecLt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gun_control<F>(
        &mut self,
        control: GunControl,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GunControl { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gun_heat<F>(
        &mut self,
        control: GunHeat,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GunHeat { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gunsight_brt<F>(
        &mut self,
        control: GunsightBrt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GunsightBrt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gunsight_fixed_gyro<F>(
        &mut self,
        control: GunsightFixedGyro,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GunsightFixedGyro { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gunsight_on_off<F>(
        &mut self,
        control: GunsightOnOff,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GunsightOnOff { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hdg_cage_button<F>(
        &mut self,
        control: HdgCageButton,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HdgCageButton { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hdg_cage_rotary<F>(
        &mut self,
        control: HdgCageRotary,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HdgCageRotary { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_heading_value<F>(
        &mut self,
        control: HeadingValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HeadingValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_high_blower_lamp<F>(
        &mut self,
        control: HighBlowerLamp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::HighBlowerLamp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_high_blower_test_lamp<F>(
        &mut self,
        control: HighBlowerTestLamp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HighBlowerTestLamp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_homing_adapter_cb<F>(
        &mut self,
        control: HomingAdapterCb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HomingAdapterCb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_homing_adapter_mode<F>(
        &mut self,
        control: HomingAdapterMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HomingAdapterMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_homing_adapter_power<F>(
        &mut self,
        control: HomingAdapterPower,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HomingAdapterPower { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hot_air<F>(
        &mut self,
        control: HotAir,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HotAir { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hyd_rel<F>(
        &mut self,
        control: HydRel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydRel { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hydraulic_pressure<F>(
        &mut self,
        control: HydraulicPressure,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydraulicPressure { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hydraulic_pressure_value<F>(
        &mut self,
        control: HydraulicPressureValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydraulicPressureValue { control, callback })
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

    pub fn register_iff_det_circuit<F>(
        &mut self,
        control: IffDetCircuit,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffDetCircuit { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_det_left<F>(
        &mut self,
        control: IffDetLeft,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffDetLeft { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_det_right<F>(
        &mut self,
        control: IffDetRight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffDetRight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_distress<F>(
        &mut self,
        control: IffDistress,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffDistress { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_time_off_on<F>(
        &mut self,
        control: IffTimeOffOn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffTimeOffOn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ignition<F>(
        &mut self,
        control: Ignition,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ignition { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_landing_gear_contr0l_handle<F>(
        &mut self,
        control: LandingGearContr0lHandle,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LandingGearContr0lHandle { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_landing_gear_green<F>(
        &mut self,
        control: LandingGearGreen,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LandingGearGreen { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_landing_gear_red<F>(
        &mut self,
        control: LandingGearRed,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LandingGearRed { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_landing_light<F>(
        &mut self,
        control: LandingLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LandingLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_bomb_arm_chem<F>(
        &mut self,
        control: LeftBombArmChem,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftBombArmChem { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_fluorescent_light<F>(
        &mut self,
        control: LeftFluorescentLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftFluorescentLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_fuel_tank_value<F>(
        &mut self,
        control: LeftFuelTankValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftFuelTankValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lft_pylod_slvo<F>(
        &mut self,
        control: LftPylodSlvo,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LftPylodSlvo { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lock_propeller_mixture<F>(
        &mut self,
        control: LockPropellerMixture,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LockPropellerMixture { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lock_stick_frwd_neut<F>(
        &mut self,
        control: LockStickFrwdNeut,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LockStickFrwdNeut { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lock_throttle<F>(
        &mut self,
        control: LockThrottle,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LockThrottle { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_manifold_pressure<F>(
        &mut self,
        control: ManifoldPressure,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ManifoldPressure { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_manifold_pressure_value<F>(
        &mut self,
        control: ManifoldPressureValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ManifoldPressureValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_microphone_on<F>(
        &mut self,
        control: MicrophoneOn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MicrophoneOn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mixture_control<F>(
        &mut self,
        control: MixtureControl,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MixtureControl { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_off_power<F>(
        &mut self,
        control: OffPower,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OffPower { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oil_control<F>(
        &mut self,
        control: OilControl,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OilControl { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oil_control_cover<F>(
        &mut self,
        control: OilControlCover,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OilControlCover { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oil_dilute<F>(
        &mut self,
        control: OilDilute,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OilDilute { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oil_pres<F>(
        &mut self,
        control: OilPres,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OilPres { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oil_pressure_value<F>(
        &mut self,
        control: OilPressureValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OilPressureValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oil_temp<F>(
        &mut self,
        control: OilTemp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OilTemp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oil_temperature_value<F>(
        &mut self,
        control: OilTemperatureValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OilTemperatureValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxygen_auto_mix<F>(
        &mut self,
        control: OxygenAutoMix,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxygenAutoMix { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxygen_by_pass<F>(
        &mut self,
        control: OxygenByPass,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxygenByPass { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxygen_flow<F>(
        &mut self,
        control: OxygenFlow,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxygenFlow { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxygen_pres<F>(
        &mut self,
        control: OxygenPres,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxygenPres { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxygen_pressure_value<F>(
        &mut self,
        control: OxygenPressureValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxygenPressureValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxygen_warning<F>(
        &mut self,
        control: OxygenWarning,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxygenWarning { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_park_brake_handle<F>(
        &mut self,
        control: ParkBrakeHandle,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ParkBrakeHandle { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pitch_adjust<F>(
        &mut self,
        control: PitchAdjust,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PitchAdjust { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pitot<F>(
        &mut self,
        control: Pitot,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Pitot { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_primer<F>(
        &mut self,
        control: Primer,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Primer { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_propeller_rpm<F>(
        &mut self,
        control: PropellerRpm,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PropellerRpm { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_warning_light<F>(
        &mut self,
        control: RadarWarningLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarWarningLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radio_lights_dimmer<F>(
        &mut self,
        control: RadioLightsDimmer,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadioLightsDimmer { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radio_mode2<F>(
        &mut self,
        control: RadioMode2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadioMode2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radio_mode3<F>(
        &mut self,
        control: RadioMode3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadioMode3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radio_volume<F>(
        &mut self,
        control: RadioVolume,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadioVolume { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rec_lts_key<F>(
        &mut self,
        control: RecLtsKey,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RecLtsKey { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_red_rec_lt<F>(
        &mut self,
        control: RedRecLt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RedRecLt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_remote_compass_crs<F>(
        &mut self,
        control: RemoteCompassCrs,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RemoteCompassCrs { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_remote_compass_hdg<F>(
        &mut self,
        control: RemoteCompassHdg,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RemoteCompassHdg { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_remote_compass_set<F>(
        &mut self,
        control: RemoteCompassSet,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RemoteCompassSet { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ret_mask_lvr<F>(
        &mut self,
        control: RetMaskLvr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RetMaskLvr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_bomb_arm_chem<F>(
        &mut self,
        control: RightBombArmChem,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightBombArmChem { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_fuel_tank_value<F>(
        &mut self,
        control: RightFuelTankValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightFuelTankValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rkt_del_switch<F>(
        &mut self,
        control: RktDelSwitch,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RktDelSwitch { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rkt_rel_mode<F>(
        &mut self,
        control: RktRelMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RktRelMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rocket_counter<F>(
        &mut self,
        control: RocketCounter,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RocketCounter { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rockets_bombs_modes<F>(
        &mut self,
        control: RocketsBombsModes,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RocketsBombsModes { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rt_flres_lt<F>(
        &mut self,
        control: RtFlresLt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RtFlresLt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rt_pylod_slvo<F>(
        &mut self,
        control: RtPylodSlvo,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RtPylodSlvo { control, callback })
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

    pub fn register_safe_lnd_gear_lt_test<F>(
        &mut self,
        control: SafeLndGearLtTest,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SafeLndGearLtTest { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_set_pressure<F>(
        &mut self,
        control: SetPressure,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SetPressure { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_silence_gear_horn<F>(
        &mut self,
        control: SilenceGearHorn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SilenceGearHorn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_slipball<F>(
        &mut self,
        control: Slipball,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Slipball { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_starter<F>(
        &mut self,
        control: Starter,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Starter { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_starter_cover<F>(
        &mut self,
        control: StarterCover,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StarterCover { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_supercharger_auto_low_high<F>(
        &mut self,
        control: SuperchargerAutoLowHigh,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SuperchargerAutoLowHigh { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_supercharger_switch_cover<F>(
        &mut self,
        control: SuperchargerSwitchCover,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SuperchargerSwitchCover { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tail_lts<F>(
        &mut self,
        control: TailLts,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TailLts { control, callback })
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

    pub fn register_turn_indicator<F>(
        &mut self,
        control: TurnIndicator,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TurnIndicator { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_unsafe_lnd_gear_lt_test<F>(
        &mut self,
        control: UnsafeLndGearLtTest,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UnsafeLndGearLtTest { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vacuum_suction<F>(
        &mut self,
        control: VacuumSuction,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VacuumSuction { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vacuum_suction_value<F>(
        &mut self,
        control: VacuumSuctionValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VacuumSuctionValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_variometer_vvi<F>(
        &mut self,
        control: VariometerVvi,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VariometerVvi { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_locking_lever<F>(
        &mut self,
        control: VhfLockingLever,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfLockingLever { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_radio_a_light<F>(
        &mut self,
        control: VhfRadioALight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfRadioALight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_radio_b_light<F>(
        &mut self,
        control: VhfRadioBLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfRadioBLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_radio_c_light<F>(
        &mut self,
        control: VhfRadioCLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfRadioCLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_radio_chan_a<F>(
        &mut self,
        control: VhfRadioChanA,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfRadioChanA { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_radio_chan_b<F>(
        &mut self,
        control: VhfRadioChanB,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfRadioChanB { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_radio_chan_c<F>(
        &mut self,
        control: VhfRadioChanC,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfRadioChanC { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_radio_chan_d<F>(
        &mut self,
        control: VhfRadioChanD,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfRadioChanD { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_radio_d_light<F>(
        &mut self,
        control: VhfRadioDLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfRadioDLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_radio_on_off<F>(
        &mut self,
        control: VhfRadioOnOff,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfRadioOnOff { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_radio_tx_light<F>(
        &mut self,
        control: VhfRadioTxLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfRadioTxLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_warning_radar_light<F>(
        &mut self,
        control: WarningRadarLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WarningRadarLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_warning_radar_power<F>(
        &mut self,
        control: WarningRadarPower,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WarningRadarPower { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_warning_radar_test<F>(
        &mut self,
        control: WarningRadarTest,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WarningRadarTest { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_windshield_oil<F>(
        &mut self,
        control: WindshieldOil,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WindshieldOil { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_windshieldcracks<F>(
        &mut self,
        control: Windshieldcracks,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Windshieldcracks { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_windshieldoil<F>(
        &mut self,
        control: Windshieldoil,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Windshieldoil { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wing_lts<F>(
        &mut self,
        control: WingLts,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WingLts { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wing_span_sel<F>(
        &mut self,
        control: WingSpanSel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WingSpanSel { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn build(self) -> P51d<'a, MAX_CONTROLS, MAX_PACKET_DATA> {
        P51d {
            parser: ProtocolParser::new(),
            controls: self.controls,
            saw_frame: false,
        }
    }
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> Default
    for P51dBuilder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
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

pub struct P51d<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    parser: ProtocolParser<MAX_PACKET_DATA>,
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    saw_frame: bool,
}

impl<const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    P51d<'_, MAX_CONTROLS, MAX_PACKET_DATA>
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
