//! Generated from model-definitions/P-51D.json.
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

    /// Accelerometer Main
    /// Output: gauge position
    pub struct AccelerometerMain(AnalogGauge<20588, 65535, 0>);
    impl AccelerometerMain {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct AccelerometerMax(AnalogGauge<20592, 65535, 0>);
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
    pub struct AccelerometerMin(AnalogGauge<20590, 65535, 0>);
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AccelerometerRstValue {
        Select1,
        Select2,
    }
    pub type AccelerometerRstValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(AccelerometerRstValue) + 'a>;
    impl core::convert::TryFrom<u16> for AccelerometerRstValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// G-meter Reset
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AccelerometerRst(Selector<20586, 4096, 12>);
    impl AccelerometerRst {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod airspeedindicator {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Airspeed Needle
    /// Output: gauge position
    pub struct AirspeedNeedle(AnalogGauge<20528, 65535, 0>);
    impl AirspeedNeedle {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
}

pub mod altimeter {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Altimeter 10000 Foot
    /// Output: gauge position
    pub struct Altimeter10000Foot(AnalogGauge<20538, 65535, 0>);
    impl Altimeter10000Foot {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct Altimeter1000Foot(AnalogGauge<20536, 65535, 0>);
    impl Altimeter1000Foot {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct Altimeter100Foot(AnalogGauge<20534, 65535, 0>);
    impl Altimeter100Foot {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct AltimeterPressure(AnalogGauge<20532, 65535, 0>);
    impl AltimeterPressure {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Set Pressure
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct SetPressure(AnalogDial<20530, 65535, 0>);
    impl SetPressure {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
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
}

pub mod artificialhorizon {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Artificial Horizon - Bank
    /// Output: gauge position
    pub struct AhorizonBank(AnalogGauge<20542, 65535, 0>);
    impl AhorizonBank {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct AhorizonCaged(AnalogGauge<20548, 65535, 0>);
    impl AhorizonCaged {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct AhorizonPitch(AnalogGauge<20544, 65535, 0>);
    impl AhorizonPitch {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct AhorizonPitchShift(AnalogGauge<20546, 65535, 0>);
    impl AhorizonPitchShift {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CageButtonValue {
        Select1,
        Select2,
    }
    pub type CageButtonValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CageButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for CageButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Artificial Horizon Cage
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CageButton(Selector<20526, 512, 9>);
    impl CageButton {
        pub const fn new() -> Self {
            Self(Selector::new())
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
    pub struct CageRotary(LimitedDial<20552, 65535, 0>);
    impl CageRotary {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    /// Pitch Adjustment
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct PitchAdjust(AnalogDial<20550, 65535, 0>);
    impl PitchAdjust {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
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
}

pub mod clock {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Clock Adjust
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct ClockAdjust(AnalogDial<20560, 65535, 0>);
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

    /// Clock Adjust Button
    /// Input: switch to previous or next state
    /// Output: selector position
    pub struct ClockAdjustButton(Selector<20526, 2048, 11>);
    impl ClockAdjustButton {
        pub const fn new() -> Self {
            Self(Selector::new())
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
    pub struct ClockHours(AnalogGauge<20562, 65535, 0>);
    impl ClockHours {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct ClockMinutes(AnalogGauge<20564, 65535, 0>);
    impl ClockMinutes {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct ClockSeconds(AnalogGauge<20566, 65535, 0>);
    impl ClockSeconds {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
}

pub mod cockpitmechanical {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ArmRestValue {
        Select1,
        Select2,
    }
    pub type ArmRestValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ArmRestValue) + 'a>;
    impl core::convert::TryFrom<u16> for ArmRestValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Arm rest
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ArmRest(Selector<20494, 8, 3>);
    impl ArmRest {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CanopyEmergencyReleaseHandleValue {
        Select1,
        Select2,
    }
    pub type CanopyEmergencyReleaseHandleValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(CanopyEmergencyReleaseHandleValue) + 'a>;
    impl core::convert::TryFrom<u16> for CanopyEmergencyReleaseHandleValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Canopy Emergency Release Handle
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CanopyEmergencyReleaseHandle(Selector<20494, 16, 4>);
    impl CanopyEmergencyReleaseHandle {
        pub const fn new() -> Self {
            Self(Selector::new())
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
    pub struct CanopyHandCrank(LimitedDial<20504, 65535, 0>);
    impl CanopyHandCrank {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MicrophoneOnValue {
        Select1,
        Select2,
    }
    pub type MicrophoneOnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MicrophoneOnValue) + 'a>;
    impl core::convert::TryFrom<u16> for MicrophoneOnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Microphone On
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MicrophoneOn(Selector<20494, 4, 2>);
    impl MicrophoneOn {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod controlsystem {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Aileron Trim
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct AileronTrim(AnalogDial<20516, 65535, 0>);
    impl AileronTrim {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
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

    /// Elevator Trim
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct ElevatorTrim(AnalogDial<20518, 65535, 0>);
    impl ElevatorTrim {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
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

    /// Flaps Control Handle
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct FlapsControlHandle(LimitedDial<20522, 65535, 0>);
    impl FlapsControlHandle {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LandingGearContr0lHandleValue {
        Select1,
        Select2,
    }
    pub type LandingGearContr0lHandleValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(LandingGearContr0lHandleValue) + 'a>;
    impl core::convert::TryFrom<u16> for LandingGearContr0lHandleValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Landing Gear Control Handle
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LandingGearContr0lHandle(Selector<20514, 4, 2>);
    impl LandingGearContr0lHandle {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LockStickFrwdNeutValue {
        Select1,
        Select2,
    }
    pub type LockStickFrwdNeutValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(LockStickFrwdNeutValue) + 'a>;
    impl core::convert::TryFrom<u16> for LockStickFrwdNeutValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Lock Stick Forward/Neutral
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LockStickFrwdNeut(Selector<20514, 8, 3>);
    impl LockStickFrwdNeut {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ParkBrakeHandleValue {
        Select1,
        Select2,
    }
    pub type ParkBrakeHandleValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(ParkBrakeHandleValue) + 'a>;
    impl core::convert::TryFrom<u16> for ParkBrakeHandleValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Parking Brake Handle
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ParkBrakeHandle(Selector<20514, 2, 1>);
    impl ParkBrakeHandle {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// Rudder Trim
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct RudderTrim(AnalogDial<20520, 65535, 0>);
    impl RudderTrim {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
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
    pub enum SafeLndGearLtTestValue {
        Select1,
        Select2,
    }
    pub type SafeLndGearLtTestValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(SafeLndGearLtTestValue) + 'a>;
    impl core::convert::TryFrom<u16> for SafeLndGearLtTestValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Safe Landing Gear Light Test
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SafeLndGearLtTest(Selector<20494, 32768, 15>);
    impl SafeLndGearLtTest {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UnsafeLndGearLtTestValue {
        Select1,
        Select2,
    }
    pub type UnsafeLndGearLtTestValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(UnsafeLndGearLtTestValue) + 'a>;
    impl core::convert::TryFrom<u16> for UnsafeLndGearLtTestValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Unsafe Landing Gear Light Test
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UnsafeLndGearLtTest(Selector<20514, 1, 0>);
    impl UnsafeLndGearLtTest {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod damage {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Windshield Crack Holes
    /// Output: gauge position
    pub struct Windshieldcracks(AnalogGauge<20670, 65535, 0>);
    impl Windshieldcracks {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct Windshieldoil(Led<20586, 32768, 15>);
    impl Windshieldoil {
        pub const fn new() -> Self {
            Self(Led::new())
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
}

pub mod detrola {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Detrola Frequency Selector
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct DetrolaFrequency(LimitedDial<20582, 65535, 0>);
    impl DetrolaFrequency {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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
    pub struct DetrolaVolume(LimitedDial<20584, 65535, 0>);
    impl DetrolaVolume {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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
}

pub mod directionalgyro {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Directional Gyro Heading
    /// Output: gauge position
    pub struct DirectionalGyro(AnalogGauge<20554, 65535, 0>);
    impl DirectionalGyro {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HdgCageButtonValue {
        Select1,
        Select2,
    }
    pub type HdgCageButtonValueCallback<'a> = alloc::boxed::Box<dyn FnMut(HdgCageButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for HdgCageButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Directional Gyro Cage
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct HdgCageButton(Selector<20526, 1024, 10>);
    impl HdgCageButton {
        pub const fn new() -> Self {
            Self(Selector::new())
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
    pub struct HdgCageRotary(AnalogDial<20556, 65535, 0>);
    impl HdgCageRotary {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
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
}

pub mod electricsystem {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Ammeter
    /// Output: gauge position
    pub struct Ammeter(AnalogGauge<20614, 65535, 0>);
    impl Ammeter {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
}

pub mod enginecontrolpanel {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FuelBoosterValue {
        Select1,
        Select2,
    }
    pub type FuelBoosterValueCallback<'a> = alloc::boxed::Box<dyn FnMut(FuelBoosterValue) + 'a>;
    impl core::convert::TryFrom<u16> for FuelBoosterValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Fuel Booster On/Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FuelBooster(Selector<20526, 16, 4>);
    impl FuelBooster {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// High Blower Lamp
    /// Output: 0 if light is off, 1 if light is on
    pub struct HighBlowerLamp(Led<20526, 8, 3>);
    impl HighBlowerLamp {
        pub const fn new() -> Self {
            Self(Led::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HighBlowerTestLampValue {
        Select1,
        Select2,
    }
    pub type HighBlowerTestLampValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(HighBlowerTestLampValue) + 'a>;
    impl core::convert::TryFrom<u16> for HighBlowerTestLampValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// High Blower Lamp Test
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct HighBlowerTestLamp(Selector<20526, 4, 2>);
    impl HighBlowerTestLamp {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum OilDiluteValue {
        Select1,
        Select2,
    }
    pub type OilDiluteValueCallback<'a> = alloc::boxed::Box<dyn FnMut(OilDiluteValue) + 'a>;
    impl core::convert::TryFrom<u16> for OilDiluteValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Oil Dilute Activate
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct OilDilute(Selector<20526, 32, 5>);
    impl OilDilute {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PrimerValue {
        Select1,
        Select2,
    }
    pub type PrimerValueCallback<'a> = alloc::boxed::Box<dyn FnMut(PrimerValue) + 'a>;
    impl core::convert::TryFrom<u16> for PrimerValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Primer Activate
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Primer(Selector<20526, 256, 8>);
    impl Primer {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum StarterValue {
        Select1,
        Select2,
    }
    pub type StarterValueCallback<'a> = alloc::boxed::Box<dyn FnMut(StarterValue) + 'a>;
    impl core::convert::TryFrom<u16> for StarterValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Starter Activate
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Starter(Selector<20526, 64, 6>);
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
    pub enum StarterCoverValue {
        Select1,
        Select2,
    }
    pub type StarterCoverValueCallback<'a> = alloc::boxed::Box<dyn FnMut(StarterCoverValue) + 'a>;
    impl core::convert::TryFrom<u16> for StarterCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Starter Switch Cover
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct StarterCover(Selector<20526, 128, 7>);
    impl StarterCover {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SuperchargerAutoLowHighValue {
        Select1,
        Select2,
        Select3,
    }
    pub type SuperchargerAutoLowHighValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(SuperchargerAutoLowHighValue) + 'a>;
    impl core::convert::TryFrom<u16> for SuperchargerAutoLowHighValue {
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

    /// Supercharger Auto/Low/High
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct SuperchargerAutoLowHigh(Selector<20526, 3, 0>);
    impl SuperchargerAutoLowHigh {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SuperchargerSwitchCoverValue {
        Select1,
        Select2,
    }
    pub type SuperchargerSwitchCoverValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(SuperchargerSwitchCoverValue) + 'a>;
    impl core::convert::TryFrom<u16> for SuperchargerSwitchCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Supercharger Switch Cover
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SuperchargerSwitchCover(Selector<20514, 32768, 15>);
    impl SuperchargerSwitchCover {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod enginesystem {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Carburetor Cold Air Control
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct CarbColdAir(LimitedDial<20490, 65535, 0>);
    impl CarbColdAir {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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
    pub struct CarbTemp(AnalogGauge<20606, 65535, 0>);
    impl CarbTemp {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct CarbWarmAir(LimitedDial<20492, 65535, 0>);
    impl CarbWarmAir {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CoolantControlValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type CoolantControlValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(CoolantControlValue) + 'a>;
    impl core::convert::TryFrom<u16> for CoolantControlValue {
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

    /// Coolant Control
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct CoolantControl(Selector<20484, 6144, 11>);
    impl CoolantControl {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CoolantControlCoverValue {
        Select1,
        Select2,
    }
    pub type CoolantControlCoverValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(CoolantControlCoverValue) + 'a>;
    impl core::convert::TryFrom<u16> for CoolantControlCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Coolant Control Cover
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CoolantControlCover(Selector<20484, 1024, 10>);
    impl CoolantControlCover {
        pub const fn new() -> Self {
            Self(Selector::new())
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
    pub struct CoolantTemp(AnalogGauge<20608, 65535, 0>);
    impl CoolantTemp {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Engine RPM
    /// Output: gauge position
    pub struct EngineRpm(AnalogGauge<20594, 65535, 0>);
    impl EngineRpm {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Lock Propeller & Mixture
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct LockPropellerMixture(LimitedDial<20502, 65535, 0>);
    impl LockPropellerMixture {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    /// Lock Throttle
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct LockThrottle(LimitedDial<20500, 65535, 0>);
    impl LockThrottle {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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
    pub struct ManifoldPressure(AnalogGauge<20600, 65535, 0>);
    impl ManifoldPressure {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MixtureControlValue {
        Select1,
        Select2,
        Select3,
    }
    pub type MixtureControlValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(MixtureControlValue) + 'a>;
    impl core::convert::TryFrom<u16> for MixtureControlValue {
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

    /// Mixture Control
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct MixtureControl(Selector<20494, 3, 0>);
    impl MixtureControl {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum OilControlValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type OilControlValueCallback<'a> = alloc::boxed::Box<dyn FnMut(OilControlValue) + 'a>;
    impl core::convert::TryFrom<u16> for OilControlValue {
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

    /// Oil Control
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct OilControl(Selector<20484, 49152, 14>);
    impl OilControl {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum OilControlCoverValue {
        Select1,
        Select2,
    }
    pub type OilControlCoverValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(OilControlCoverValue) + 'a>;
    impl core::convert::TryFrom<u16> for OilControlCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Oil Control Cover
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct OilControlCover(Selector<20484, 8192, 13>);
    impl OilControlCover {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// Oil Pressure
    /// Output: gauge position
    pub struct OilPres(AnalogGauge<20612, 65535, 0>);
    impl OilPres {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Oil Temperature
    /// Output: gauge position
    pub struct OilTemp(AnalogGauge<20610, 65535, 0>);
    impl OilTemp {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Propeller RPM
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct PropellerRpm(LimitedDial<20498, 65535, 0>);
    impl PropellerRpm {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    /// Throttle
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct Throttle(LimitedDial<20496, 65535, 0>);
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

    /// Vacuum System
    /// Output: gauge position
    pub struct VacuumSuction(AnalogGauge<20604, 65535, 0>);
    impl VacuumSuction {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
}

pub mod environmentsystem {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Cold Air
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct ColdAir(LimitedDial<20510, 65535, 0>);
    impl ColdAir {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    /// Defroster
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct Defroster(LimitedDial<20508, 65535, 0>);
    impl Defroster {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    /// Hot Air
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct HotAir(LimitedDial<20512, 65535, 0>);
    impl HotAir {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    /// Windshield Oil
    /// Output: gauge position
    pub struct WindshieldOil(AnalogGauge<20616, 65535, 0>);
    impl WindshieldOil {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
}

pub mod externalaircraftmodel {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Formation Lights
    /// Output: Formation Lights
    pub struct ExtFormationLights(Metadata<20630, 65535, 0>);
    impl ExtFormationLights {
        pub const fn new() -> Self {
            Self(Metadata::new())
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
    pub struct ExtPositionLightLeft(Metadata<20586, 8192, 13>);
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
    pub struct ExtPositionLightRight(Metadata<20586, 16384, 14>);
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
}

pub mod frontswitchbox {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Cockpit Lights
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct CockpitLights(LimitedDial<20524, 65535, 0>);
    impl CockpitLights {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GunControlValue {
        Select1,
        Select2,
        Select3,
    }
    pub type GunControlValueCallback<'a> = alloc::boxed::Box<dyn FnMut(GunControlValue) + 'a>;
    impl core::convert::TryFrom<u16> for GunControlValue {
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

    /// Gun/Camera Control
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct GunControl(Selector<20514, 192, 6>);
    impl GunControl {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IgnitionValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type IgnitionValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IgnitionValue) + 'a>;
    impl core::convert::TryFrom<u16> for IgnitionValue {
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

    /// Ignition Off/Right/Left/Both
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Ignition(Selector<20514, 48, 4>);
    impl Ignition {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftBombArmChemValue {
        Select1,
        Select2,
        Select3,
    }
    pub type LeftBombArmChemValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(LeftBombArmChemValue) + 'a>;
    impl core::convert::TryFrom<u16> for LeftBombArmChemValue {
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

    /// Left Bomb Arm/Chemical Release
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct LeftBombArmChem(Selector<20514, 1536, 9>);
    impl LeftBombArmChem {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightBombArmChemValue {
        Select1,
        Select2,
        Select3,
    }
    pub type RightBombArmChemValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(RightBombArmChemValue) + 'a>;
    impl core::convert::TryFrom<u16> for RightBombArmChemValue {
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

    /// Right Bomb Arm/Chemical Release
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct RightBombArmChem(Selector<20514, 6144, 11>);
    impl RightBombArmChem {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RocketsBombsModesValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type RocketsBombsModesValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(RocketsBombsModesValue) + 'a>;
    impl core::convert::TryFrom<u16> for RocketsBombsModesValue {
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

    /// Rockets/Bombs Modes
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct RocketsBombsModes(Selector<20514, 24576, 13>);
    impl RocketsBombsModes {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SilenceGearHornValue {
        Select1,
        Select2,
    }
    pub type SilenceGearHornValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(SilenceGearHornValue) + 'a>;
    impl core::convert::TryFrom<u16> for SilenceGearHornValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Silence Landing Gear Warning Horn Cut Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SilenceGearHorn(Selector<20514, 256, 8>);
    impl SilenceGearHorn {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod fuelsystem {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Fuel Pressure
    /// Output: gauge position
    pub struct FuelPressure(AnalogGauge<20596, 65535, 0>);
    impl FuelPressure {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FuelSelectorValveValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type FuelSelectorValveValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FuelSelectorValveValue) + 'a>;
    impl core::convert::TryFrom<u16> for FuelSelectorValveValue {
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

    /// Fuel Selector Valve
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct FuelSelectorValve(Selector<20494, 1792, 8>);
    impl FuelSelectorValve {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FuelShutOffValveValue {
        Select1,
        Select2,
    }
    pub type FuelShutOffValveValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FuelShutOffValveValue) + 'a>;
    impl core::convert::TryFrom<u16> for FuelShutOffValveValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Fuel Shut-Off Valve
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FuelShutOffValve(Selector<20494, 2048, 11>);
    impl FuelShutOffValve {
        pub const fn new() -> Self {
            Self(Selector::new())
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
    pub struct FuelTankFuselage(AnalogGauge<20628, 65535, 0>);
    impl FuelTankFuselage {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct FuelTankLeft(AnalogGauge<20624, 65535, 0>);
    impl FuelTankLeft {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct FuelTankRight(AnalogGauge<20626, 65535, 0>);
    impl FuelTankRight {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
}

pub mod gaugevalues {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Accelerometer
    /// Output: Accelerometer
    pub struct AccelerometerValue(Metadata<20668, 65535, 0>);
    impl AccelerometerValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// Airspeed MPH
    /// Output: Airspeed MPH
    pub struct AirspeedMphValue(Metadata<20632, 65535, 0>);
    impl AirspeedMphValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// Altimeter
    /// Output: Altimeter
    pub struct AltimeterValue(Metadata<20634, 65535, 0>);
    impl AltimeterValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// Ammeter
    /// Output: Ammeter
    pub struct AmmeterValue(Metadata<20656, 65535, 0>);
    impl AmmeterValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// Barometric Pressure
    /// Output: Barometric Pressure
    pub struct BarometricPressureValue(Metadata<20666, 65535, 0>);
    impl BarometricPressureValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// Remote Compass Course
    /// Output: Remote Compass Course
    pub struct CourseValue(Metadata<20642, 65535, 0>);
    impl CourseValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// Directional Gyro
    /// Output: Directional Gyro
    pub struct DirectionalGyroValue(Metadata<20638, 65535, 0>);
    impl DirectionalGyroValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// Engine RPM Value
    /// Output: Engine RPM Value
    pub struct EngineRpmValue(Metadata<20636, 65535, 0>);
    impl EngineRpmValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// Fuel Pressure
    /// Output: Fuel Pressure
    pub struct FuelPressureValue(Metadata<20644, 65535, 0>);
    impl FuelPressureValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// Fuselage Fuel Tank Gallons
    /// Output: Fuselage Fuel Tank Gallons
    pub struct FuselageFuelTankValue(Metadata<20664, 65535, 0>);
    impl FuselageFuelTankValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// Remote Compass Heading
    /// Output: Remote Compass Heading
    pub struct HeadingValue(Metadata<20640, 65535, 0>);
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

    /// Hydraulic Pressure
    /// Output: Hydraulic Pressure
    pub struct HydraulicPressureValue(Metadata<20646, 65535, 0>);
    impl HydraulicPressureValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// Left Fuel Tank Gallons
    /// Output: Left Fuel Tank Gallons
    pub struct LeftFuelTankValue(Metadata<20660, 65535, 0>);
    impl LeftFuelTankValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// Manifold Pressure
    /// Output: Manifold Pressure
    pub struct ManifoldPressureValue(Metadata<20648, 65535, 0>);
    impl ManifoldPressureValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// Oil Pressure
    /// Output: Oil Pressure
    pub struct OilPressureValue(Metadata<20654, 65535, 0>);
    impl OilPressureValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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
    /// Output: Oil Temperature
    pub struct OilTemperatureValue(Metadata<20652, 65535, 0>);
    impl OilTemperatureValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// Oxygen Pressure
    /// Output: Oxygen Pressure
    pub struct OxygenPressureValue(Metadata<20658, 65535, 0>);
    impl OxygenPressureValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// Right Fuel Tank Gallons
    /// Output: Right Fuel Tank Gallons
    pub struct RightFuelTankValue(Metadata<20662, 65535, 0>);
    impl RightFuelTankValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// Vacuum Suction read as X.X or XX.X
    /// Output: Vacuum Suction read as X.X or XX.X
    pub struct VacuumSuctionValue(Metadata<20650, 65535, 0>);
    impl VacuumSuctionValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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
}

pub mod homingadapter {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HomingAdapterCbValue {
        Select1,
        Select2,
    }
    pub type HomingAdapterCbValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(HomingAdapterCbValue) + 'a>;
    impl core::convert::TryFrom<u16> for HomingAdapterCbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Homing Adapter Circuit Breaker
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct HomingAdapterCb(Selector<20586, 512, 9>);
    impl HomingAdapterCb {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HomingAdapterModeValue {
        Select1,
        Select2,
        Select3,
    }
    pub type HomingAdapterModeValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(HomingAdapterModeValue) + 'a>;
    impl core::convert::TryFrom<u16> for HomingAdapterModeValue {
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

    /// Homing Adapter Mode
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct HomingAdapterMode(Selector<20586, 3072, 10>);
    impl HomingAdapterMode {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HomingAdapterPowerValue {
        Select1,
        Select2,
    }
    pub type HomingAdapterPowerValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(HomingAdapterPowerValue) + 'a>;
    impl core::convert::TryFrom<u16> for HomingAdapterPowerValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Homing Adapter On/Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct HomingAdapterPower(Selector<20586, 256, 8>);
    impl HomingAdapterPower {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod hydraulicsystem {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HydRelValue {
        Select1,
        Select2,
    }
    pub type HydRelValueCallback<'a> = alloc::boxed::Box<dyn FnMut(HydRelValue) + 'a>;
    impl core::convert::TryFrom<u16> for HydRelValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Hydraulic Release Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct HydRel(Selector<20484, 16, 4>);
    impl HydRel {
        pub const fn new() -> Self {
            Self(Selector::new())
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
    pub struct HydraulicPressure(AnalogGauge<20598, 65535, 0>);
    impl HydraulicPressure {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
}

pub mod iff {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffCodeValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
    }
    pub type IffCodeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffCodeValue) + 'a>;
    impl core::convert::TryFrom<u16> for IffCodeValue {
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

    /// IFF Code 1-6
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffCode(Selector<20586, 224, 5>);
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
    pub enum IffDetCircuitValue {
        Select1,
        Select2,
    }
    pub type IffDetCircuitValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffDetCircuitValue) + 'a>;
    impl core::convert::TryFrom<u16> for IffDetCircuitValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// IFF Detonator Circuit On/Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IffDetCircuit(Selector<20586, 1, 0>);
    impl IffDetCircuit {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffDetLeftValue {
        Select1,
        Select2,
    }
    pub type IffDetLeftValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffDetLeftValue) + 'a>;
    impl core::convert::TryFrom<u16> for IffDetLeftValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// IFF Detonator Left
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IffDetLeft(Selector<20586, 2, 1>);
    impl IffDetLeft {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffDetRightValue {
        Select1,
        Select2,
    }
    pub type IffDetRightValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffDetRightValue) + 'a>;
    impl core::convert::TryFrom<u16> for IffDetRightValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// IFF Detonator Right
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IffDetRight(Selector<20586, 4, 2>);
    impl IffDetRight {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffDistressValue {
        Select1,
        Select2,
    }
    pub type IffDistressValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffDistressValue) + 'a>;
    impl core::convert::TryFrom<u16> for IffDistressValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// IFF Distress Signal On/Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IffDistress(Selector<20574, 32768, 15>);
    impl IffDistress {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffTimeOffOnValue {
        Select1,
        Select2,
        Select3,
    }
    pub type IffTimeOffOnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffTimeOffOnValue) + 'a>;
    impl core::convert::TryFrom<u16> for IffTimeOffOnValue {
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

    /// IFF Time/Off/On
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffTimeOffOn(Selector<20586, 24, 3>);
    impl IffTimeOffOn {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum OffPowerValue {
        Select1,
        Select2,
    }
    pub type OffPowerValueCallback<'a> = alloc::boxed::Box<dyn FnMut(OffPowerValue) + 'a>;
    impl core::convert::TryFrom<u16> for OffPowerValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// IFF Power On/Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct OffPower(Selector<20574, 16384, 14>);
    impl OffPower {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod k14gunsight {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Gun Sight Brightness
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct GunsightBrt(LimitedDial<20486, 65535, 0>);
    impl GunsightBrt {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GunsightFixedGyroValue {
        Select1,
        Select2,
        Select3,
    }
    pub type GunsightFixedGyroValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(GunsightFixedGyroValue) + 'a>;
    impl core::convert::TryFrom<u16> for GunsightFixedGyroValue {
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

    /// Fixed Fixed-Gyro Gyro
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct GunsightFixedGyro(Selector<20484, 12, 2>);
    impl GunsightFixedGyro {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GunsightOnOffValue {
        Select1,
        Select2,
    }
    pub type GunsightOnOffValueCallback<'a> = alloc::boxed::Box<dyn FnMut(GunsightOnOffValue) + 'a>;
    impl core::convert::TryFrom<u16> for GunsightOnOffValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Gunsight On/Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct GunsightOnOff(Selector<20484, 1, 0>);
    impl GunsightOnOff {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RetMaskLvrValue {
        Select1,
        Select2,
    }
    pub type RetMaskLvrValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RetMaskLvrValue) + 'a>;
    impl core::convert::TryFrom<u16> for RetMaskLvrValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Fixed Reticle Mask Lever
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RetMaskLvr(Selector<20484, 2, 1>);
    impl RetMaskLvr {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// Wing Span Selector
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct WingSpanSel(LimitedDial<20488, 65535, 0>);
    impl WingSpanSel {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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
}

pub mod lightsystem {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Landing Gear Green Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct LandingGearGreen(Led<20494, 8192, 13>);
    impl LandingGearGreen {
        pub const fn new() -> Self {
            Self(Led::new())
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
    pub struct LandingGearRed(Led<20494, 16384, 14>);
    impl LandingGearRed {
        pub const fn new() -> Self {
            Self(Led::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LandingLightValue {
        Select1,
        Select2,
    }
    pub type LandingLightValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LandingLightValue) + 'a>;
    impl core::convert::TryFrom<u16> for LandingLightValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Landing Light On/Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LandingLight(Selector<20494, 4096, 12>);
    impl LandingLight {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// Left Fluorescent Light
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct LeftFluorescentLight(LimitedDial<20506, 65535, 0>);
    impl LeftFluorescentLight {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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
}

pub mod oxygensystem {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum OxygenAutoMixValue {
        Select1,
        Select2,
    }
    pub type OxygenAutoMixValueCallback<'a> = alloc::boxed::Box<dyn FnMut(OxygenAutoMixValue) + 'a>;
    impl core::convert::TryFrom<u16> for OxygenAutoMixValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Auto-Mix On-Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct OxygenAutoMix(Selector<20494, 32, 5>);
    impl OxygenAutoMix {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum OxygenByPassValue {
        Select1,
        Select2,
    }
    pub type OxygenByPassValueCallback<'a> = alloc::boxed::Box<dyn FnMut(OxygenByPassValue) + 'a>;
    impl core::convert::TryFrom<u16> for OxygenByPassValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Oxygen Emergency By-pass
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct OxygenByPass(Selector<20494, 64, 6>);
    impl OxygenByPass {
        pub const fn new() -> Self {
            Self(Selector::new())
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
    pub struct OxygenFlow(AnalogGauge<20620, 65535, 0>);
    impl OxygenFlow {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct OxygenPres(AnalogGauge<20618, 65535, 0>);
    impl OxygenPres {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Oxygen Warning Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct OxygenWarning(Led<20494, 128, 7>);
    impl OxygenWarning {
        pub const fn new() -> Self {
            Self(Led::new())
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
}

pub mod remotecompass {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Remote Compass Course
    /// Output: gauge position
    pub struct RemoteCompassCrs(AnalogGauge<20572, 65535, 0>);
    impl RemoteCompassCrs {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct RemoteCompassHdg(AnalogGauge<20570, 65535, 0>);
    impl RemoteCompassHdg {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct RemoteCompassSet(AnalogDial<20568, 65535, 0>);
    impl RemoteCompassSet {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
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
}

pub mod rightswitchpanel {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmbrRecLtValue {
        Select1,
        Select2,
        Select3,
    }
    pub type AmbrRecLtValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AmbrRecLtValue) + 'a>;
    impl core::convert::TryFrom<u16> for AmbrRecLtValue {
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

    /// Amber Recognition Light Key/Off/Steady
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct AmbrRecLt(Selector<20480, 12288, 12>);
    impl AmbrRecLt {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BatValue {
        Select1,
        Select2,
    }
    pub type BatValueCallback<'a> = alloc::boxed::Box<dyn FnMut(BatValue) + 'a>;
    impl core::convert::TryFrom<u16> for BatValue {
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
    pub struct Bat(Selector<20480, 2, 1>);
    impl Bat {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CircProRstValue {
        Select1,
        Select2,
    }
    pub type CircProRstValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CircProRstValue) + 'a>;
    impl core::convert::TryFrom<u16> for CircProRstValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Circuit Protectors Reset
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CircProRst(Selector<20480, 32768, 15>);
    impl CircProRst {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GenValue {
        Select1,
        Select2,
    }
    pub type GenValueCallback<'a> = alloc::boxed::Box<dyn FnMut(GenValue) + 'a>;
    impl core::convert::TryFrom<u16> for GenValue {
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
    pub struct Gen(Selector<20480, 1, 0>);
    impl Gen {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GrnRecLtValue {
        Select1,
        Select2,
        Select3,
    }
    pub type GrnRecLtValueCallback<'a> = alloc::boxed::Box<dyn FnMut(GrnRecLtValue) + 'a>;
    impl core::convert::TryFrom<u16> for GrnRecLtValue {
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

    /// Green Recognition Light Key/Off/Steady
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct GrnRecLt(Selector<20480, 3072, 10>);
    impl GrnRecLt {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GunHeatValue {
        Select1,
        Select2,
    }
    pub type GunHeatValueCallback<'a> = alloc::boxed::Box<dyn FnMut(GunHeatValue) + 'a>;
    impl core::convert::TryFrom<u16> for GunHeatValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Gun Heating
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct GunHeat(Selector<20480, 4, 2>);
    impl GunHeat {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// Pitot Heating
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Pitot(Selector<20480, 8, 3>);
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
    pub enum RecLtsKeyValue {
        Select1,
        Select2,
    }
    pub type RecLtsKeyValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RecLtsKeyValue) + 'a>;
    impl core::convert::TryFrom<u16> for RecLtsKeyValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Recognition Lights Key
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RecLtsKey(Selector<20480, 16384, 14>);
    impl RecLtsKey {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RedRecLtValue {
        Select1,
        Select2,
        Select3,
    }
    pub type RedRecLtValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RedRecLtValue) + 'a>;
    impl core::convert::TryFrom<u16> for RedRecLtValue {
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

    /// Red Recognition Light Key/Off/Steady
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct RedRecLt(Selector<20480, 768, 8>);
    impl RedRecLt {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// Right Fluorescent Light
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RtFlresLt(LimitedDial<20482, 65535, 0>);
    impl RtFlresLt {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TailLtsValue {
        Select1,
        Select2,
        Select3,
    }
    pub type TailLtsValueCallback<'a> = alloc::boxed::Box<dyn FnMut(TailLtsValue) + 'a>;
    impl core::convert::TryFrom<u16> for TailLtsValue {
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

    /// Tail Position Lights Bright/Off/Dim
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct TailLts(Selector<20480, 192, 6>);
    impl TailLts {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WingLtsValue {
        Select1,
        Select2,
        Select3,
    }
    pub type WingLtsValueCallback<'a> = alloc::boxed::Box<dyn FnMut(WingLtsValue) + 'a>;
    impl core::convert::TryFrom<u16> for WingLtsValue {
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

    /// Wing Position Lights Bright/Off/Dim
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct WingLts(Selector<20480, 48, 4>);
    impl WingLts {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod tailwarningradar {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Radar Warning Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct RadarWarningLight(Led<20574, 8192, 13>);
    impl RadarWarningLight {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Tail Warning Radar Light Control
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct WarningRadarLight(LimitedDial<20580, 65535, 0>);
    impl WarningRadarLight {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WarningRadarPowerValue {
        Select1,
        Select2,
    }
    pub type WarningRadarPowerValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(WarningRadarPowerValue) + 'a>;
    impl core::convert::TryFrom<u16> for WarningRadarPowerValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Tail Warning Radar Power
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct WarningRadarPower(Selector<20574, 2048, 11>);
    impl WarningRadarPower {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WarningRadarTestValue {
        Select1,
        Select2,
    }
    pub type WarningRadarTestValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(WarningRadarTestValue) + 'a>;
    impl core::convert::TryFrom<u16> for WarningRadarTestValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Tail Warning Radar Test
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct WarningRadarTest(Selector<20574, 4096, 12>);
    impl WarningRadarTest {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod turnindicator {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Slipball
    /// Output: gauge position
    pub struct Slipball(AnalogGauge<20622, 65535, 0>);
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

    /// Turn Indicator
    /// Output: gauge position
    pub struct TurnIndicator(AnalogGauge<20558, 65535, 0>);
    impl TurnIndicator {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
}

pub mod variometer {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Variometer-VVI
    /// Output: gauge position
    pub struct VariometerVvi(AnalogGauge<20540, 65535, 0>);
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
}

pub mod vhfradio {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Radio Lights Dimmer
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RadioLightsDimmer(LimitedDial<20576, 65535, 0>);
    impl RadioLightsDimmer {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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
    pub struct RadioMode2(Selector<20574, 12, 2>);
    impl RadioMode2 {
        pub const fn new() -> Self {
            Self(Selector::new())
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
    pub struct RadioMode3(Selector<20574, 48, 4>);
    impl RadioMode3 {
        pub const fn new() -> Self {
            Self(Selector::new())
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
    pub struct RadioVolume(LimitedDial<20578, 65535, 0>);
    impl RadioVolume {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum VhfLockingLeverValue {
        Select1,
        Select2,
    }
    pub type VhfLockingLeverValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(VhfLockingLeverValue) + 'a>;
    impl core::convert::TryFrom<u16> for VhfLockingLeverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Switch Locking Lever
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct VhfLockingLever(Selector<20574, 2, 1>);
    impl VhfLockingLever {
        pub const fn new() -> Self {
            Self(Selector::new())
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
    pub struct VhfRadioALight(Led<20574, 64, 6>);
    impl VhfRadioALight {
        pub const fn new() -> Self {
            Self(Led::new())
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
    pub struct VhfRadioBLight(Led<20574, 128, 7>);
    impl VhfRadioBLight {
        pub const fn new() -> Self {
            Self(Led::new())
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
    pub struct VhfRadioCLight(Led<20574, 256, 8>);
    impl VhfRadioCLight {
        pub const fn new() -> Self {
            Self(Led::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum VhfRadioChanAValue {
        Select1,
        Select2,
    }
    pub type VhfRadioChanAValueCallback<'a> = alloc::boxed::Box<dyn FnMut(VhfRadioChanAValue) + 'a>;
    impl core::convert::TryFrom<u16> for VhfRadioChanAValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// VHF Radio Channel A
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct VhfRadioChanA(Selector<20526, 8192, 13>);
    impl VhfRadioChanA {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum VhfRadioChanBValue {
        Select1,
        Select2,
    }
    pub type VhfRadioChanBValueCallback<'a> = alloc::boxed::Box<dyn FnMut(VhfRadioChanBValue) + 'a>;
    impl core::convert::TryFrom<u16> for VhfRadioChanBValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// VHF Radio Channel B
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct VhfRadioChanB(Selector<20526, 16384, 14>);
    impl VhfRadioChanB {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum VhfRadioChanCValue {
        Select1,
        Select2,
    }
    pub type VhfRadioChanCValueCallback<'a> = alloc::boxed::Box<dyn FnMut(VhfRadioChanCValue) + 'a>;
    impl core::convert::TryFrom<u16> for VhfRadioChanCValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// VHF Radio Channel C
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct VhfRadioChanC(Selector<20526, 32768, 15>);
    impl VhfRadioChanC {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum VhfRadioChanDValue {
        Select1,
        Select2,
    }
    pub type VhfRadioChanDValueCallback<'a> = alloc::boxed::Box<dyn FnMut(VhfRadioChanDValue) + 'a>;
    impl core::convert::TryFrom<u16> for VhfRadioChanDValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// VHF Radio Channel D
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct VhfRadioChanD(Selector<20574, 1, 0>);
    impl VhfRadioChanD {
        pub const fn new() -> Self {
            Self(Selector::new())
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
    pub struct VhfRadioDLight(Led<20574, 512, 9>);
    impl VhfRadioDLight {
        pub const fn new() -> Self {
            Self(Led::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum VhfRadioOnOffValue {
        Select1,
        Select2,
    }
    pub type VhfRadioOnOffValueCallback<'a> = alloc::boxed::Box<dyn FnMut(VhfRadioOnOffValue) + 'a>;
    impl core::convert::TryFrom<u16> for VhfRadioOnOffValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// VHF Radio On/Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct VhfRadioOnOff(Selector<20526, 4096, 12>);
    impl VhfRadioOnOff {
        pub const fn new() -> Self {
            Self(Selector::new())
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
    pub struct VhfRadioTxLight(Led<20574, 1024, 10>);
    impl VhfRadioTxLight {
        pub const fn new() -> Self {
            Self(Led::new())
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
}

pub mod weaponcontrol {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LftPylodSlvoValue {
        Select1,
        Select2,
    }
    pub type LftPylodSlvoValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LftPylodSlvoValue) + 'a>;
    impl core::convert::TryFrom<u16> for LftPylodSlvoValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Left Payload Salvo
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LftPylodSlvo(Selector<20484, 256, 8>);
    impl LftPylodSlvo {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RktDelSwitchValue {
        Select1,
        Select2,
    }
    pub type RktDelSwitchValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RktDelSwitchValue) + 'a>;
    impl core::convert::TryFrom<u16> for RktDelSwitchValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Rockets Delay Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RktDelSwitch(Selector<20484, 128, 7>);
    impl RktDelSwitch {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RktRelModeValue {
        Select1,
        Select2,
        Select3,
    }
    pub type RktRelModeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RktRelModeValue) + 'a>;
    impl core::convert::TryFrom<u16> for RktRelModeValue {
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

    /// Rockets Release Mode
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct RktRelMode(Selector<20484, 96, 5>);
    impl RktRelMode {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RtPylodSlvoValue {
        Select1,
        Select2,
    }
    pub type RtPylodSlvoValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RtPylodSlvoValue) + 'a>;
    impl core::convert::TryFrom<u16> for RtPylodSlvoValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Right Payload Salvo
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RtPylodSlvo(Selector<20484, 512, 9>);
    impl RtPylodSlvo {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod weaponsystem {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Rocket Counter
    /// Output: gauge position
    pub struct RocketCounter(AnalogGauge<20602, 65535, 0>);
    impl RocketCounter {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
}

pub enum RegisteredControl<'a> {
    AccelerometerMain {
        control: accelerometer::AccelerometerMain,
        callback: IntegerCallback<'a>,
    },
    AccelerometerMax {
        control: accelerometer::AccelerometerMax,
        callback: IntegerCallback<'a>,
    },
    AccelerometerMin {
        control: accelerometer::AccelerometerMin,
        callback: IntegerCallback<'a>,
    },
    AccelerometerRst {
        control: accelerometer::AccelerometerRst,
        callback: accelerometer::AccelerometerRstValueCallback<'a>,
    },
    AccelerometerValue {
        control: gaugevalues::AccelerometerValue,
        callback: IntegerCallback<'a>,
    },
    AhorizonBank {
        control: artificialhorizon::AhorizonBank,
        callback: IntegerCallback<'a>,
    },
    AhorizonCaged {
        control: artificialhorizon::AhorizonCaged,
        callback: IntegerCallback<'a>,
    },
    AhorizonPitch {
        control: artificialhorizon::AhorizonPitch,
        callback: IntegerCallback<'a>,
    },
    AhorizonPitchShift {
        control: artificialhorizon::AhorizonPitchShift,
        callback: IntegerCallback<'a>,
    },
    AileronTrim {
        control: controlsystem::AileronTrim,
        callback: IntegerCallback<'a>,
    },
    AirspeedMphValue {
        control: gaugevalues::AirspeedMphValue,
        callback: IntegerCallback<'a>,
    },
    AirspeedNeedle {
        control: airspeedindicator::AirspeedNeedle,
        callback: IntegerCallback<'a>,
    },
    Altimeter10000Foot {
        control: altimeter::Altimeter10000Foot,
        callback: IntegerCallback<'a>,
    },
    Altimeter1000Foot {
        control: altimeter::Altimeter1000Foot,
        callback: IntegerCallback<'a>,
    },
    Altimeter100Foot {
        control: altimeter::Altimeter100Foot,
        callback: IntegerCallback<'a>,
    },
    AltimeterPressure {
        control: altimeter::AltimeterPressure,
        callback: IntegerCallback<'a>,
    },
    AltimeterValue {
        control: gaugevalues::AltimeterValue,
        callback: IntegerCallback<'a>,
    },
    AmbrRecLt {
        control: rightswitchpanel::AmbrRecLt,
        callback: rightswitchpanel::AmbrRecLtValueCallback<'a>,
    },
    Ammeter {
        control: electricsystem::Ammeter,
        callback: IntegerCallback<'a>,
    },
    AmmeterValue {
        control: gaugevalues::AmmeterValue,
        callback: IntegerCallback<'a>,
    },
    ArmRest {
        control: cockpitmechanical::ArmRest,
        callback: cockpitmechanical::ArmRestValueCallback<'a>,
    },
    BarometricPressureValue {
        control: gaugevalues::BarometricPressureValue,
        callback: IntegerCallback<'a>,
    },
    Bat {
        control: rightswitchpanel::Bat,
        callback: rightswitchpanel::BatValueCallback<'a>,
    },
    CageButton {
        control: artificialhorizon::CageButton,
        callback: artificialhorizon::CageButtonValueCallback<'a>,
    },
    CageRotary {
        control: artificialhorizon::CageRotary,
        callback: IntegerCallback<'a>,
    },
    CanopyEmergencyReleaseHandle {
        control: cockpitmechanical::CanopyEmergencyReleaseHandle,
        callback: cockpitmechanical::CanopyEmergencyReleaseHandleValueCallback<'a>,
    },
    CanopyHandCrank {
        control: cockpitmechanical::CanopyHandCrank,
        callback: IntegerCallback<'a>,
    },
    CarbColdAir {
        control: enginesystem::CarbColdAir,
        callback: IntegerCallback<'a>,
    },
    CarbTemp {
        control: enginesystem::CarbTemp,
        callback: IntegerCallback<'a>,
    },
    CarbWarmAir {
        control: enginesystem::CarbWarmAir,
        callback: IntegerCallback<'a>,
    },
    CircProRst {
        control: rightswitchpanel::CircProRst,
        callback: rightswitchpanel::CircProRstValueCallback<'a>,
    },
    ClockAdjust {
        control: clock::ClockAdjust,
        callback: IntegerCallback<'a>,
    },
    ClockAdjustButton {
        control: clock::ClockAdjustButton,
        callback: IntegerCallback<'a>,
    },
    ClockHours {
        control: clock::ClockHours,
        callback: IntegerCallback<'a>,
    },
    ClockMinutes {
        control: clock::ClockMinutes,
        callback: IntegerCallback<'a>,
    },
    ClockSeconds {
        control: clock::ClockSeconds,
        callback: IntegerCallback<'a>,
    },
    CockpitLights {
        control: frontswitchbox::CockpitLights,
        callback: IntegerCallback<'a>,
    },
    ColdAir {
        control: environmentsystem::ColdAir,
        callback: IntegerCallback<'a>,
    },
    CoolantControl {
        control: enginesystem::CoolantControl,
        callback: enginesystem::CoolantControlValueCallback<'a>,
    },
    CoolantControlCover {
        control: enginesystem::CoolantControlCover,
        callback: enginesystem::CoolantControlCoverValueCallback<'a>,
    },
    CoolantTemp {
        control: enginesystem::CoolantTemp,
        callback: IntegerCallback<'a>,
    },
    CourseValue {
        control: gaugevalues::CourseValue,
        callback: IntegerCallback<'a>,
    },
    Defroster {
        control: environmentsystem::Defroster,
        callback: IntegerCallback<'a>,
    },
    DetrolaFrequency {
        control: detrola::DetrolaFrequency,
        callback: IntegerCallback<'a>,
    },
    DetrolaVolume {
        control: detrola::DetrolaVolume,
        callback: IntegerCallback<'a>,
    },
    DirectionalGyro {
        control: directionalgyro::DirectionalGyro,
        callback: IntegerCallback<'a>,
    },
    DirectionalGyroValue {
        control: gaugevalues::DirectionalGyroValue,
        callback: IntegerCallback<'a>,
    },
    ElevatorTrim {
        control: controlsystem::ElevatorTrim,
        callback: IntegerCallback<'a>,
    },
    EngineRpm {
        control: enginesystem::EngineRpm,
        callback: IntegerCallback<'a>,
    },
    EngineRpmValue {
        control: gaugevalues::EngineRpmValue,
        callback: IntegerCallback<'a>,
    },
    ExtFormationLights {
        control: externalaircraftmodel::ExtFormationLights,
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
    FlapsControlHandle {
        control: controlsystem::FlapsControlHandle,
        callback: IntegerCallback<'a>,
    },
    FuelBooster {
        control: enginecontrolpanel::FuelBooster,
        callback: enginecontrolpanel::FuelBoosterValueCallback<'a>,
    },
    FuelPressure {
        control: fuelsystem::FuelPressure,
        callback: IntegerCallback<'a>,
    },
    FuelPressureValue {
        control: gaugevalues::FuelPressureValue,
        callback: IntegerCallback<'a>,
    },
    FuelSelectorValve {
        control: fuelsystem::FuelSelectorValve,
        callback: fuelsystem::FuelSelectorValveValueCallback<'a>,
    },
    FuelShutOffValve {
        control: fuelsystem::FuelShutOffValve,
        callback: fuelsystem::FuelShutOffValveValueCallback<'a>,
    },
    FuelTankFuselage {
        control: fuelsystem::FuelTankFuselage,
        callback: IntegerCallback<'a>,
    },
    FuelTankLeft {
        control: fuelsystem::FuelTankLeft,
        callback: IntegerCallback<'a>,
    },
    FuelTankRight {
        control: fuelsystem::FuelTankRight,
        callback: IntegerCallback<'a>,
    },
    FuselageFuelTankValue {
        control: gaugevalues::FuselageFuelTankValue,
        callback: IntegerCallback<'a>,
    },
    Gen {
        control: rightswitchpanel::Gen,
        callback: rightswitchpanel::GenValueCallback<'a>,
    },
    GrnRecLt {
        control: rightswitchpanel::GrnRecLt,
        callback: rightswitchpanel::GrnRecLtValueCallback<'a>,
    },
    GunControl {
        control: frontswitchbox::GunControl,
        callback: frontswitchbox::GunControlValueCallback<'a>,
    },
    GunHeat {
        control: rightswitchpanel::GunHeat,
        callback: rightswitchpanel::GunHeatValueCallback<'a>,
    },
    GunsightBrt {
        control: k14gunsight::GunsightBrt,
        callback: IntegerCallback<'a>,
    },
    GunsightFixedGyro {
        control: k14gunsight::GunsightFixedGyro,
        callback: k14gunsight::GunsightFixedGyroValueCallback<'a>,
    },
    GunsightOnOff {
        control: k14gunsight::GunsightOnOff,
        callback: k14gunsight::GunsightOnOffValueCallback<'a>,
    },
    HdgCageButton {
        control: directionalgyro::HdgCageButton,
        callback: directionalgyro::HdgCageButtonValueCallback<'a>,
    },
    HdgCageRotary {
        control: directionalgyro::HdgCageRotary,
        callback: IntegerCallback<'a>,
    },
    HeadingValue {
        control: gaugevalues::HeadingValue,
        callback: IntegerCallback<'a>,
    },
    HighBlowerLamp {
        control: enginecontrolpanel::HighBlowerLamp,
        callback: LedCallback<'a>,
    },
    HighBlowerTestLamp {
        control: enginecontrolpanel::HighBlowerTestLamp,
        callback: enginecontrolpanel::HighBlowerTestLampValueCallback<'a>,
    },
    HomingAdapterCb {
        control: homingadapter::HomingAdapterCb,
        callback: homingadapter::HomingAdapterCbValueCallback<'a>,
    },
    HomingAdapterMode {
        control: homingadapter::HomingAdapterMode,
        callback: homingadapter::HomingAdapterModeValueCallback<'a>,
    },
    HomingAdapterPower {
        control: homingadapter::HomingAdapterPower,
        callback: homingadapter::HomingAdapterPowerValueCallback<'a>,
    },
    HotAir {
        control: environmentsystem::HotAir,
        callback: IntegerCallback<'a>,
    },
    HydRel {
        control: hydraulicsystem::HydRel,
        callback: hydraulicsystem::HydRelValueCallback<'a>,
    },
    HydraulicPressure {
        control: hydraulicsystem::HydraulicPressure,
        callback: IntegerCallback<'a>,
    },
    HydraulicPressureValue {
        control: gaugevalues::HydraulicPressureValue,
        callback: IntegerCallback<'a>,
    },
    IffCode {
        control: iff::IffCode,
        callback: iff::IffCodeValueCallback<'a>,
    },
    IffDetCircuit {
        control: iff::IffDetCircuit,
        callback: iff::IffDetCircuitValueCallback<'a>,
    },
    IffDetLeft {
        control: iff::IffDetLeft,
        callback: iff::IffDetLeftValueCallback<'a>,
    },
    IffDetRight {
        control: iff::IffDetRight,
        callback: iff::IffDetRightValueCallback<'a>,
    },
    IffDistress {
        control: iff::IffDistress,
        callback: iff::IffDistressValueCallback<'a>,
    },
    IffTimeOffOn {
        control: iff::IffTimeOffOn,
        callback: iff::IffTimeOffOnValueCallback<'a>,
    },
    Ignition {
        control: frontswitchbox::Ignition,
        callback: frontswitchbox::IgnitionValueCallback<'a>,
    },
    LandingGearContr0lHandle {
        control: controlsystem::LandingGearContr0lHandle,
        callback: controlsystem::LandingGearContr0lHandleValueCallback<'a>,
    },
    LandingGearGreen {
        control: lightsystem::LandingGearGreen,
        callback: LedCallback<'a>,
    },
    LandingGearRed {
        control: lightsystem::LandingGearRed,
        callback: LedCallback<'a>,
    },
    LandingLight {
        control: lightsystem::LandingLight,
        callback: lightsystem::LandingLightValueCallback<'a>,
    },
    LeftBombArmChem {
        control: frontswitchbox::LeftBombArmChem,
        callback: frontswitchbox::LeftBombArmChemValueCallback<'a>,
    },
    LeftFluorescentLight {
        control: lightsystem::LeftFluorescentLight,
        callback: IntegerCallback<'a>,
    },
    LeftFuelTankValue {
        control: gaugevalues::LeftFuelTankValue,
        callback: IntegerCallback<'a>,
    },
    LftPylodSlvo {
        control: weaponcontrol::LftPylodSlvo,
        callback: weaponcontrol::LftPylodSlvoValueCallback<'a>,
    },
    LockPropellerMixture {
        control: enginesystem::LockPropellerMixture,
        callback: IntegerCallback<'a>,
    },
    LockStickFrwdNeut {
        control: controlsystem::LockStickFrwdNeut,
        callback: controlsystem::LockStickFrwdNeutValueCallback<'a>,
    },
    LockThrottle {
        control: enginesystem::LockThrottle,
        callback: IntegerCallback<'a>,
    },
    ManifoldPressure {
        control: enginesystem::ManifoldPressure,
        callback: IntegerCallback<'a>,
    },
    ManifoldPressureValue {
        control: gaugevalues::ManifoldPressureValue,
        callback: IntegerCallback<'a>,
    },
    MicrophoneOn {
        control: cockpitmechanical::MicrophoneOn,
        callback: cockpitmechanical::MicrophoneOnValueCallback<'a>,
    },
    MixtureControl {
        control: enginesystem::MixtureControl,
        callback: enginesystem::MixtureControlValueCallback<'a>,
    },
    OffPower {
        control: iff::OffPower,
        callback: iff::OffPowerValueCallback<'a>,
    },
    OilControl {
        control: enginesystem::OilControl,
        callback: enginesystem::OilControlValueCallback<'a>,
    },
    OilControlCover {
        control: enginesystem::OilControlCover,
        callback: enginesystem::OilControlCoverValueCallback<'a>,
    },
    OilDilute {
        control: enginecontrolpanel::OilDilute,
        callback: enginecontrolpanel::OilDiluteValueCallback<'a>,
    },
    OilPres {
        control: enginesystem::OilPres,
        callback: IntegerCallback<'a>,
    },
    OilPressureValue {
        control: gaugevalues::OilPressureValue,
        callback: IntegerCallback<'a>,
    },
    OilTemp {
        control: enginesystem::OilTemp,
        callback: IntegerCallback<'a>,
    },
    OilTemperatureValue {
        control: gaugevalues::OilTemperatureValue,
        callback: IntegerCallback<'a>,
    },
    OxygenAutoMix {
        control: oxygensystem::OxygenAutoMix,
        callback: oxygensystem::OxygenAutoMixValueCallback<'a>,
    },
    OxygenByPass {
        control: oxygensystem::OxygenByPass,
        callback: oxygensystem::OxygenByPassValueCallback<'a>,
    },
    OxygenFlow {
        control: oxygensystem::OxygenFlow,
        callback: IntegerCallback<'a>,
    },
    OxygenPres {
        control: oxygensystem::OxygenPres,
        callback: IntegerCallback<'a>,
    },
    OxygenPressureValue {
        control: gaugevalues::OxygenPressureValue,
        callback: IntegerCallback<'a>,
    },
    OxygenWarning {
        control: oxygensystem::OxygenWarning,
        callback: LedCallback<'a>,
    },
    ParkBrakeHandle {
        control: controlsystem::ParkBrakeHandle,
        callback: controlsystem::ParkBrakeHandleValueCallback<'a>,
    },
    PitchAdjust {
        control: artificialhorizon::PitchAdjust,
        callback: IntegerCallback<'a>,
    },
    Pitot {
        control: rightswitchpanel::Pitot,
        callback: rightswitchpanel::PitotValueCallback<'a>,
    },
    Primer {
        control: enginecontrolpanel::Primer,
        callback: enginecontrolpanel::PrimerValueCallback<'a>,
    },
    PropellerRpm {
        control: enginesystem::PropellerRpm,
        callback: IntegerCallback<'a>,
    },
    RadarWarningLight {
        control: tailwarningradar::RadarWarningLight,
        callback: LedCallback<'a>,
    },
    RadioLightsDimmer {
        control: vhfradio::RadioLightsDimmer,
        callback: IntegerCallback<'a>,
    },
    RadioMode2 {
        control: vhfradio::RadioMode2,
        callback: IntegerCallback<'a>,
    },
    RadioMode3 {
        control: vhfradio::RadioMode3,
        callback: IntegerCallback<'a>,
    },
    RadioVolume {
        control: vhfradio::RadioVolume,
        callback: IntegerCallback<'a>,
    },
    RecLtsKey {
        control: rightswitchpanel::RecLtsKey,
        callback: rightswitchpanel::RecLtsKeyValueCallback<'a>,
    },
    RedRecLt {
        control: rightswitchpanel::RedRecLt,
        callback: rightswitchpanel::RedRecLtValueCallback<'a>,
    },
    RemoteCompassCrs {
        control: remotecompass::RemoteCompassCrs,
        callback: IntegerCallback<'a>,
    },
    RemoteCompassHdg {
        control: remotecompass::RemoteCompassHdg,
        callback: IntegerCallback<'a>,
    },
    RemoteCompassSet {
        control: remotecompass::RemoteCompassSet,
        callback: IntegerCallback<'a>,
    },
    RetMaskLvr {
        control: k14gunsight::RetMaskLvr,
        callback: k14gunsight::RetMaskLvrValueCallback<'a>,
    },
    RightBombArmChem {
        control: frontswitchbox::RightBombArmChem,
        callback: frontswitchbox::RightBombArmChemValueCallback<'a>,
    },
    RightFuelTankValue {
        control: gaugevalues::RightFuelTankValue,
        callback: IntegerCallback<'a>,
    },
    RktDelSwitch {
        control: weaponcontrol::RktDelSwitch,
        callback: weaponcontrol::RktDelSwitchValueCallback<'a>,
    },
    RktRelMode {
        control: weaponcontrol::RktRelMode,
        callback: weaponcontrol::RktRelModeValueCallback<'a>,
    },
    RocketCounter {
        control: weaponsystem::RocketCounter,
        callback: IntegerCallback<'a>,
    },
    RocketsBombsModes {
        control: frontswitchbox::RocketsBombsModes,
        callback: frontswitchbox::RocketsBombsModesValueCallback<'a>,
    },
    RtFlresLt {
        control: rightswitchpanel::RtFlresLt,
        callback: IntegerCallback<'a>,
    },
    RtPylodSlvo {
        control: weaponcontrol::RtPylodSlvo,
        callback: weaponcontrol::RtPylodSlvoValueCallback<'a>,
    },
    RudderTrim {
        control: controlsystem::RudderTrim,
        callback: IntegerCallback<'a>,
    },
    SafeLndGearLtTest {
        control: controlsystem::SafeLndGearLtTest,
        callback: controlsystem::SafeLndGearLtTestValueCallback<'a>,
    },
    SetPressure {
        control: altimeter::SetPressure,
        callback: IntegerCallback<'a>,
    },
    SilenceGearHorn {
        control: frontswitchbox::SilenceGearHorn,
        callback: frontswitchbox::SilenceGearHornValueCallback<'a>,
    },
    Slipball {
        control: turnindicator::Slipball,
        callback: IntegerCallback<'a>,
    },
    Starter {
        control: enginecontrolpanel::Starter,
        callback: enginecontrolpanel::StarterValueCallback<'a>,
    },
    StarterCover {
        control: enginecontrolpanel::StarterCover,
        callback: enginecontrolpanel::StarterCoverValueCallback<'a>,
    },
    SuperchargerAutoLowHigh {
        control: enginecontrolpanel::SuperchargerAutoLowHigh,
        callback: enginecontrolpanel::SuperchargerAutoLowHighValueCallback<'a>,
    },
    SuperchargerSwitchCover {
        control: enginecontrolpanel::SuperchargerSwitchCover,
        callback: enginecontrolpanel::SuperchargerSwitchCoverValueCallback<'a>,
    },
    TailLts {
        control: rightswitchpanel::TailLts,
        callback: rightswitchpanel::TailLtsValueCallback<'a>,
    },
    Throttle {
        control: enginesystem::Throttle,
        callback: IntegerCallback<'a>,
    },
    TurnIndicator {
        control: turnindicator::TurnIndicator,
        callback: IntegerCallback<'a>,
    },
    UnsafeLndGearLtTest {
        control: controlsystem::UnsafeLndGearLtTest,
        callback: controlsystem::UnsafeLndGearLtTestValueCallback<'a>,
    },
    VacuumSuction {
        control: enginesystem::VacuumSuction,
        callback: IntegerCallback<'a>,
    },
    VacuumSuctionValue {
        control: gaugevalues::VacuumSuctionValue,
        callback: IntegerCallback<'a>,
    },
    VariometerVvi {
        control: variometer::VariometerVvi,
        callback: IntegerCallback<'a>,
    },
    VhfLockingLever {
        control: vhfradio::VhfLockingLever,
        callback: vhfradio::VhfLockingLeverValueCallback<'a>,
    },
    VhfRadioALight {
        control: vhfradio::VhfRadioALight,
        callback: LedCallback<'a>,
    },
    VhfRadioBLight {
        control: vhfradio::VhfRadioBLight,
        callback: LedCallback<'a>,
    },
    VhfRadioCLight {
        control: vhfradio::VhfRadioCLight,
        callback: LedCallback<'a>,
    },
    VhfRadioChanA {
        control: vhfradio::VhfRadioChanA,
        callback: vhfradio::VhfRadioChanAValueCallback<'a>,
    },
    VhfRadioChanB {
        control: vhfradio::VhfRadioChanB,
        callback: vhfradio::VhfRadioChanBValueCallback<'a>,
    },
    VhfRadioChanC {
        control: vhfradio::VhfRadioChanC,
        callback: vhfradio::VhfRadioChanCValueCallback<'a>,
    },
    VhfRadioChanD {
        control: vhfradio::VhfRadioChanD,
        callback: vhfradio::VhfRadioChanDValueCallback<'a>,
    },
    VhfRadioDLight {
        control: vhfradio::VhfRadioDLight,
        callback: LedCallback<'a>,
    },
    VhfRadioOnOff {
        control: vhfradio::VhfRadioOnOff,
        callback: vhfradio::VhfRadioOnOffValueCallback<'a>,
    },
    VhfRadioTxLight {
        control: vhfradio::VhfRadioTxLight,
        callback: LedCallback<'a>,
    },
    WarningRadarLight {
        control: tailwarningradar::WarningRadarLight,
        callback: IntegerCallback<'a>,
    },
    WarningRadarPower {
        control: tailwarningradar::WarningRadarPower,
        callback: tailwarningradar::WarningRadarPowerValueCallback<'a>,
    },
    WarningRadarTest {
        control: tailwarningradar::WarningRadarTest,
        callback: tailwarningradar::WarningRadarTestValueCallback<'a>,
    },
    WindshieldOil {
        control: environmentsystem::WindshieldOil,
        callback: IntegerCallback<'a>,
    },
    Windshieldcracks {
        control: damage::Windshieldcracks,
        callback: IntegerCallback<'a>,
    },
    Windshieldoil {
        control: damage::Windshieldoil,
        callback: LedCallback<'a>,
    },
    WingLts {
        control: rightswitchpanel::WingLts,
        callback: rightswitchpanel::WingLtsValueCallback<'a>,
    },
    WingSpanSel {
        control: k14gunsight::WingSpanSel,
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
                    if let Ok(value) = <accelerometer::AccelerometerRstValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
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
                    if let Ok(value) =
                        <rightswitchpanel::AmbrRecLtValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
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
                    if let Ok(value) = <cockpitmechanical::ArmRestValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::BarometricPressureValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Bat { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rightswitchpanel::BatValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CageButton { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <artificialhorizon::CageButtonValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CageRotary { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CanopyEmergencyReleaseHandle { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <cockpitmechanical::CanopyEmergencyReleaseHandleValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
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
                    if let Ok(value) =
                        <rightswitchpanel::CircProRstValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
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
                    if let Ok(value) =
                        <enginesystem::CoolantControlValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CoolantControlCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <enginesystem::CoolantControlCoverValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
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
                    if let Ok(value) = <enginecontrolpanel::FuelBoosterValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
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
                    if let Ok(value) = <fuelsystem::FuelSelectorValveValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::FuelShutOffValve { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <fuelsystem::FuelShutOffValveValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
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
                    if let Ok(value) =
                        <rightswitchpanel::GenValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::GrnRecLt { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightswitchpanel::GrnRecLtValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::GunControl { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <frontswitchbox::GunControlValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::GunHeat { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightswitchpanel::GunHeatValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::GunsightBrt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GunsightFixedGyro { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <k14gunsight::GunsightFixedGyroValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::GunsightOnOff { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <k14gunsight::GunsightOnOffValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HdgCageButton { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <directionalgyro::HdgCageButtonValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
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
                    if let Ok(value) =
                        <enginecontrolpanel::HighBlowerTestLampValue as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HomingAdapterCb { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <homingadapter::HomingAdapterCbValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::HomingAdapterMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <homingadapter::HomingAdapterModeValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::HomingAdapterPower { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <homingadapter::HomingAdapterPowerValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::HotAir { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HydRel { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <hydraulicsystem::HydRelValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
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
                    if let Ok(value) = <iff::IffCodeValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::IffDetCircuit { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffDetCircuitValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffDetLeft { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffDetLeftValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffDetRight { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffDetRightValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffDistress { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffDistressValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffTimeOffOn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffTimeOffOnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Ignition { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <frontswitchbox::IgnitionValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LandingGearContr0lHandle { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <controlsystem::LandingGearContr0lHandleValue as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
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
                    if let Ok(value) = <lightsystem::LandingLightValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftBombArmChem { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <frontswitchbox::LeftBombArmChemValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
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
                    if let Ok(value) =
                        <weaponcontrol::LftPylodSlvoValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LockPropellerMixture { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LockStickFrwdNeut { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <controlsystem::LockStickFrwdNeutValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
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
                    if let Ok(value) = <cockpitmechanical::MicrophoneOnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::MixtureControl { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <enginesystem::MixtureControlValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::OffPower { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <iff::OffPowerValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::OilControl { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <enginesystem::OilControlValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::OilControlCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <enginesystem::OilControlCoverValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::OilDilute { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <enginecontrolpanel::OilDiluteValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
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
                    if let Ok(value) =
                        <oxygensystem::OxygenAutoMixValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::OxygenByPass { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <oxygensystem::OxygenByPassValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
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
                    if let Ok(value) = <controlsystem::ParkBrakeHandleValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::PitchAdjust { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Pitot { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightswitchpanel::PitotValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Primer { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <enginecontrolpanel::PrimerValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
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
                    if let Ok(value) =
                        <rightswitchpanel::RecLtsKeyValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RedRecLt { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightswitchpanel::RedRecLtValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
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
                    if let Ok(value) = <k14gunsight::RetMaskLvrValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightBombArmChem { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <frontswitchbox::RightBombArmChemValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::RightFuelTankValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RktDelSwitch { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <weaponcontrol::RktDelSwitchValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RktRelMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weaponcontrol::RktRelModeValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RocketCounter { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RocketsBombsModes { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <frontswitchbox::RocketsBombsModesValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::RtFlresLt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RtPylodSlvo { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weaponcontrol::RtPylodSlvoValue as core::convert::TryFrom<
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
            Self::SafeLndGearLtTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <controlsystem::SafeLndGearLtTestValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::SetPressure { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SilenceGearHorn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <frontswitchbox::SilenceGearHornValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::Slipball { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Starter { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <enginecontrolpanel::StarterValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::StarterCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <enginecontrolpanel::StarterCoverValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::SuperchargerAutoLowHigh { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <enginecontrolpanel::SuperchargerAutoLowHighValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::SuperchargerSwitchCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <enginecontrolpanel::SuperchargerSwitchCoverValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::TailLts { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightswitchpanel::TailLtsValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
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
            Self::TurnIndicator { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UnsafeLndGearLtTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <controlsystem::UnsafeLndGearLtTestValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
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
                    if let Ok(value) = <vhfradio::VhfLockingLeverValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
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
                    if let Ok(value) = <vhfradio::VhfRadioChanAValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VhfRadioChanB { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <vhfradio::VhfRadioChanBValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VhfRadioChanC { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <vhfradio::VhfRadioChanCValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VhfRadioChanD { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <vhfradio::VhfRadioChanDValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VhfRadioDLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::VhfRadioOnOff { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <vhfradio::VhfRadioOnOffValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
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
                    if let Ok(value) = <tailwarningradar::WarningRadarPowerValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::WarningRadarTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <tailwarningradar::WarningRadarTestValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
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
                    if let Ok(value) = <rightswitchpanel::WingLtsValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
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

    pub fn register_accelerometer_main<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AccelerometerMain {
                control: accelerometer::AccelerometerMain::new(),
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
                control: accelerometer::AccelerometerMax::new(),
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
                control: accelerometer::AccelerometerMin::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_accelerometer_rst<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(accelerometer::AccelerometerRstValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AccelerometerRst {
                control: accelerometer::AccelerometerRst::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_accelerometer_value<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AccelerometerValue {
                control: gaugevalues::AccelerometerValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ahorizon_bank<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AhorizonBank {
                control: artificialhorizon::AhorizonBank::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ahorizon_caged<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AhorizonCaged {
                control: artificialhorizon::AhorizonCaged::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ahorizon_pitch<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AhorizonPitch {
                control: artificialhorizon::AhorizonPitch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ahorizon_pitch_shift<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AhorizonPitchShift {
                control: artificialhorizon::AhorizonPitchShift::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aileron_trim<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AileronTrim {
                control: controlsystem::AileronTrim::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_airspeed_mph_value<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AirspeedMphValue {
                control: gaugevalues::AirspeedMphValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_airspeed_needle<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AirspeedNeedle {
                control: airspeedindicator::AirspeedNeedle::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter10000_foot<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Altimeter10000Foot {
                control: altimeter::Altimeter10000Foot::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter1000_foot<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Altimeter1000Foot {
                control: altimeter::Altimeter1000Foot::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter100_foot<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Altimeter100Foot {
                control: altimeter::Altimeter100Foot::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter_pressure<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltimeterPressure {
                control: altimeter::AltimeterPressure::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter_value<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltimeterValue {
                control: gaugevalues::AltimeterValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ambr_rec_lt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightswitchpanel::AmbrRecLtValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmbrRecLt {
                control: rightswitchpanel::AmbrRecLt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ammeter<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ammeter {
                control: electricsystem::Ammeter::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ammeter_value<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmmeterValue {
                control: gaugevalues::AmmeterValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_arm_rest<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cockpitmechanical::ArmRestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ArmRest {
                control: cockpitmechanical::ArmRest::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_barometric_pressure_value<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BarometricPressureValue {
                control: gaugevalues::BarometricPressureValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bat<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightswitchpanel::BatValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Bat {
                control: rightswitchpanel::Bat::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cage_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(artificialhorizon::CageButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CageButton {
                control: artificialhorizon::CageButton::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cage_rotary<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CageRotary {
                control: artificialhorizon::CageRotary::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_emergency_release_handle<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cockpitmechanical::CanopyEmergencyReleaseHandleValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyEmergencyReleaseHandle {
                control: cockpitmechanical::CanopyEmergencyReleaseHandle::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_hand_crank<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyHandCrank {
                control: cockpitmechanical::CanopyHandCrank::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_carb_cold_air<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CarbColdAir {
                control: enginesystem::CarbColdAir::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_carb_temp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CarbTemp {
                control: enginesystem::CarbTemp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_carb_warm_air<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CarbWarmAir {
                control: enginesystem::CarbWarmAir::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_circ_pro_rst<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightswitchpanel::CircProRstValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CircProRst {
                control: rightswitchpanel::CircProRst::new(),
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

    pub fn register_clock_adjust_button<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockAdjustButton {
                control: clock::ClockAdjustButton::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_hours<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockHours {
                control: clock::ClockHours::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_minutes<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockMinutes {
                control: clock::ClockMinutes::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_seconds<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockSeconds {
                control: clock::ClockSeconds::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cockpit_lights<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CockpitLights {
                control: frontswitchbox::CockpitLights::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cold_air<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ColdAir {
                control: environmentsystem::ColdAir::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_coolant_control<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginesystem::CoolantControlValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CoolantControl {
                control: enginesystem::CoolantControl::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_coolant_control_cover<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginesystem::CoolantControlCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CoolantControlCover {
                control: enginesystem::CoolantControlCover::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_coolant_temp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CoolantTemp {
                control: enginesystem::CoolantTemp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_course_value<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CourseValue {
                control: gaugevalues::CourseValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_defroster<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Defroster {
                control: environmentsystem::Defroster::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_detrola_frequency<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DetrolaFrequency {
                control: detrola::DetrolaFrequency::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_detrola_volume<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DetrolaVolume {
                control: detrola::DetrolaVolume::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_directional_gyro<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DirectionalGyro {
                control: directionalgyro::DirectionalGyro::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_directional_gyro_value<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DirectionalGyroValue {
                control: gaugevalues::DirectionalGyroValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_elevator_trim<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ElevatorTrim {
                control: controlsystem::ElevatorTrim::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_rpm<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineRpm {
                control: enginesystem::EngineRpm::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_rpm_value<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineRpmValue {
                control: gaugevalues::EngineRpmValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_formation_lights<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtFormationLights {
                control: externalaircraftmodel::ExtFormationLights::new(),
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

    pub fn register_flaps_control_handle<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlapsControlHandle {
                control: controlsystem::FlapsControlHandle::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_booster<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginecontrolpanel::FuelBoosterValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelBooster {
                control: enginecontrolpanel::FuelBooster::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_pressure<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelPressure {
                control: fuelsystem::FuelPressure::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_pressure_value<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelPressureValue {
                control: gaugevalues::FuelPressureValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_selector_valve<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuelsystem::FuelSelectorValveValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelSelectorValve {
                control: fuelsystem::FuelSelectorValve::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_shut_off_valve<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuelsystem::FuelShutOffValveValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelShutOffValve {
                control: fuelsystem::FuelShutOffValve::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_tank_fuselage<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelTankFuselage {
                control: fuelsystem::FuelTankFuselage::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_tank_left<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelTankLeft {
                control: fuelsystem::FuelTankLeft::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_tank_right<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelTankRight {
                control: fuelsystem::FuelTankRight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuselage_fuel_tank_value<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuselageFuelTankValue {
                control: gaugevalues::FuselageFuelTankValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gen<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightswitchpanel::GenValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Gen {
                control: rightswitchpanel::Gen::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_grn_rec_lt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightswitchpanel::GrnRecLtValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GrnRecLt {
                control: rightswitchpanel::GrnRecLt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gun_control<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(frontswitchbox::GunControlValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GunControl {
                control: frontswitchbox::GunControl::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gun_heat<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightswitchpanel::GunHeatValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GunHeat {
                control: rightswitchpanel::GunHeat::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gunsight_brt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GunsightBrt {
                control: k14gunsight::GunsightBrt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gunsight_fixed_gyro<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(k14gunsight::GunsightFixedGyroValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GunsightFixedGyro {
                control: k14gunsight::GunsightFixedGyro::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gunsight_on_off<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(k14gunsight::GunsightOnOffValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GunsightOnOff {
                control: k14gunsight::GunsightOnOff::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hdg_cage_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(directionalgyro::HdgCageButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HdgCageButton {
                control: directionalgyro::HdgCageButton::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hdg_cage_rotary<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HdgCageRotary {
                control: directionalgyro::HdgCageRotary::new(),
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

    pub fn register_high_blower_lamp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::HighBlowerLamp {
                control: enginecontrolpanel::HighBlowerLamp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_high_blower_test_lamp<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginecontrolpanel::HighBlowerTestLampValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HighBlowerTestLamp {
                control: enginecontrolpanel::HighBlowerTestLamp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_homing_adapter_cb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(homingadapter::HomingAdapterCbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HomingAdapterCb {
                control: homingadapter::HomingAdapterCb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_homing_adapter_mode<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(homingadapter::HomingAdapterModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HomingAdapterMode {
                control: homingadapter::HomingAdapterMode::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_homing_adapter_power<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(homingadapter::HomingAdapterPowerValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HomingAdapterPower {
                control: homingadapter::HomingAdapterPower::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hot_air<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HotAir {
                control: environmentsystem::HotAir::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hyd_rel<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(hydraulicsystem::HydRelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydRel {
                control: hydraulicsystem::HydRel::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hydraulic_pressure<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydraulicPressure {
                control: hydraulicsystem::HydraulicPressure::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hydraulic_pressure_value<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydraulicPressureValue {
                control: gaugevalues::HydraulicPressureValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_code<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffCodeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffCode {
                control: iff::IffCode::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_det_circuit<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffDetCircuitValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffDetCircuit {
                control: iff::IffDetCircuit::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_det_left<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffDetLeftValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffDetLeft {
                control: iff::IffDetLeft::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_det_right<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffDetRightValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffDetRight {
                control: iff::IffDetRight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_distress<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffDistressValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffDistress {
                control: iff::IffDistress::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_time_off_on<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffTimeOffOnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffTimeOffOn {
                control: iff::IffTimeOffOn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ignition<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(frontswitchbox::IgnitionValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ignition {
                control: frontswitchbox::Ignition::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_landing_gear_contr0l_handle<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlsystem::LandingGearContr0lHandleValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LandingGearContr0lHandle {
                control: controlsystem::LandingGearContr0lHandle::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_landing_gear_green<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LandingGearGreen {
                control: lightsystem::LandingGearGreen::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_landing_gear_red<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LandingGearRed {
                control: lightsystem::LandingGearRed::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_landing_light<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(lightsystem::LandingLightValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LandingLight {
                control: lightsystem::LandingLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_bomb_arm_chem<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(frontswitchbox::LeftBombArmChemValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftBombArmChem {
                control: frontswitchbox::LeftBombArmChem::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_fluorescent_light<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftFluorescentLight {
                control: lightsystem::LeftFluorescentLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_fuel_tank_value<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftFuelTankValue {
                control: gaugevalues::LeftFuelTankValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lft_pylod_slvo<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponcontrol::LftPylodSlvoValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LftPylodSlvo {
                control: weaponcontrol::LftPylodSlvo::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lock_propeller_mixture<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LockPropellerMixture {
                control: enginesystem::LockPropellerMixture::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lock_stick_frwd_neut<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlsystem::LockStickFrwdNeutValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LockStickFrwdNeut {
                control: controlsystem::LockStickFrwdNeut::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lock_throttle<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LockThrottle {
                control: enginesystem::LockThrottle::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_manifold_pressure<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ManifoldPressure {
                control: enginesystem::ManifoldPressure::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_manifold_pressure_value<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ManifoldPressureValue {
                control: gaugevalues::ManifoldPressureValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_microphone_on<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cockpitmechanical::MicrophoneOnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MicrophoneOn {
                control: cockpitmechanical::MicrophoneOn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mixture_control<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginesystem::MixtureControlValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MixtureControl {
                control: enginesystem::MixtureControl::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_off_power<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::OffPowerValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::OffPower {
                control: iff::OffPower::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oil_control<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginesystem::OilControlValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::OilControl {
                control: enginesystem::OilControl::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oil_control_cover<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginesystem::OilControlCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::OilControlCover {
                control: enginesystem::OilControlCover::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oil_dilute<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginecontrolpanel::OilDiluteValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::OilDilute {
                control: enginecontrolpanel::OilDilute::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oil_pres<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OilPres {
                control: enginesystem::OilPres::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oil_pressure_value<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OilPressureValue {
                control: gaugevalues::OilPressureValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oil_temp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OilTemp {
                control: enginesystem::OilTemp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oil_temperature_value<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OilTemperatureValue {
                control: gaugevalues::OilTemperatureValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxygen_auto_mix<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(oxygensystem::OxygenAutoMixValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxygenAutoMix {
                control: oxygensystem::OxygenAutoMix::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxygen_by_pass<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(oxygensystem::OxygenByPassValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxygenByPass {
                control: oxygensystem::OxygenByPass::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxygen_flow<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxygenFlow {
                control: oxygensystem::OxygenFlow::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxygen_pres<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxygenPres {
                control: oxygensystem::OxygenPres::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxygen_pressure_value<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxygenPressureValue {
                control: gaugevalues::OxygenPressureValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxygen_warning<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxygenWarning {
                control: oxygensystem::OxygenWarning::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_park_brake_handle<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlsystem::ParkBrakeHandleValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ParkBrakeHandle {
                control: controlsystem::ParkBrakeHandle::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pitch_adjust<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PitchAdjust {
                control: artificialhorizon::PitchAdjust::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pitot<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightswitchpanel::PitotValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Pitot {
                control: rightswitchpanel::Pitot::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_primer<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginecontrolpanel::PrimerValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Primer {
                control: enginecontrolpanel::Primer::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_propeller_rpm<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PropellerRpm {
                control: enginesystem::PropellerRpm::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_warning_light<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarWarningLight {
                control: tailwarningradar::RadarWarningLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radio_lights_dimmer<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadioLightsDimmer {
                control: vhfradio::RadioLightsDimmer::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radio_mode2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadioMode2 {
                control: vhfradio::RadioMode2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radio_mode3<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadioMode3 {
                control: vhfradio::RadioMode3::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radio_volume<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadioVolume {
                control: vhfradio::RadioVolume::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rec_lts_key<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightswitchpanel::RecLtsKeyValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RecLtsKey {
                control: rightswitchpanel::RecLtsKey::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_red_rec_lt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightswitchpanel::RedRecLtValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RedRecLt {
                control: rightswitchpanel::RedRecLt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_remote_compass_crs<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RemoteCompassCrs {
                control: remotecompass::RemoteCompassCrs::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_remote_compass_hdg<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RemoteCompassHdg {
                control: remotecompass::RemoteCompassHdg::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_remote_compass_set<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RemoteCompassSet {
                control: remotecompass::RemoteCompassSet::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ret_mask_lvr<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(k14gunsight::RetMaskLvrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RetMaskLvr {
                control: k14gunsight::RetMaskLvr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_bomb_arm_chem<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(frontswitchbox::RightBombArmChemValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightBombArmChem {
                control: frontswitchbox::RightBombArmChem::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_fuel_tank_value<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightFuelTankValue {
                control: gaugevalues::RightFuelTankValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rkt_del_switch<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponcontrol::RktDelSwitchValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RktDelSwitch {
                control: weaponcontrol::RktDelSwitch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rkt_rel_mode<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponcontrol::RktRelModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RktRelMode {
                control: weaponcontrol::RktRelMode::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rocket_counter<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RocketCounter {
                control: weaponsystem::RocketCounter::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rockets_bombs_modes<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(frontswitchbox::RocketsBombsModesValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RocketsBombsModes {
                control: frontswitchbox::RocketsBombsModes::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rt_flres_lt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RtFlresLt {
                control: rightswitchpanel::RtFlresLt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rt_pylod_slvo<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponcontrol::RtPylodSlvoValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RtPylodSlvo {
                control: weaponcontrol::RtPylodSlvo::new(),
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
                control: controlsystem::RudderTrim::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_safe_lnd_gear_lt_test<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlsystem::SafeLndGearLtTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SafeLndGearLtTest {
                control: controlsystem::SafeLndGearLtTest::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_set_pressure<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SetPressure {
                control: altimeter::SetPressure::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_silence_gear_horn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(frontswitchbox::SilenceGearHornValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SilenceGearHorn {
                control: frontswitchbox::SilenceGearHorn::new(),
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
                control: turnindicator::Slipball::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_starter<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginecontrolpanel::StarterValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Starter {
                control: enginecontrolpanel::Starter::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_starter_cover<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginecontrolpanel::StarterCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::StarterCover {
                control: enginecontrolpanel::StarterCover::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_supercharger_auto_low_high<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginecontrolpanel::SuperchargerAutoLowHighValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SuperchargerAutoLowHigh {
                control: enginecontrolpanel::SuperchargerAutoLowHigh::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_supercharger_switch_cover<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginecontrolpanel::SuperchargerSwitchCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SuperchargerSwitchCover {
                control: enginecontrolpanel::SuperchargerSwitchCover::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tail_lts<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightswitchpanel::TailLtsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TailLts {
                control: rightswitchpanel::TailLts::new(),
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
                control: enginesystem::Throttle::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_turn_indicator<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TurnIndicator {
                control: turnindicator::TurnIndicator::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_unsafe_lnd_gear_lt_test<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlsystem::UnsafeLndGearLtTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UnsafeLndGearLtTest {
                control: controlsystem::UnsafeLndGearLtTest::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vacuum_suction<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VacuumSuction {
                control: enginesystem::VacuumSuction::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vacuum_suction_value<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VacuumSuctionValue {
                control: gaugevalues::VacuumSuctionValue::new(),
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
                control: variometer::VariometerVvi::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_locking_lever<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(vhfradio::VhfLockingLeverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfLockingLever {
                control: vhfradio::VhfLockingLever::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_radio_a_light<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfRadioALight {
                control: vhfradio::VhfRadioALight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_radio_b_light<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfRadioBLight {
                control: vhfradio::VhfRadioBLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_radio_c_light<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfRadioCLight {
                control: vhfradio::VhfRadioCLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_radio_chan_a<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(vhfradio::VhfRadioChanAValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfRadioChanA {
                control: vhfradio::VhfRadioChanA::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_radio_chan_b<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(vhfradio::VhfRadioChanBValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfRadioChanB {
                control: vhfradio::VhfRadioChanB::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_radio_chan_c<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(vhfradio::VhfRadioChanCValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfRadioChanC {
                control: vhfradio::VhfRadioChanC::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_radio_chan_d<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(vhfradio::VhfRadioChanDValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfRadioChanD {
                control: vhfradio::VhfRadioChanD::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_radio_d_light<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfRadioDLight {
                control: vhfradio::VhfRadioDLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_radio_on_off<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(vhfradio::VhfRadioOnOffValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfRadioOnOff {
                control: vhfradio::VhfRadioOnOff::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_radio_tx_light<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfRadioTxLight {
                control: vhfradio::VhfRadioTxLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_warning_radar_light<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WarningRadarLight {
                control: tailwarningradar::WarningRadarLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_warning_radar_power<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(tailwarningradar::WarningRadarPowerValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::WarningRadarPower {
                control: tailwarningradar::WarningRadarPower::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_warning_radar_test<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(tailwarningradar::WarningRadarTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::WarningRadarTest {
                control: tailwarningradar::WarningRadarTest::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_windshield_oil<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WindshieldOil {
                control: environmentsystem::WindshieldOil::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_windshieldcracks<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Windshieldcracks {
                control: damage::Windshieldcracks::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_windshieldoil<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Windshieldoil {
                control: damage::Windshieldoil::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wing_lts<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightswitchpanel::WingLtsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::WingLts {
                control: rightswitchpanel::WingLts::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wing_span_sel<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WingSpanSel {
                control: k14gunsight::WingSpanSel::new(),
                callback: alloc::boxed::Box::new(callback),
            })
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
