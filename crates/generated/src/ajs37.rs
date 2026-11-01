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

pub mod countermeasures {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CountermeasureChaffFlaresSelectorValue {
        Select1,
        Select2,
        Select3,
    }
    pub type CountermeasureChaffFlaresSelectorValueCallback<'a> =
        &'a mut dyn FnMut(CountermeasureChaffFlaresSelectorValue);
    impl core::convert::TryFrom<u16> for CountermeasureChaffFlaresSelectorValue {
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

    /// Countermeasure Chaff/Flares Selector
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct CountermeasureChaffFlaresSelector(Selector<17944, 192, 6>);
    impl CountermeasureChaffFlaresSelector {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CountermeasureModeSelectorValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type CountermeasureModeSelectorValueCallback<'a> =
        &'a mut dyn FnMut(CountermeasureModeSelectorValue);
    impl core::convert::TryFrom<u16> for CountermeasureModeSelectorValue {
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

    /// Countermeasure Operation Mode Selector
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct CountermeasureModeSelector(Selector<17944, 56, 3>);
    impl CountermeasureModeSelector {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CountermeasureReleaseModeValue {
        Select1,
        Select2,
        Select3,
    }
    pub type CountermeasureReleaseModeValueCallback<'a> =
        &'a mut dyn FnMut(CountermeasureReleaseModeValue);
    impl core::convert::TryFrom<u16> for CountermeasureReleaseModeValue {
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

    /// Countermeasure Release Mode
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct CountermeasureReleaseMode(Selector<17944, 768, 8>);
    impl CountermeasureReleaseMode {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CountermeasureStreakModeSelectorValue {
        Select1,
        Select2,
    }
    pub type CountermeasureStreakModeSelectorValueCallback<'a> =
        &'a mut dyn FnMut(CountermeasureStreakModeSelectorValue);
    impl core::convert::TryFrom<u16> for CountermeasureStreakModeSelectorValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Countermeasure Streak Mode Selector
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CountermeasureStreakModeSelector(Selector<17942, 16384, 14>);
    impl CountermeasureStreakModeSelector {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum JammerBandSelectorValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type JammerBandSelectorValueCallback<'a> = &'a mut dyn FnMut(JammerBandSelectorValue);
    impl core::convert::TryFrom<u16> for JammerBandSelectorValue {
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

    /// Jammer Band Selector
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct JammerBandSelector(Selector<17944, 7, 0>);
    impl JammerBandSelector {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum JammerModeSelectorValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type JammerModeSelectorValueCallback<'a> = &'a mut dyn FnMut(JammerModeSelectorValue);
    impl core::convert::TryFrom<u16> for JammerModeSelectorValue {
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

    /// Jammer Mode Selector
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct JammerModeSelector(Selector<17942, 14336, 11>);
    impl JammerModeSelector {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod doppler {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Doppler Land/Sea Mode
    /// Input: toggle switch state
    /// Output: selector position
    pub struct DopplerLandSeaMode(Selector<17936, 2, 1>);
    impl DopplerLandSeaMode {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod electricsystem {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BackupGeneratorValue {
        Select1,
        Select2,
    }
    pub type BackupGeneratorValueCallback<'a> = &'a mut dyn FnMut(BackupGeneratorValue);
    impl core::convert::TryFrom<u16> for BackupGeneratorValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Backup Generator
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct BackupGenerator(Selector<17934, 32768, 15>);
    impl BackupGenerator {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GeneratorValue {
        Select1,
        Select2,
    }
    pub type GeneratorValueCallback<'a> = &'a mut dyn FnMut(GeneratorValue);
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
    pub struct Generator(Selector<17934, 8192, 13>);
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
    pub enum MainElectricPowerValue {
        Select1,
        Select2,
    }
    pub type MainElectricPowerValueCallback<'a> = &'a mut dyn FnMut(MainElectricPowerValue);
    impl core::convert::TryFrom<u16> for MainElectricPowerValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Main Electric Power
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MainElectricPower(Selector<17934, 16384, 14>);
    impl MainElectricPower {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod enginepanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AfkLeverValue {
        Select1,
        Select2,
    }
    pub type AfkLeverValueCallback<'a> = &'a mut dyn FnMut(AfkLeverValue);
    impl core::convert::TryFrom<u16> for AfkLeverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// AFK Lever
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AfkLever(Selector<17934, 1024, 10>);
    impl AfkLever {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CabinAirValveValue {
        Select1,
        Select2,
    }
    pub type CabinAirValveValueCallback<'a> = &'a mut dyn FnMut(CabinAirValveValue);
    impl core::convert::TryFrom<u16> for CabinAirValveValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Cabin Air Valve
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CabinAirValve(Selector<18076, 1, 0>);
    impl CabinAirValve {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbAutopilotValue {
        Select1,
        Select2,
    }
    pub type CbAutopilotValueCallback<'a> = &'a mut dyn FnMut(CbAutopilotValue);
    impl core::convert::TryFrom<u16> for CbAutopilotValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CB Autopilot SA
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbAutopilot(Selector<17926, 4096, 12>);
    impl CbAutopilot {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbCiSiValue {
        Select1,
        Select2,
    }
    pub type CbCiSiValueCallback<'a> = &'a mut dyn FnMut(CbCiSiValue);
    impl core::convert::TryFrom<u16> for CbCiSiValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CB CI/SI
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbCiSi(Selector<17926, 32768, 15>);
    impl CbCiSi {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbEjectionSystemValue {
        Select1,
        Select2,
    }
    pub type CbEjectionSystemValueCallback<'a> = &'a mut dyn FnMut(CbEjectionSystemValue);
    impl core::convert::TryFrom<u16> for CbEjectionSystemValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CB Ejection System
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbEjectionSystem(Selector<17934, 1, 0>);
    impl CbEjectionSystem {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbEngineValue {
        Select1,
        Select2,
    }
    pub type CbEngineValueCallback<'a> = &'a mut dyn FnMut(CbEngineValue);
    impl core::convert::TryFrom<u16> for CbEngineValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CB Engine
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbEngine(Selector<17934, 2, 1>);
    impl CbEngine {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbHighAlphaWarnValue {
        Select1,
        Select2,
    }
    pub type CbHighAlphaWarnValueCallback<'a> = &'a mut dyn FnMut(CbHighAlphaWarnValue);
    impl core::convert::TryFrom<u16> for CbHighAlphaWarnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CB High Alpha Warning
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbHighAlphaWarn(Selector<17926, 8192, 13>);
    impl CbHighAlphaWarn {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbTrimSystemValue {
        Select1,
        Select2,
    }
    pub type CbTrimSystemValueCallback<'a> = &'a mut dyn FnMut(CbTrimSystemValue);
    impl core::convert::TryFrom<u16> for CbTrimSystemValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CB Trim System
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbTrimSystem(Selector<17926, 16384, 14>);
    impl CbTrimSystem {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DataCartridgeValue {
        Select1,
        Select2,
    }
    pub type DataCartridgeValueCallback<'a> = &'a mut dyn FnMut(DataCartridgeValue);
    impl core::convert::TryFrom<u16> for DataCartridgeValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Insert/Remove Data Cartridge
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct DataCartridge(Selector<17934, 2048, 11>);
    impl DataCartridge {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// Windscreen De-Ice
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct DeIce(LimitedDial<18070, 65535, 0>);
    impl DeIce {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DmeSelectorValue {
        Select1,
        Select2,
    }
    pub type DmeSelectorValueCallback<'a> = &'a mut dyn FnMut(DmeSelectorValue);
    impl core::convert::TryFrom<u16> for DmeSelectorValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// DME Selector
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct DmeSelector(Selector<17934, 16, 4>);
    impl DmeSelector {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// Drysuit Ventilation Adjustment
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct Drysuit(LimitedDial<18074, 65535, 0>);
    impl Drysuit {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EngineDeiceValue {
        Select1,
        Select2,
    }
    pub type EngineDeiceValueCallback<'a> = &'a mut dyn FnMut(EngineDeiceValue);
    impl core::convert::TryFrom<u16> for EngineDeiceValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Engine De-Ice
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EngineDeice(Selector<17926, 1024, 10>);
    impl EngineDeice {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FlightRecorderValue {
        Select1,
        Select2,
        Select3,
    }
    pub type FlightRecorderValueCallback<'a> = &'a mut dyn FnMut(FlightRecorderValue);
    impl core::convert::TryFrom<u16> for FlightRecorderValue {
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

    /// Flight Recorder
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct FlightRecorder(Selector<17934, 384, 7>);
    impl FlightRecorder {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HighPresFuelValveValue {
        Select1,
        Select2,
    }
    pub type HighPresFuelValveValueCallback<'a> = &'a mut dyn FnMut(HighPresFuelValveValue);
    impl core::convert::TryFrom<u16> for HighPresFuelValveValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// High Pressure Fuel Valve
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct HighPresFuelValve(Selector<17926, 512, 9>);
    impl HighPresFuelValve {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffChannelValue {
        Select1,
        Select2,
    }
    pub type IffChannelValueCallback<'a> = &'a mut dyn FnMut(IffChannelValue);
    impl core::convert::TryFrom<u16> for IffChannelValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// IFF Channel Selector
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IffChannel(Selector<17934, 8, 3>);
    impl IffChannel {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffTransponderPowerValue {
        Select1,
        Select2,
    }
    pub type IffTransponderPowerValueCallback<'a> = &'a mut dyn FnMut(IffTransponderPowerValue);
    impl core::convert::TryFrom<u16> for IffTransponderPowerValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// IFF/Transponder Power
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IffTransponderPower(Selector<17934, 4, 2>);
    impl IffTransponderPower {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IgnitionSystemValue {
        Select1,
        Select2,
    }
    pub type IgnitionSystemValueCallback<'a> = &'a mut dyn FnMut(IgnitionSystemValue);
    impl core::convert::TryFrom<u16> for IgnitionSystemValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Ignition System
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IgnitionSystem(Selector<17934, 32, 5>);
    impl IgnitionSystem {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LowPresFuelValveValue {
        Select1,
        Select2,
    }
    pub type LowPresFuelValveValueCallback<'a> = &'a mut dyn FnMut(LowPresFuelValveValue);
    impl core::convert::TryFrom<u16> for LowPresFuelValveValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Low Pressure Fuel Valve
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LowPresFuelValve(Selector<17926, 256, 8>);
    impl LowPresFuelValve {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ManAfterburnFuelRegValue {
        Select1,
        Select2,
    }
    pub type ManAfterburnFuelRegValueCallback<'a> = &'a mut dyn FnMut(ManAfterburnFuelRegValue);
    impl core::convert::TryFrom<u16> for ManAfterburnFuelRegValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Manual Afterburner Fuel Regulator
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ManAfterburnFuelReg(Selector<17934, 64, 6>);
    impl ManAfterburnFuelReg {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ManualFuelRegValue {
        Select1,
        Select2,
    }
    pub type ManualFuelRegValueCallback<'a> = &'a mut dyn FnMut(ManualFuelRegValue);
    impl core::convert::TryFrom<u16> for ManualFuelRegValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Manual Fuel Regulator
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ManualFuelReg(Selector<17926, 2048, 11>);
    impl ManualFuelReg {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MissileSelectButtonValue {
        Select1,
        Select2,
    }
    pub type MissileSelectButtonValueCallback<'a> = &'a mut dyn FnMut(MissileSelectButtonValue);
    impl core::convert::TryFrom<u16> for MissileSelectButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Missile Select Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MissileSelectButton(Selector<17934, 4096, 12>);
    impl MissileSelectButton {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RestartValue {
        Select1,
        Select2,
    }
    pub type RestartValueCallback<'a> = &'a mut dyn FnMut(RestartValue);
    impl core::convert::TryFrom<u16> for RestartValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Restart
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Restart(Selector<17934, 512, 9>);
    impl Restart {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum StartSystemValue {
        Select1,
        Select2,
    }
    pub type StartSystemValueCallback<'a> = &'a mut dyn FnMut(StartSystemValue);
    impl core::convert::TryFrom<u16> for StartSystemValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Start System
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct StartSystem(Selector<17926, 128, 7>);
    impl StartSystem {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// Maintenance Testing Mode
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct TestMode(LimitedDial<18072, 65535, 0>);
    impl TestMode {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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
}

pub mod errorpanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Autothrottle
    /// Output: 0 if light is off, 1 if light is on
    pub struct AfkFel(Led<18028, 4096, 12>);
    impl AfkFel {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Low Fuel
    /// Output: 0 if light is off, 1 if light is on
    pub struct Bra24(Led<18030, 1024, 10>);
    impl Bra24 {
        pub const fn new() -> Self {
            Self(Led::new())
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
    pub struct BraUppf(Led<17944, 32768, 15>);
    impl BraUppf {
        pub const fn new() -> Self {
            Self(Led::new())
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
    pub struct Brand1(Led<17944, 8192, 13>);
    impl Brand1 {
        pub const fn new() -> Self {
            Self(Led::new())
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
    pub struct Brand2(Led<17944, 16384, 14>);
    impl Brand2 {
        pub const fn new() -> Self {
            Self(Led::new())
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
    pub struct BrandGts(Led<18030, 2048, 11>);
    impl BrandGts {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Computer
    /// Output: 0 if light is off, 1 if light is on
    pub struct Ck(Led<18030, 8, 3>);
    impl Ck {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Thrust Reverser Inoperable
    /// Output: 0 if light is off, 1 if light is on
    pub struct EjRev(Led<18028, 8192, 13>);
    impl EjRev {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Electrical System
    /// Output: 0 if light is off, 1 if light is on
    pub struct Elfel(Led<18028, 256, 8>);
    impl Elfel {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Flares Empty
    /// Output: 0 if light is off, 1 if light is on
    pub struct FacklSl(Led<18032, 1, 0>);
    impl FacklSl {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Thrust Reverser Warning
    /// Output: 0 if light is off, 1 if light is on
    pub struct ForvForb(Led<18028, 8, 3>);
    impl ForvForb {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Autopilot Hold
    /// Output: 0 if light is off, 1 if light is on
    pub struct HallFunk(Led<18030, 1, 0>);
    impl HallFunk {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Right Gear
    /// Output: 0 if light is off, 1 if light is on
    pub struct Hstall(Led<18028, 64, 6>);
    impl Hstall {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Ejection Seat/Canopy
    /// Output: 0 if light is off, 1 if light is on
    pub struct HuvOStol(Led<18030, 32, 5>);
    impl HuvOStol {
        pub const fn new() -> Self {
            Self(Led::new())
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
    pub struct HydraTa1(Led<18028, 2048, 11>);
    impl HydraTa1 {
        pub const fn new() -> Self {
            Self(Led::new())
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
    pub struct HydraTa2(Led<18028, 1024, 10>);
    impl HydraTa2 {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Cabin Pressure
    /// Output: 0 if light is off, 1 if light is on
    pub struct Kabinhojd(Led<18030, 16, 4>);
    impl Kabinhojd {
        pub const fn new() -> Self {
            Self(Led::new())
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
    pub struct KbHKaSl(Led<18030, 32768, 15>);
    impl KbHKaSl {
        pub const fn new() -> Self {
            Self(Led::new())
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
    pub struct KbVSlut(Led<18030, 16384, 14>);
    impl KbVSlut {
        pub const fn new() -> Self {
            Self(Led::new())
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
    pub struct Landstall(Led<18028, 4, 2>);
    impl Landstall {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Airbrakes
    /// Output: 0 if light is off, 1 if light is on
    pub struct Luftbroms(Led<18032, 4, 2>);
    impl Luftbroms {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Manual Fuel Regulator
    /// Output: 0 if light is off, 1 if light is on
    pub struct ManBgReg(Led<18030, 256, 8>);
    impl ManBgReg {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Countermeasures/RWR
    /// Output: 0 if light is off, 1 if light is on
    pub struct Motverk(Led<18032, 2, 1>);
    impl Motverk {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Navigation System
    /// Output: 0 if light is off, 1 if light is on
    pub struct NavSyst(Led<18030, 8192, 13>);
    impl NavSyst {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Nose Gear
    /// Output: 0 if light is off, 1 if light is on
    pub struct Nosstall(Led<18028, 16, 4>);
    impl Nosstall {
        pub const fn new() -> Self {
            Self(Led::new())
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
    pub struct Oljetryck(Led<18028, 16384, 14>);
    impl Oljetryck {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Backup Hydraulic
    /// Output: 0 if light is off, 1 if light is on
    pub struct Reserveff(Led<18028, 512, 9>);
    impl Reserveff {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Radar Altimeter
    /// Output: 0 if light is off, 1 if light is on
    pub struct RhmFel(Led<18030, 2, 1>);
    impl RhmFel {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Roll Gearing
    /// Output: 0 if light is off, 1 if light is on
    pub struct RollVaxel(Led<18030, 4, 2>);
    impl RollVaxel {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// SPAK Error
    /// Output: 0 if light is off, 1 if light is on
    pub struct SpakError(Led<18028, 32768, 15>);
    impl SpakError {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Starter System
    /// Output: 0 if light is off, 1 if light is on
    pub struct Startsyst(Led<18030, 128, 7>);
    impl Startsyst {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Oxygen
    /// Output: 0 if light is off, 1 if light is on
    pub struct Syrgas(Led<18030, 512, 9>);
    impl Syrgas {
        pub const fn new() -> Self {
            Self(Led::new())
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
    pub struct Tandsyst(Led<18030, 64, 6>);
    impl Tandsyst {
        pub const fn new() -> Self {
            Self(Led::new())
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
    pub struct TankPump(Led<18028, 2, 1>);
    impl TankPump {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// TILS
    /// Output: 0 if light is off, 1 if light is on
    pub struct Tils(Led<18030, 4096, 12>);
    impl Tils {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Pitch Gearing
    /// Output: 0 if light is off, 1 if light is on
    pub struct Tippvaxel(Led<18028, 128, 7>);
    impl Tippvaxel {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// Left Gear
    /// Output: 0 if light is off, 1 if light is on
    pub struct Vstall(Led<18028, 32, 5>);
    impl Vstall {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// External Fuel Tank Feed System
    /// Output: 0 if light is off, 1 if light is on
    pub struct XTankBra(Led<18028, 1, 0>);
    impl XTankBra {
        pub const fn new() -> Self {
            Self(Led::new())
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
}

pub mod externalaircraftmodel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Formation Lights
    /// Output: Formation Lights
    pub struct ExtFormationLights(Metadata<18032, 128, 7>);
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

    /// Navigation Lights Tail
    /// Output: Navigation Lights Tail
    pub struct ExtNavLightsTail(Metadata<18032, 32, 5>);
    impl ExtNavLightsTail {
        pub const fn new() -> Self {
            Self(Metadata::new())
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
    pub struct ExtNavLightsWing(Metadata<18032, 16, 4>);
    impl ExtNavLightsWing {
        pub const fn new() -> Self {
            Self(Metadata::new())
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
    pub struct ExtPositionLights(Metadata<18032, 8, 3>);
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
    pub struct ExtSpeedBrakeLeft(Metadata<18052, 65535, 0>);
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
    pub struct ExtSpeedBrakeRight(Metadata<18050, 65535, 0>);
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
    pub struct ExtStrobeLights(Metadata<18032, 64, 6>);
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
}

pub mod flightdataunit {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// AFK 15 Deg Mode
    /// Input: toggle switch state
    /// Output: selector position
    pub struct Afk15DegMode(Selector<17940, 4096, 12>);
    impl Afk15DegMode {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// AFK Mode 3
    /// Input: toggle switch state
    /// Output: selector position
    pub struct AfkMode3(Selector<17940, 2048, 11>);
    impl AfkMode3 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// Altimeter Setting
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct AltSet(LimitedDial<18054, 65535, 0>);
    impl AltSet {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AltitudeHoldValue {
        Select1,
        Select2,
    }
    pub type AltitudeHoldValueCallback<'a> = &'a mut dyn FnMut(AltitudeHoldValue);
    impl core::convert::TryFrom<u16> for AltitudeHoldValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Altitude Hold
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AltitudeHold(Selector<17936, 32, 5>);
    impl AltitudeHold {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AttitudeHoldValue {
        Select1,
        Select2,
    }
    pub type AttitudeHoldValueCallback<'a> = &'a mut dyn FnMut(AttitudeHoldValue);
    impl core::convert::TryFrom<u16> for AttitudeHoldValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Attitude Hold
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AttitudeHold(Selector<17936, 16, 4>);
    impl AttitudeHold {
        pub const fn new() -> Self {
            Self(Selector::new())
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
    pub struct AutoYawTrim(LimitedDial<18060, 65535, 0>);
    impl AutoYawTrim {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BackAdiCageValue {
        Select1,
        Select2,
    }
    pub type BackAdiCageValueCallback<'a> = &'a mut dyn FnMut(BackAdiCageValue);
    impl core::convert::TryFrom<u16> for BackAdiCageValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Backup ADI Cage
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct BackAdiCage(Selector<18032, 512, 9>);
    impl BackAdiCage {
        pub const fn new() -> Self {
            Self(Selector::new())
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
    pub struct BackupAltSetting(LimitedDial<17938, 65535, 0>);
    impl BackupAltSetting {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CanopyJettisonValue {
        Select1,
        Select2,
    }
    pub type CanopyJettisonValueCallback<'a> = &'a mut dyn FnMut(CanopyJettisonValue);
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

    /// Canopy Jettison
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CanopyJettison(Selector<17940, 4, 2>);
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
    pub enum CanopyOpenCloseValue {
        Select1,
        Select2,
        Select3,
    }
    pub type CanopyOpenCloseValueCallback<'a> = &'a mut dyn FnMut(CanopyOpenCloseValue);
    impl core::convert::TryFrom<u16> for CanopyOpenCloseValue {
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

    /// Canopy Open/Close
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct CanopyOpenClose(Selector<17940, 3, 0>);
    impl CanopyOpenClose {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// Clock Setting
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct ClockSet(LimitedDial<18062, 65535, 0>);
    impl ClockSet {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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
    pub enum CountermeasureFastReleaseValue {
        Select1,
        Select2,
    }
    pub type CountermeasureFastReleaseValueCallback<'a> =
        &'a mut dyn FnMut(CountermeasureFastReleaseValue);
    impl core::convert::TryFrom<u16> for CountermeasureFastReleaseValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Countermeasure Fast Release
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CountermeasureFastRelease(Selector<17940, 8, 3>);
    impl CountermeasureFastRelease {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EjectionSeatArmValue {
        Select1,
        Select2,
    }
    pub type EjectionSeatArmValueCallback<'a> = &'a mut dyn FnMut(EjectionSeatArmValue);
    impl core::convert::TryFrom<u16> for EjectionSeatArmValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Ejection Seat Arm
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EjectionSeatArm(Selector<17936, 2048, 11>);
    impl EjectionSeatArm {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EmergencyPitchTrimValue {
        Select1,
        Select2,
    }
    pub type EmergencyPitchTrimValueCallback<'a> = &'a mut dyn FnMut(EmergencyPitchTrimValue);
    impl core::convert::TryFrom<u16> for EmergencyPitchTrimValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Emergency Pitch Trim
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EmergencyPitchTrim(Selector<17940, 32, 5>);
    impl EmergencyPitchTrim {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EmergencyRollTrimValue {
        Select1,
        Select2,
    }
    pub type EmergencyRollTrimValueCallback<'a> = &'a mut dyn FnMut(EmergencyRollTrimValue);
    impl core::convert::TryFrom<u16> for EmergencyRollTrimValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Emergency Roll Trim
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EmergencyRollTrim(Selector<17940, 16, 4>);
    impl EmergencyRollTrim {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EmergencyYawTrimValue {
        Select1,
        Select2,
    }
    pub type EmergencyYawTrimValueCallback<'a> = &'a mut dyn FnMut(EmergencyYawTrimValue);
    impl core::convert::TryFrom<u16> for EmergencyYawTrimValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Emergency Yaw Trim
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EmergencyYawTrim(Selector<17940, 64, 6>);
    impl EmergencyYawTrim {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// Gear Handle
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct GearHandle(Selector<17936, 4, 2>);
    impl GearHandle {
        pub const fn new() -> Self {
            Self(Selector::new())
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
    pub enum HojdCisiValue {
        Select1,
        Select2,
    }
    pub type HojdCisiValueCallback<'a> = &'a mut dyn FnMut(HojdCisiValue);
    impl core::convert::TryFrom<u16> for HojdCisiValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// HOJD CISI
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct HojdCisi(Selector<17936, 8192, 13>);
    impl HojdCisi {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// HUD Brightness Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct HudBright(LimitedDial<18056, 65535, 0>);
    impl HudBright {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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
    pub struct HudGlassPosition(Selector<17940, 1024, 10>);
    impl HudGlassPosition {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// Magnetic Declination Correction
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct MagCorrection(LimitedDial<18058, 65535, 0>);
    impl MagCorrection {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MagDecCoverValue {
        Select1,
        Select2,
    }
    pub type MagDecCoverValueCallback<'a> = &'a mut dyn FnMut(MagDecCoverValue);
    impl core::convert::TryFrom<u16> for MagDecCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Magnetic Declination Cover
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MagDecCover(Selector<17940, 128, 7>);
    impl MagDecCover {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MasterModeSelectValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
        Select7,
    }
    pub type MasterModeSelectValueCallback<'a> = &'a mut dyn FnMut(MasterModeSelectValue);
    impl core::convert::TryFrom<u16> for MasterModeSelectValue {
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

    /// Master Mode Selector
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct MasterModeSelect(Selector<17940, 57344, 13>);
    impl MasterModeSelect {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MaxGResetValue {
        Select1,
        Select2,
    }
    pub type MaxGResetValueCallback<'a> = &'a mut dyn FnMut(MaxGResetValue);
    impl core::convert::TryFrom<u16> for MaxGResetValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Max G Reset
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MaxGReset(Selector<18032, 256, 8>);
    impl MaxGReset {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum OxygenLeverValue {
        Select1,
        Select2,
    }
    pub type OxygenLeverValueCallback<'a> = &'a mut dyn FnMut(OxygenLeverValue);
    impl core::convert::TryFrom<u16> for OxygenLeverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Oxygen Lever
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct OxygenLever(Selector<17936, 32768, 15>);
    impl OxygenLever {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// Parking Brake
    /// Input: toggle switch state
    /// Output: selector position
    pub struct ParkingBrake(Selector<17940, 512, 9>);
    impl ParkingBrake {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PitchGearModeValue {
        Select1,
        Select2,
    }
    pub type PitchGearModeValueCallback<'a> = &'a mut dyn FnMut(PitchGearModeValue);
    impl core::convert::TryFrom<u16> for PitchGearModeValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pitch Gear Automatic/Landing
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct PitchGearMode(Selector<17936, 16384, 14>);
    impl PitchGearMode {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RollCenterValue {
        Select1,
        Select2,
    }
    pub type RollCenterValueCallback<'a> = &'a mut dyn FnMut(RollCenterValue);
    impl core::convert::TryFrom<u16> for RollCenterValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Roll Centering
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RollCenter(Selector<18032, 1024, 10>);
    impl RollCenter {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SlavSiValue {
        Select1,
        Select2,
    }
    pub type SlavSiValueCallback<'a> = &'a mut dyn FnMut(SlavSiValue);
    impl core::convert::TryFrom<u16> for SlavSiValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Slav SI
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SlavSi(Selector<17936, 4096, 12>);
    impl SlavSi {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SpakValue {
        Select1,
        Select2,
    }
    pub type SpakValueCallback<'a> = &'a mut dyn FnMut(SpakValue);
    impl core::convert::TryFrom<u16> for SpakValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// SPAK
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Spak(Selector<17936, 8, 3>);
    impl Spak {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TilsChannelLayerValue {
        Select1,
        Select2,
    }
    pub type TilsChannelLayerValueCallback<'a> = &'a mut dyn FnMut(TilsChannelLayerValue);
    impl core::convert::TryFrom<u16> for TilsChannelLayerValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// TILS Channel Layer Selection
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct TilsChannelLayer(Selector<17936, 1024, 10>);
    impl TilsChannelLayer {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TilsChannelSelectValue {
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
    pub type TilsChannelSelectValueCallback<'a> = &'a mut dyn FnMut(TilsChannelSelectValue);
    impl core::convert::TryFrom<u16> for TilsChannelSelectValue {
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

    /// TILS Channel Selection
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct TilsChannelSelect(Selector<17936, 960, 6>);
    impl TilsChannelSelect {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum YawTrimCoverValue {
        Select1,
        Select2,
    }
    pub type YawTrimCoverValueCallback<'a> = &'a mut dyn FnMut(YawTrimCoverValue);
    impl core::convert::TryFrom<u16> for YawTrimCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Autopilot Yaw Trim Cover
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct YawTrimCover(Selector<17940, 256, 8>);
    impl YawTrimCover {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod fr22radio {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22BaseValue {
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
    pub type Fr22BaseValueCallback<'a> = &'a mut dyn FnMut(Fr22BaseValue);
    impl core::convert::TryFrom<u16> for Fr22BaseValue {
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

    /// FR22 Base Selector
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Fr22Base(Selector<18066, 120, 3>);
    impl Fr22Base {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22ChannelAgValue {
        Select1,
        Select2,
    }
    pub type Fr22ChannelAgValueCallback<'a> = &'a mut dyn FnMut(Fr22ChannelAgValue);
    impl core::convert::TryFrom<u16> for Fr22ChannelAgValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Channel A/G
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Fr22ChannelAg(Selector<18064, 1024, 10>);
    impl Fr22ChannelAg {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22ChannelBValue {
        Select1,
        Select2,
    }
    pub type Fr22ChannelBValueCallback<'a> = &'a mut dyn FnMut(Fr22ChannelBValue);
    impl core::convert::TryFrom<u16> for Fr22ChannelBValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Channel B
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Fr22ChannelB(Selector<18064, 2048, 11>);
    impl Fr22ChannelB {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22ChannelC2Value {
        Select1,
        Select2,
    }
    pub type Fr22ChannelC2ValueCallback<'a> = &'a mut dyn FnMut(Fr22ChannelC2Value);
    impl core::convert::TryFrom<u16> for Fr22ChannelC2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Channel C2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Fr22ChannelC2(Selector<18064, 8192, 13>);
    impl Fr22ChannelC2 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22ChannelCfValue {
        Select1,
        Select2,
    }
    pub type Fr22ChannelCfValueCallback<'a> = &'a mut dyn FnMut(Fr22ChannelCfValue);
    impl core::convert::TryFrom<u16> for Fr22ChannelCfValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Channel C/F
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Fr22ChannelCf(Selector<18064, 4096, 12>);
    impl Fr22ChannelCf {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22ChannelDeValue {
        Select1,
        Select2,
    }
    pub type Fr22ChannelDeValueCallback<'a> = &'a mut dyn FnMut(Fr22ChannelDeValue);
    impl core::convert::TryFrom<u16> for Fr22ChannelDeValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Channel D/E
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Fr22ChannelDe(Selector<18064, 16384, 14>);
    impl Fr22ChannelDe {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22ChannelHValue {
        Select1,
        Select2,
    }
    pub type Fr22ChannelHValueCallback<'a> = &'a mut dyn FnMut(Fr22ChannelHValue);
    impl core::convert::TryFrom<u16> for Fr22ChannelHValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Channel H
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Fr22ChannelH(Selector<18064, 32, 5>);
    impl Fr22ChannelH {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22Flight0Value {
        Select1,
        Select2,
    }
    pub type Fr22Flight0ValueCallback<'a> = &'a mut dyn FnMut(Fr22Flight0Value);
    impl core::convert::TryFrom<u16> for Fr22Flight0Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Flight 0
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Fr22Flight0(Selector<18032, 2048, 11>);
    impl Fr22Flight0 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22Flight1Value {
        Select1,
        Select2,
    }
    pub type Fr22Flight1ValueCallback<'a> = &'a mut dyn FnMut(Fr22Flight1Value);
    impl core::convert::TryFrom<u16> for Fr22Flight1Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Flight 1
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Fr22Flight1(Selector<18032, 4096, 12>);
    impl Fr22Flight1 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22Flight2Value {
        Select1,
        Select2,
    }
    pub type Fr22Flight2ValueCallback<'a> = &'a mut dyn FnMut(Fr22Flight2Value);
    impl core::convert::TryFrom<u16> for Fr22Flight2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Flight 2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Fr22Flight2(Selector<18032, 8192, 13>);
    impl Fr22Flight2 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22Flight3Value {
        Select1,
        Select2,
    }
    pub type Fr22Flight3ValueCallback<'a> = &'a mut dyn FnMut(Fr22Flight3Value);
    impl core::convert::TryFrom<u16> for Fr22Flight3Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Flight 3
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Fr22Flight3(Selector<18032, 16384, 14>);
    impl Fr22Flight3 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22Flight4Value {
        Select1,
        Select2,
    }
    pub type Fr22Flight4ValueCallback<'a> = &'a mut dyn FnMut(Fr22Flight4Value);
    impl core::convert::TryFrom<u16> for Fr22Flight4Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Flight 4
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Fr22Flight4(Selector<18032, 32768, 15>);
    impl Fr22Flight4 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22Flight5Value {
        Select1,
        Select2,
    }
    pub type Fr22Flight5ValueCallback<'a> = &'a mut dyn FnMut(Fr22Flight5Value);
    impl core::convert::TryFrom<u16> for Fr22Flight5Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Flight 5
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Fr22Flight5(Selector<18064, 1, 0>);
    impl Fr22Flight5 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22Flight6Value {
        Select1,
        Select2,
    }
    pub type Fr22Flight6ValueCallback<'a> = &'a mut dyn FnMut(Fr22Flight6Value);
    impl core::convert::TryFrom<u16> for Fr22Flight6Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Flight 6
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Fr22Flight6(Selector<18064, 2, 1>);
    impl Fr22Flight6 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22Flight7Value {
        Select1,
        Select2,
    }
    pub type Fr22Flight7ValueCallback<'a> = &'a mut dyn FnMut(Fr22Flight7Value);
    impl core::convert::TryFrom<u16> for Fr22Flight7Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Flight 7
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Fr22Flight7(Selector<18064, 4, 2>);
    impl Fr22Flight7 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22Flight8Value {
        Select1,
        Select2,
    }
    pub type Fr22Flight8ValueCallback<'a> = &'a mut dyn FnMut(Fr22Flight8Value);
    impl core::convert::TryFrom<u16> for Fr22Flight8Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Flight 8
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Fr22Flight8(Selector<18064, 8, 3>);
    impl Fr22Flight8 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22Flight9Value {
        Select1,
        Select2,
    }
    pub type Fr22Flight9ValueCallback<'a> = &'a mut dyn FnMut(Fr22Flight9Value);
    impl core::convert::TryFrom<u16> for Fr22Flight9Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Flight 9
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Fr22Flight9(Selector<18064, 16, 4>);
    impl Fr22Flight9 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22GroundComValue {
        Select1,
        Select2,
    }
    pub type Fr22GroundComValueCallback<'a> = &'a mut dyn FnMut(Fr22GroundComValue);
    impl core::convert::TryFrom<u16> for Fr22GroundComValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Ground Intercom
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Fr22GroundCom(Selector<18064, 32768, 15>);
    impl Fr22GroundCom {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22GroupValue {
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
    pub type Fr22GroupValueCallback<'a> = &'a mut dyn FnMut(Fr22GroupValue);
    impl core::convert::TryFrom<u16> for Fr22GroupValue {
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

    /// FR22 Group Selector
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Fr22Group(Selector<18066, 1920, 7>);
    impl Fr22Group {
        pub const fn new() -> Self {
            Self(Selector::new())
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
    pub struct Fr22InnerLeftKnob(AnalogDial<17946, 65535, 0>);
    impl Fr22InnerLeftKnob {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
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
    pub struct Fr22InnerRightKnob(AnalogDial<17950, 65535, 0>);
    impl Fr22InnerRightKnob {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22MinusValue {
        Select1,
        Select2,
    }
    pub type Fr22MinusValueCallback<'a> = &'a mut dyn FnMut(Fr22MinusValue);
    impl core::convert::TryFrom<u16> for Fr22MinusValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Minus
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Fr22Minus(Selector<18064, 512, 9>);
    impl Fr22Minus {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22ModeValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
    }
    pub type Fr22ModeValueCallback<'a> = &'a mut dyn FnMut(Fr22ModeValue);
    impl core::convert::TryFrom<u16> for Fr22ModeValue {
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

    /// FR22 Mode Selector
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Fr22Mode(Selector<18066, 7, 0>);
    impl Fr22Mode {
        pub const fn new() -> Self {
            Self(Selector::new())
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
    pub struct Fr22OuterLeftKnob(AnalogDial<17948, 65535, 0>);
    impl Fr22OuterLeftKnob {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
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
    pub struct Fr22OuterRightKnob(AnalogDial<17952, 65535, 0>);
    impl Fr22OuterRightKnob {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
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
    pub struct Fr22SetModulation(Selector<17942, 32768, 15>);
    impl Fr22SetModulation {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22Special1Value {
        Select1,
        Select2,
    }
    pub type Fr22Special1ValueCallback<'a> = &'a mut dyn FnMut(Fr22Special1Value);
    impl core::convert::TryFrom<u16> for Fr22Special1Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Special 1
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Fr22Special1(Selector<18064, 64, 6>);
    impl Fr22Special1 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22Special2Value {
        Select1,
        Select2,
    }
    pub type Fr22Special2ValueCallback<'a> = &'a mut dyn FnMut(Fr22Special2Value);
    impl core::convert::TryFrom<u16> for Fr22Special2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Special 2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Fr22Special2(Selector<18064, 128, 7>);
    impl Fr22Special2 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fr22Special3Value {
        Select1,
        Select2,
    }
    pub type Fr22Special3ValueCallback<'a> = &'a mut dyn FnMut(Fr22Special3Value);
    impl core::convert::TryFrom<u16> for Fr22Special3Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Special 3
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Fr22Special3(Selector<18064, 256, 8>);
    impl Fr22Special3 {
        pub const fn new() -> Self {
            Self(Selector::new())
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
    pub struct Fr22Vol(AnalogDial<18068, 65535, 0>);
    impl Fr22Vol {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
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
}

pub mod gaugevalues {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Airspeed Value
    /// Output: Airspeed Value
    pub struct AirspeedValue(Metadata<18038, 65535, 0>);
    impl AirspeedValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// Altitude Value
    /// Output: Altitude Value
    pub struct AltitudeValue(Metadata<18040, 65535, 0>);
    impl AltitudeValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// CI Commanded Heading Value
    /// Output: CI Commanded Heading Value
    pub struct CiCommandHeadingValue(Metadata<18048, 65535, 0>);
    impl CiCommandHeadingValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// CI Heading Value
    /// Output: CI Heading Value
    pub struct CiHeadingValue(Metadata<18046, 65535, 0>);
    impl CiHeadingValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// Engine RPM Value
    /// Output: Engine RPM Value
    pub struct EngineRpmValue(Metadata<18034, 65535, 0>);
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

    /// Engine Temp Value
    /// Output: Engine Temp Value
    pub struct EngineTempValue(Metadata<18036, 65535, 0>);
    impl EngineTempValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// Fuel Level Value
    /// Output: Fuel Level Value
    pub struct FuelLevelValue(Metadata<18042, 65535, 0>);
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

    /// Fuel Needed Value
    /// Output: Fuel Needed Value
    pub struct FuelNeededValue(Metadata<18044, 65535, 0>);
    impl FuelNeededValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
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
}

pub mod indicatorlights {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// ATT Lamp
    /// Output: 0 if light is off, 1 if light is on
    pub struct AttLamp(Led<17944, 2048, 11>);
    impl AttLamp {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// HOJD Lamp
    /// Output: 0 if light is off, 1 if light is on
    pub struct HojdLamp(Led<17944, 4096, 12>);
    impl HojdLamp {
        pub const fn new() -> Self {
            Self(Led::new())
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

    /// SPAK Lamp
    /// Output: 0 if light is off, 1 if light is on
    pub struct SpakLamp(Led<17944, 1024, 10>);
    impl SpakLamp {
        pub const fn new() -> Self {
            Self(Led::new())
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
}

pub mod lights {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AntiCollisionLightsValue {
        Select1,
        Select2,
    }
    pub type AntiCollisionLightsValueCallback<'a> = &'a mut dyn FnMut(AntiCollisionLightsValue);
    impl core::convert::TryFrom<u16> for AntiCollisionLightsValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Anti Collision Lights
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AntiCollisionLights(Selector<17924, 16384, 14>);
    impl AntiCollisionLights {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EmergencyLightsValue {
        Select1,
        Select2,
    }
    pub type EmergencyLightsValueCallback<'a> = &'a mut dyn FnMut(EmergencyLightsValue);
    impl core::convert::TryFrom<u16> for EmergencyLightsValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Emergency Lights
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EmergencyLights(Selector<17926, 16, 4>);
    impl EmergencyLights {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// Flood Lights
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct FloodLights(LimitedDial<17928, 65535, 0>);
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
    pub enum FormationLightsValue {
        Select1,
        Select2,
    }
    pub type FormationLightsValueCallback<'a> = &'a mut dyn FnMut(FormationLightsValue);
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

    /// Formation Lights
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FormationLights(Selector<17924, 32768, 15>);
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

    /// Instrument Lights
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct InstrumentLights(LimitedDial<17932, 65535, 0>);
    impl InstrumentLights {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NavigationLightsValue {
        Select1,
        Select2,
        Select3,
    }
    pub type NavigationLightsValueCallback<'a> = &'a mut dyn FnMut(NavigationLightsValue);
    impl core::convert::TryFrom<u16> for NavigationLightsValue {
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

    /// Navigation Lights
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct NavigationLights(Selector<17926, 3, 0>);
    impl NavigationLights {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// Panel Lights
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct PanelLights(LimitedDial<17930, 65535, 0>);
    impl PanelLights {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PositionLightsValue {
        Select1,
        Select2,
    }
    pub type PositionLightsValueCallback<'a> = &'a mut dyn FnMut(PositionLightsValue);
    impl core::convert::TryFrom<u16> for PositionLightsValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Position Lights
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct PositionLights(Selector<17926, 4, 2>);
    impl PositionLights {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PositionLightsBrightnessValue {
        Select1,
        Select2,
        Select3,
    }
    pub type PositionLightsBrightnessValueCallback<'a> =
        &'a mut dyn FnMut(PositionLightsBrightnessValue);
    impl core::convert::TryFrom<u16> for PositionLightsBrightnessValue {
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

    /// Position Lights Brightness
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct PositionLightsBrightness(Selector<17926, 96, 5>);
    impl PositionLightsBrightness {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TaxiLandingLightsValue {
        Select1,
        Select2,
    }
    pub type TaxiLandingLightsValueCallback<'a> = &'a mut dyn FnMut(TaxiLandingLightsValue);
    impl core::convert::TryFrom<u16> for TaxiLandingLightsValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Taxi/Landing Lights
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct TaxiLandingLights(Selector<17926, 8, 3>);
    impl TaxiLandingLights {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod navigation {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DatapanelKey0Value {
        Select1,
        Select2,
    }
    pub type DatapanelKey0ValueCallback<'a> = &'a mut dyn FnMut(DatapanelKey0Value);
    impl core::convert::TryFrom<u16> for DatapanelKey0Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Datapanel Key 0
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct DatapanelKey0(Selector<17922, 256, 8>);
    impl DatapanelKey0 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DatapanelKey1Value {
        Select1,
        Select2,
    }
    pub type DatapanelKey1ValueCallback<'a> = &'a mut dyn FnMut(DatapanelKey1Value);
    impl core::convert::TryFrom<u16> for DatapanelKey1Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Datapanel Key 1
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct DatapanelKey1(Selector<17922, 512, 9>);
    impl DatapanelKey1 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DatapanelKey2Value {
        Select1,
        Select2,
    }
    pub type DatapanelKey2ValueCallback<'a> = &'a mut dyn FnMut(DatapanelKey2Value);
    impl core::convert::TryFrom<u16> for DatapanelKey2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Datapanel Key 2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct DatapanelKey2(Selector<17922, 1024, 10>);
    impl DatapanelKey2 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DatapanelKey3Value {
        Select1,
        Select2,
    }
    pub type DatapanelKey3ValueCallback<'a> = &'a mut dyn FnMut(DatapanelKey3Value);
    impl core::convert::TryFrom<u16> for DatapanelKey3Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Datapanel Key 3
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct DatapanelKey3(Selector<17922, 2048, 11>);
    impl DatapanelKey3 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DatapanelKey4Value {
        Select1,
        Select2,
    }
    pub type DatapanelKey4ValueCallback<'a> = &'a mut dyn FnMut(DatapanelKey4Value);
    impl core::convert::TryFrom<u16> for DatapanelKey4Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Datapanel Key 4
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct DatapanelKey4(Selector<17922, 4096, 12>);
    impl DatapanelKey4 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DatapanelKey5Value {
        Select1,
        Select2,
    }
    pub type DatapanelKey5ValueCallback<'a> = &'a mut dyn FnMut(DatapanelKey5Value);
    impl core::convert::TryFrom<u16> for DatapanelKey5Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Datapanel Key 5
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct DatapanelKey5(Selector<17922, 8192, 13>);
    impl DatapanelKey5 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DatapanelKey6Value {
        Select1,
        Select2,
    }
    pub type DatapanelKey6ValueCallback<'a> = &'a mut dyn FnMut(DatapanelKey6Value);
    impl core::convert::TryFrom<u16> for DatapanelKey6Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Datapanel Key 6
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct DatapanelKey6(Selector<17922, 16384, 14>);
    impl DatapanelKey6 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DatapanelKey7Value {
        Select1,
        Select2,
    }
    pub type DatapanelKey7ValueCallback<'a> = &'a mut dyn FnMut(DatapanelKey7Value);
    impl core::convert::TryFrom<u16> for DatapanelKey7Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Datapanel Key 7
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct DatapanelKey7(Selector<17922, 32768, 15>);
    impl DatapanelKey7 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DatapanelKey8Value {
        Select1,
        Select2,
    }
    pub type DatapanelKey8ValueCallback<'a> = &'a mut dyn FnMut(DatapanelKey8Value);
    impl core::convert::TryFrom<u16> for DatapanelKey8Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Datapanel Key 8
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct DatapanelKey8(Selector<17924, 1, 0>);
    impl DatapanelKey8 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DatapanelKey9Value {
        Select1,
        Select2,
    }
    pub type DatapanelKey9ValueCallback<'a> = &'a mut dyn FnMut(DatapanelKey9Value);
    impl core::convert::TryFrom<u16> for DatapanelKey9Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Datapanel Key 9
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct DatapanelKey9(Selector<17924, 2, 1>);
    impl DatapanelKey9 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NavSelectBtnB1Value {
        Select1,
        Select2,
    }
    pub type NavSelectBtnB1ValueCallback<'a> = &'a mut dyn FnMut(NavSelectBtnB1Value);
    impl core::convert::TryFrom<u16> for NavSelectBtnB1Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Navigation Selector Button B1
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NavSelectBtnB1(Selector<17924, 4, 2>);
    impl NavSelectBtnB1 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NavSelectBtnB2Value {
        Select1,
        Select2,
    }
    pub type NavSelectBtnB2ValueCallback<'a> = &'a mut dyn FnMut(NavSelectBtnB2Value);
    impl core::convert::TryFrom<u16> for NavSelectBtnB2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Navigation Selector Button B2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NavSelectBtnB2(Selector<17924, 8, 3>);
    impl NavSelectBtnB2 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NavSelectBtnB3Value {
        Select1,
        Select2,
    }
    pub type NavSelectBtnB3ValueCallback<'a> = &'a mut dyn FnMut(NavSelectBtnB3Value);
    impl core::convert::TryFrom<u16> for NavSelectBtnB3Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Navigation Selector Button B3
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NavSelectBtnB3(Selector<17924, 16, 4>);
    impl NavSelectBtnB3 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NavSelectBtnB4Value {
        Select1,
        Select2,
    }
    pub type NavSelectBtnB4ValueCallback<'a> = &'a mut dyn FnMut(NavSelectBtnB4Value);
    impl core::convert::TryFrom<u16> for NavSelectBtnB4Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Navigation Selector Button B4
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NavSelectBtnB4(Selector<17924, 32, 5>);
    impl NavSelectBtnB4 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NavSelectBtnB5Value {
        Select1,
        Select2,
    }
    pub type NavSelectBtnB5ValueCallback<'a> = &'a mut dyn FnMut(NavSelectBtnB5Value);
    impl core::convert::TryFrom<u16> for NavSelectBtnB5Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Navigation Selector Button B5
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NavSelectBtnB5(Selector<17924, 64, 6>);
    impl NavSelectBtnB5 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NavSelectBtnB6Value {
        Select1,
        Select2,
    }
    pub type NavSelectBtnB6ValueCallback<'a> = &'a mut dyn FnMut(NavSelectBtnB6Value);
    impl core::convert::TryFrom<u16> for NavSelectBtnB6Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Navigation Selector Button B6
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NavSelectBtnB6(Selector<17924, 128, 7>);
    impl NavSelectBtnB6 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NavSelectBtnB7Value {
        Select1,
        Select2,
    }
    pub type NavSelectBtnB7ValueCallback<'a> = &'a mut dyn FnMut(NavSelectBtnB7Value);
    impl core::convert::TryFrom<u16> for NavSelectBtnB7Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Navigation Selector Button B7
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NavSelectBtnB7(Selector<17924, 256, 8>);
    impl NavSelectBtnB7 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NavSelectBtnB8Value {
        Select1,
        Select2,
    }
    pub type NavSelectBtnB8ValueCallback<'a> = &'a mut dyn FnMut(NavSelectBtnB8Value);
    impl core::convert::TryFrom<u16> for NavSelectBtnB8Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Navigation Selector Button B8
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NavSelectBtnB8(Selector<17924, 512, 9>);
    impl NavSelectBtnB8 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NavSelectBtnB9Value {
        Select1,
        Select2,
    }
    pub type NavSelectBtnB9ValueCallback<'a> = &'a mut dyn FnMut(NavSelectBtnB9Value);
    impl core::convert::TryFrom<u16> for NavSelectBtnB9Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Navigation Selector Button B9
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NavSelectBtnB9(Selector<17924, 1024, 10>);
    impl NavSelectBtnB9 {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NavSelectBtnBxValue {
        Select1,
        Select2,
    }
    pub type NavSelectBtnBxValueCallback<'a> = &'a mut dyn FnMut(NavSelectBtnBxValue);
    impl core::convert::TryFrom<u16> for NavSelectBtnBxValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Navigation Selector Button BX
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NavSelectBtnBx(Selector<17924, 2048, 11>);
    impl NavSelectBtnBx {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NavSelectBtnLMalValue {
        Select1,
        Select2,
    }
    pub type NavSelectBtnLMalValueCallback<'a> = &'a mut dyn FnMut(NavSelectBtnLMalValue);
    impl core::convert::TryFrom<u16> for NavSelectBtnLMalValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Navigation Selector Button L MAL
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NavSelectBtnLMal(Selector<17924, 8192, 13>);
    impl NavSelectBtnLMal {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NavSelectBtnLsValue {
        Select1,
        Select2,
    }
    pub type NavSelectBtnLsValueCallback<'a> = &'a mut dyn FnMut(NavSelectBtnLsValue);
    impl core::convert::TryFrom<u16> for NavSelectBtnLsValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Navigation Selector Button LS
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NavSelectBtnLs(Selector<17924, 4096, 12>);
    impl NavSelectBtnLs {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod navigationpanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Destination Panel Data Digit 1
    /// Output: Destination Panel Data Digit 1
    pub struct Ajs37DestIndicatorData1(Display<18094, 1>);
    impl Ajs37DestIndicatorData1 {
        pub const fn new() -> Self {
            Self(Display::new())
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
    pub struct Ajs37DestIndicatorData2(Display<18096, 1>);
    impl Ajs37DestIndicatorData2 {
        pub const fn new() -> Self {
            Self(Display::new())
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
    pub struct Ajs37NavIndicatorData1(Display<18082, 1>);
    impl Ajs37NavIndicatorData1 {
        pub const fn new() -> Self {
            Self(Display::new())
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
    pub struct Ajs37NavIndicatorData2(Display<18084, 1>);
    impl Ajs37NavIndicatorData2 {
        pub const fn new() -> Self {
            Self(Display::new())
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
    pub struct Ajs37NavIndicatorData3(Display<18086, 1>);
    impl Ajs37NavIndicatorData3 {
        pub const fn new() -> Self {
            Self(Display::new())
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
    pub struct Ajs37NavIndicatorData4(Display<18088, 1>);
    impl Ajs37NavIndicatorData4 {
        pub const fn new() -> Self {
            Self(Display::new())
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
    pub struct Ajs37NavIndicatorData5(Display<18090, 1>);
    impl Ajs37NavIndicatorData5 {
        pub const fn new() -> Self {
            Self(Display::new())
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
    pub struct Ajs37NavIndicatorData6(Display<18092, 1>);
    impl Ajs37NavIndicatorData6 {
        pub const fn new() -> Self {
            Self(Display::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ck37RensaClearValue {
        Select1,
        Select2,
    }
    pub type Ck37RensaClearValueCallback<'a> = &'a mut dyn FnMut(Ck37RensaClearValue);
    impl core::convert::TryFrom<u16> for Ck37RensaClearValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CK37 Rensa Clear
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Ck37RensaClear(Selector<17942, 32, 5>);
    impl Ck37RensaClear {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DataInOutValue {
        Select1,
        Select2,
    }
    pub type DataInOutValueCallback<'a> = &'a mut dyn FnMut(DataInOutValue);
    impl core::convert::TryFrom<u16> for DataInOutValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Data In/Out
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct DataInOut(Selector<17942, 8, 3>);
    impl DataInOut {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DatapanelSelectorValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
        Select7,
    }
    pub type DatapanelSelectorValueCallback<'a> = &'a mut dyn FnMut(DatapanelSelectorValue);
    impl core::convert::TryFrom<u16> for DatapanelSelectorValue {
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

    /// Datapanel Selector
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct DatapanelSelector(Selector<17942, 7, 0>);
    impl DatapanelSelector {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RensaButtonCoverValue {
        Select1,
        Select2,
    }
    pub type RensaButtonCoverValueCallback<'a> = &'a mut dyn FnMut(RensaButtonCoverValue);
    impl core::convert::TryFrom<u16> for RensaButtonCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Rensa Button Cover
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RensaButtonCover(Selector<17942, 16, 4>);
    impl RensaButtonCover {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod radar {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AntiJamModeValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
        Select7,
        Select8,
    }
    pub type AntiJamModeValueCallback<'a> = &'a mut dyn FnMut(AntiJamModeValue);
    impl core::convert::TryFrom<u16> for AntiJamModeValue {
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

    /// Anti Jamming Mode (AS) Selector
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct AntiJamMode(Selector<17922, 7, 0>);
    impl AntiJamMode {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffCodeValue {
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
    pub type IffCodeValueCallback<'a> = &'a mut dyn FnMut(IffCodeValue);
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
                6 => Ok(Self::Select7),
                7 => Ok(Self::Select8),
                8 => Ok(Self::Select9),
                9 => Ok(Self::Select10),
                10 => Ok(Self::Select11),
                value => Err(value),
            }
        }
    }

    /// IFF Code
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffCode(Selector<18066, 61440, 12>);
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
    pub enum IffPowerValue {
        Select1,
        Select2,
    }
    pub type IffPowerValueCallback<'a> = &'a mut dyn FnMut(IffPowerValue);
    impl core::convert::TryFrom<u16> for IffPowerValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// IFF Power
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IffPower(Selector<18066, 2048, 11>);
    impl IffPower {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// Radar Brightness
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RadarBright(LimitedDial<18080, 65535, 0>);
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RadarGainValue {
        Select1,
        Select2,
    }
    pub type RadarGainValueCallback<'a> = &'a mut dyn FnMut(RadarGainValue);
    impl core::convert::TryFrom<u16> for RadarGainValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Lin/Log Radar Gain Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RadarGain(Selector<17920, 16384, 14>);
    impl RadarGain {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RadarIffIdValue {
        Select1,
        Select2,
    }
    pub type RadarIffIdValueCallback<'a> = &'a mut dyn FnMut(RadarIffIdValue);
    impl core::convert::TryFrom<u16> for RadarIffIdValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// IFF Identification
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RadarIffId(Selector<17922, 64, 6>);
    impl RadarIffId {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RadarIgnitionCoilsValue {
        Select1,
        Select2,
    }
    pub type RadarIgnitionCoilsValueCallback<'a> = &'a mut dyn FnMut(RadarIgnitionCoilsValue);
    impl core::convert::TryFrom<u16> for RadarIgnitionCoilsValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Ignition Coils
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RadarIgnitionCoils(Selector<17922, 32, 5>);
    impl RadarIgnitionCoils {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RadarMaintenanceTestValue {
        Select1,
        Select2,
    }
    pub type RadarMaintenanceTestValueCallback<'a> = &'a mut dyn FnMut(RadarMaintenanceTestValue);
    impl core::convert::TryFrom<u16> for RadarMaintenanceTestValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Radar/EL Maintenance Test
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RadarMaintenanceTest(Selector<17922, 16, 4>);
    impl RadarMaintenanceTest {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RadarPulseNormalShortValue {
        Select1,
        Select2,
    }
    pub type RadarPulseNormalShortValueCallback<'a> = &'a mut dyn FnMut(RadarPulseNormalShortValue);
    impl core::convert::TryFrom<u16> for RadarPulseNormalShortValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pulse Normal/Short Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RadarPulseNormalShort(Selector<17920, 32768, 15>);
    impl RadarPulseNormalShort {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RadarRecceOnOffValue {
        Select1,
        Select2,
    }
    pub type RadarRecceOnOffValueCallback<'a> = &'a mut dyn FnMut(RadarRecceOnOffValue);
    impl core::convert::TryFrom<u16> for RadarRecceOnOffValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Passive Recce On/Off Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RadarRecceOnOff(Selector<17922, 8, 3>);
    impl RadarRecceOnOff {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod radaraltimeter {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RadarAltimeterPowerValue {
        Select1,
        Select2,
    }
    pub type RadarAltimeterPowerValueCallback<'a> = &'a mut dyn FnMut(RadarAltimeterPowerValue);
    impl core::convert::TryFrom<u16> for RadarAltimeterPowerValue {
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
    pub struct RadarAltimeterPower(Selector<17936, 1, 0>);
    impl RadarAltimeterPower {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod rawgaugevalues {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// ADI AoA Indicator
    /// Output: gauge position
    pub struct AdiAoaIndicator(AnalogGauge<18004, 65535, 0>);
    impl AdiAoaIndicator {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct AdiHeading(AnalogGauge<17992, 65535, 0>);
    impl AdiHeading {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct AdiHorizontalIls(AnalogGauge<18000, 65535, 0>);
    impl AdiHorizontalIls {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct AdiPitch(AnalogGauge<17990, 65535, 0>);
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

    /// ADI Roll
    /// Output: gauge position
    pub struct AdiRoll(AnalogGauge<17994, 65535, 0>);
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

    /// ADI Slipball
    /// Output: gauge position
    pub struct AdiSlipball(AnalogGauge<18002, 65535, 0>);
    impl AdiSlipball {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct AdiVerticalIls(AnalogGauge<17998, 65535, 0>);
    impl AdiVerticalIls {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct AdiVerticalVelocity(AnalogGauge<17996, 65535, 0>);
    impl AdiVerticalVelocity {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Airspeed m/s
    /// Output: gauge position
    pub struct AirspeedMS(AnalogGauge<17968, 65535, 0>);
    impl AirspeedMS {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Altimeter 10000 Meter
    /// Output: gauge position
    pub struct Altimeter10000m(AnalogGauge<17978, 65535, 0>);
    impl Altimeter10000m {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct Altimeter1000m(AnalogGauge<17980, 65535, 0>);
    impl Altimeter1000m {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct AltimeterBaro1000Hpa(AnalogGauge<17988, 65535, 0>);
    impl AltimeterBaro1000Hpa {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct AltimeterBaro100Hpa(AnalogGauge<17986, 65535, 0>);
    impl AltimeterBaro100Hpa {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct AltimeterBaro10Hpa(AnalogGauge<17984, 65535, 0>);
    impl AltimeterBaro10Hpa {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct AltimeterBaro1Hpa(AnalogGauge<17982, 65535, 0>);
    impl AltimeterBaro1Hpa {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Backup Altimeter 10000 Meter
    /// Output: gauge position
    pub struct BackupAltimeter10000m(AnalogGauge<18014, 65535, 0>);
    impl BackupAltimeter10000m {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct BackupAltimeter1000m(AnalogGauge<18016, 65535, 0>);
    impl BackupAltimeter1000m {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Backup Indicated Airspeed
    /// Output: gauge position
    pub struct BackupIndicatedAirspeed(AnalogGauge<18012, 65535, 0>);
    impl BackupIndicatedAirspeed {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct BackupPitch(AnalogGauge<18018, 65535, 0>);
    impl BackupPitch {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct BackupRoll(AnalogGauge<18020, 65535, 0>);
    impl BackupRoll {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// CI Commanded Heading
    /// Output: gauge position
    pub struct CiCommandedHeading(AnalogGauge<18008, 65535, 0>);
    impl CiCommandedHeading {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct CiHeading(AnalogGauge<18006, 65535, 0>);
    impl CiHeading {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Distance Indicator
    /// Output: gauge position
    pub struct DistanceIndicator(AnalogGauge<18022, 65535, 0>);
    impl DistanceIndicator {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Engine RPM 10
    /// Output: gauge position
    pub struct EngineRpm10(AnalogGauge<17964, 65535, 0>);
    impl EngineRpm10 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct EngineRpm100(AnalogGauge<17962, 65535, 0>);
    impl EngineRpm100 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Engine Temp
    /// Output: gauge position
    pub struct EngineTemp(AnalogGauge<17966, 65535, 0>);
    impl EngineTemp {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Fuel Level
    /// Output: gauge position
    pub struct FuelLevel(AnalogGauge<18024, 65535, 0>);
    impl FuelLevel {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Fuel Needed
    /// Output: gauge position
    pub struct FuelNeeded(AnalogGauge<18026, 65535, 0>);
    impl FuelNeeded {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Mach Meter First Decimal 0.X0
    /// Output: gauge position
    pub struct MachMeterFirstDecimal(AnalogGauge<17972, 65535, 0>);
    impl MachMeterFirstDecimal {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct MachMeterInteger(AnalogGauge<17970, 65535, 0>);
    impl MachMeterInteger {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct MachMeterSecondDecimal(AnalogGauge<17974, 65535, 0>);
    impl MachMeterSecondDecimal {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Magnetic Heading
    /// Output: gauge position
    pub struct MagneticHeading(AnalogGauge<18010, 65535, 0>);
    impl MagneticHeading {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Pedals
    /// Output: gauge position
    pub struct Pedals(AnalogGauge<17958, 65535, 0>);
    impl Pedals {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Stick Pitch
    /// Output: gauge position
    pub struct StickPitch(AnalogGauge<17954, 65535, 0>);
    impl StickPitch {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
    pub struct StickRoll(AnalogGauge<17956, 65535, 0>);
    impl StickRoll {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Throttle
    /// Output: gauge position
    pub struct Throttle(AnalogGauge<17960, 65535, 0>);
    impl Throttle {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Vertical Acceleration G Meter
    /// Output: gauge position
    pub struct VerticalAcceleration(AnalogGauge<17976, 65535, 0>);
    impl VerticalAcceleration {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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
}

pub mod rwr {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Master Volume / Sidewinder Tone
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct MasterVol(LimitedDial<18078, 65535, 0>);
    impl MasterVol {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RadarWarnSelectValue {
        Select1,
        Select2,
        Select3,
    }
    pub type RadarWarnSelectValueCallback<'a> = &'a mut dyn FnMut(RadarWarnSelectValue);
    impl core::convert::TryFrom<u16> for RadarWarnSelectValue {
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

    /// Radar Warning Indication Selector
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct RadarWarnSelect(Selector<17942, 192, 6>);
    impl RadarWarnSelect {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod thrustreverser {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ReversalValue {
        Select1,
        Select2,
    }
    pub type ReversalValueCallback<'a> = &'a mut dyn FnMut(ReversalValue);
    impl core::convert::TryFrom<u16> for ReversalValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Thrust Reverser On/Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Reversal(Selector<17922, 128, 7>);
    impl Reversal {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod warningpanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IndicatorSystemTestValue {
        Select1,
        Select2,
    }
    pub type IndicatorSystemTestValueCallback<'a> = &'a mut dyn FnMut(IndicatorSystemTestValue);
    impl core::convert::TryFrom<u16> for IndicatorSystemTestValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Indicator System Test
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IndicatorSystemTest(Selector<17942, 512, 9>);
    impl IndicatorSystemTest {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MasterCautionResetValue {
        Select1,
        Select2,
    }
    pub type MasterCautionResetValueCallback<'a> = &'a mut dyn FnMut(MasterCautionResetValue);
    impl core::convert::TryFrom<u16> for MasterCautionResetValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Master Caution Reset
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MasterCautionReset(Selector<17942, 1024, 10>);
    impl MasterCautionReset {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WarningPanelTestValue {
        Select1,
        Select2,
    }
    pub type WarningPanelTestValueCallback<'a> = &'a mut dyn FnMut(WarningPanelTestValue);
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

    /// Warning Panel Light Test
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct WarningPanelTest(Selector<17942, 256, 8>);
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
}

pub mod weaponsystem {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RbBkRelModeValue {
        Select1,
        Select2,
    }
    pub type RbBkRelModeValueCallback<'a> = &'a mut dyn FnMut(RbBkRelModeValue);
    impl core::convert::TryFrom<u16> for RbBkRelModeValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RB-04/RB-15/BK Release Mode Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RbBkRelMode(Selector<17920, 8192, 13>);
    impl RbBkRelMode {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TankReleaseValue {
        Select1,
        Select2,
    }
    pub type TankReleaseValueCallback<'a> = &'a mut dyn FnMut(TankReleaseValue);
    impl core::convert::TryFrom<u16> for TankReleaseValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// External Tank Release Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct TankRelease(Selector<17920, 16, 4>);
    impl TankRelease {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TankReleaseCoverValue {
        Select1,
        Select2,
    }
    pub type TankReleaseCoverValueCallback<'a> = &'a mut dyn FnMut(TankReleaseCoverValue);
    impl core::convert::TryFrom<u16> for TankReleaseCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// External Tank Release Cover
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct TankReleaseCover(Selector<17920, 8, 3>);
    impl TankReleaseCover {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TriggerSafetyBracketValue {
        Select1,
        Select2,
    }
    pub type TriggerSafetyBracketValueCallback<'a> = &'a mut dyn FnMut(TriggerSafetyBracketValue);
    impl core::convert::TryFrom<u16> for TriggerSafetyBracketValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Trigger Safety Bracket
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct TriggerSafetyBracket(Selector<17920, 1, 0>);
    impl TriggerSafetyBracket {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WeaponIntervalValue {
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
    pub type WeaponIntervalValueCallback<'a> = &'a mut dyn FnMut(WeaponIntervalValue);
    impl core::convert::TryFrom<u16> for WeaponIntervalValue {
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

    /// Weapon Interval Selector Mode Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct WeaponInterval(Selector<17920, 3840, 8>);
    impl WeaponInterval {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WeaponRelModeValue {
        Select1,
        Select2,
    }
    pub type WeaponRelModeValueCallback<'a> = &'a mut dyn FnMut(WeaponRelModeValue);
    impl core::convert::TryFrom<u16> for WeaponRelModeValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Weapon Release Mode Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct WeaponRelMode(Selector<17920, 4096, 12>);
    impl WeaponRelMode {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WeaponReleaseValue {
        Select1,
        Select2,
    }
    pub type WeaponReleaseValueCallback<'a> = &'a mut dyn FnMut(WeaponReleaseValue);
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

    /// Weapon Emergency Release Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct WeaponRelease(Selector<17920, 4, 2>);
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WeaponReleaseCoverValue {
        Select1,
        Select2,
    }
    pub type WeaponReleaseCoverValueCallback<'a> = &'a mut dyn FnMut(WeaponReleaseCoverValue);
    impl core::convert::TryFrom<u16> for WeaponReleaseCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Weapon Emergency Release Cover
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct WeaponReleaseCover(Selector<17920, 2, 1>);
    impl WeaponReleaseCover {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WeaponSelectValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
    }
    pub type WeaponSelectValueCallback<'a> = &'a mut dyn FnMut(WeaponSelectValue);
    impl core::convert::TryFrom<u16> for WeaponSelectValue {
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

    /// Weapon Selector Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct WeaponSelect(Selector<17920, 224, 5>);
    impl WeaponSelect {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub enum RegisteredControl<'a> {
    AdiAoaIndicator {
        control: rawgaugevalues::AdiAoaIndicator,
        callback: IntegerCallback<'a>,
    },
    AdiHeading {
        control: rawgaugevalues::AdiHeading,
        callback: IntegerCallback<'a>,
    },
    AdiHorizontalIls {
        control: rawgaugevalues::AdiHorizontalIls,
        callback: IntegerCallback<'a>,
    },
    AdiPitch {
        control: rawgaugevalues::AdiPitch,
        callback: IntegerCallback<'a>,
    },
    AdiRoll {
        control: rawgaugevalues::AdiRoll,
        callback: IntegerCallback<'a>,
    },
    AdiSlipball {
        control: rawgaugevalues::AdiSlipball,
        callback: IntegerCallback<'a>,
    },
    AdiVerticalIls {
        control: rawgaugevalues::AdiVerticalIls,
        callback: IntegerCallback<'a>,
    },
    AdiVerticalVelocity {
        control: rawgaugevalues::AdiVerticalVelocity,
        callback: IntegerCallback<'a>,
    },
    Afk15DegMode {
        control: flightdataunit::Afk15DegMode,
        callback: IntegerCallback<'a>,
    },
    AfkFel {
        control: errorpanel::AfkFel,
        callback: LedCallback<'a>,
    },
    AfkLever {
        control: enginepanel::AfkLever,
        callback: enginepanel::AfkLeverValueCallback<'a>,
    },
    AfkMode3 {
        control: flightdataunit::AfkMode3,
        callback: IntegerCallback<'a>,
    },
    AirspeedMS {
        control: rawgaugevalues::AirspeedMS,
        callback: IntegerCallback<'a>,
    },
    AirspeedValue {
        control: gaugevalues::AirspeedValue,
        callback: IntegerCallback<'a>,
    },
    Ajs37DestIndicatorData1 {
        control: navigationpanel::Ajs37DestIndicatorData1,
        callback: DisplayCallback<'a>,
    },
    Ajs37DestIndicatorData2 {
        control: navigationpanel::Ajs37DestIndicatorData2,
        callback: DisplayCallback<'a>,
    },
    Ajs37NavIndicatorData1 {
        control: navigationpanel::Ajs37NavIndicatorData1,
        callback: DisplayCallback<'a>,
    },
    Ajs37NavIndicatorData2 {
        control: navigationpanel::Ajs37NavIndicatorData2,
        callback: DisplayCallback<'a>,
    },
    Ajs37NavIndicatorData3 {
        control: navigationpanel::Ajs37NavIndicatorData3,
        callback: DisplayCallback<'a>,
    },
    Ajs37NavIndicatorData4 {
        control: navigationpanel::Ajs37NavIndicatorData4,
        callback: DisplayCallback<'a>,
    },
    Ajs37NavIndicatorData5 {
        control: navigationpanel::Ajs37NavIndicatorData5,
        callback: DisplayCallback<'a>,
    },
    Ajs37NavIndicatorData6 {
        control: navigationpanel::Ajs37NavIndicatorData6,
        callback: DisplayCallback<'a>,
    },
    AltSet {
        control: flightdataunit::AltSet,
        callback: IntegerCallback<'a>,
    },
    Altimeter10000m {
        control: rawgaugevalues::Altimeter10000m,
        callback: IntegerCallback<'a>,
    },
    Altimeter1000m {
        control: rawgaugevalues::Altimeter1000m,
        callback: IntegerCallback<'a>,
    },
    AltimeterBaro1000Hpa {
        control: rawgaugevalues::AltimeterBaro1000Hpa,
        callback: IntegerCallback<'a>,
    },
    AltimeterBaro100Hpa {
        control: rawgaugevalues::AltimeterBaro100Hpa,
        callback: IntegerCallback<'a>,
    },
    AltimeterBaro10Hpa {
        control: rawgaugevalues::AltimeterBaro10Hpa,
        callback: IntegerCallback<'a>,
    },
    AltimeterBaro1Hpa {
        control: rawgaugevalues::AltimeterBaro1Hpa,
        callback: IntegerCallback<'a>,
    },
    AltitudeHold {
        control: flightdataunit::AltitudeHold,
        callback: flightdataunit::AltitudeHoldValueCallback<'a>,
    },
    AltitudeValue {
        control: gaugevalues::AltitudeValue,
        callback: IntegerCallback<'a>,
    },
    AntiCollisionLights {
        control: lights::AntiCollisionLights,
        callback: lights::AntiCollisionLightsValueCallback<'a>,
    },
    AntiJamMode {
        control: radar::AntiJamMode,
        callback: radar::AntiJamModeValueCallback<'a>,
    },
    AttLamp {
        control: indicatorlights::AttLamp,
        callback: LedCallback<'a>,
    },
    AttitudeHold {
        control: flightdataunit::AttitudeHold,
        callback: flightdataunit::AttitudeHoldValueCallback<'a>,
    },
    AutoYawTrim {
        control: flightdataunit::AutoYawTrim,
        callback: IntegerCallback<'a>,
    },
    BackAdiCage {
        control: flightdataunit::BackAdiCage,
        callback: flightdataunit::BackAdiCageValueCallback<'a>,
    },
    BackupAltSetting {
        control: flightdataunit::BackupAltSetting,
        callback: IntegerCallback<'a>,
    },
    BackupAltimeter10000m {
        control: rawgaugevalues::BackupAltimeter10000m,
        callback: IntegerCallback<'a>,
    },
    BackupAltimeter1000m {
        control: rawgaugevalues::BackupAltimeter1000m,
        callback: IntegerCallback<'a>,
    },
    BackupGenerator {
        control: electricsystem::BackupGenerator,
        callback: electricsystem::BackupGeneratorValueCallback<'a>,
    },
    BackupIndicatedAirspeed {
        control: rawgaugevalues::BackupIndicatedAirspeed,
        callback: IntegerCallback<'a>,
    },
    BackupPitch {
        control: rawgaugevalues::BackupPitch,
        callback: IntegerCallback<'a>,
    },
    BackupRoll {
        control: rawgaugevalues::BackupRoll,
        callback: IntegerCallback<'a>,
    },
    Bra24 {
        control: errorpanel::Bra24,
        callback: LedCallback<'a>,
    },
    BraUppf {
        control: errorpanel::BraUppf,
        callback: LedCallback<'a>,
    },
    Brand1 {
        control: errorpanel::Brand1,
        callback: LedCallback<'a>,
    },
    Brand2 {
        control: errorpanel::Brand2,
        callback: LedCallback<'a>,
    },
    BrandGts {
        control: errorpanel::BrandGts,
        callback: LedCallback<'a>,
    },
    CabinAirValve {
        control: enginepanel::CabinAirValve,
        callback: enginepanel::CabinAirValveValueCallback<'a>,
    },
    CanopyJettison {
        control: flightdataunit::CanopyJettison,
        callback: flightdataunit::CanopyJettisonValueCallback<'a>,
    },
    CanopyOpenClose {
        control: flightdataunit::CanopyOpenClose,
        callback: flightdataunit::CanopyOpenCloseValueCallback<'a>,
    },
    CbAutopilot {
        control: enginepanel::CbAutopilot,
        callback: enginepanel::CbAutopilotValueCallback<'a>,
    },
    CbCiSi {
        control: enginepanel::CbCiSi,
        callback: enginepanel::CbCiSiValueCallback<'a>,
    },
    CbEjectionSystem {
        control: enginepanel::CbEjectionSystem,
        callback: enginepanel::CbEjectionSystemValueCallback<'a>,
    },
    CbEngine {
        control: enginepanel::CbEngine,
        callback: enginepanel::CbEngineValueCallback<'a>,
    },
    CbHighAlphaWarn {
        control: enginepanel::CbHighAlphaWarn,
        callback: enginepanel::CbHighAlphaWarnValueCallback<'a>,
    },
    CbTrimSystem {
        control: enginepanel::CbTrimSystem,
        callback: enginepanel::CbTrimSystemValueCallback<'a>,
    },
    CiCommandHeadingValue {
        control: gaugevalues::CiCommandHeadingValue,
        callback: IntegerCallback<'a>,
    },
    CiCommandedHeading {
        control: rawgaugevalues::CiCommandedHeading,
        callback: IntegerCallback<'a>,
    },
    CiHeading {
        control: rawgaugevalues::CiHeading,
        callback: IntegerCallback<'a>,
    },
    CiHeadingValue {
        control: gaugevalues::CiHeadingValue,
        callback: IntegerCallback<'a>,
    },
    Ck {
        control: errorpanel::Ck,
        callback: LedCallback<'a>,
    },
    Ck37RensaClear {
        control: navigationpanel::Ck37RensaClear,
        callback: navigationpanel::Ck37RensaClearValueCallback<'a>,
    },
    ClockSet {
        control: flightdataunit::ClockSet,
        callback: IntegerCallback<'a>,
    },
    CountermeasureChaffFlaresSelector {
        control: countermeasures::CountermeasureChaffFlaresSelector,
        callback: countermeasures::CountermeasureChaffFlaresSelectorValueCallback<'a>,
    },
    CountermeasureFastRelease {
        control: flightdataunit::CountermeasureFastRelease,
        callback: flightdataunit::CountermeasureFastReleaseValueCallback<'a>,
    },
    CountermeasureModeSelector {
        control: countermeasures::CountermeasureModeSelector,
        callback: countermeasures::CountermeasureModeSelectorValueCallback<'a>,
    },
    CountermeasureReleaseMode {
        control: countermeasures::CountermeasureReleaseMode,
        callback: countermeasures::CountermeasureReleaseModeValueCallback<'a>,
    },
    CountermeasureStreakModeSelector {
        control: countermeasures::CountermeasureStreakModeSelector,
        callback: countermeasures::CountermeasureStreakModeSelectorValueCallback<'a>,
    },
    DataCartridge {
        control: enginepanel::DataCartridge,
        callback: enginepanel::DataCartridgeValueCallback<'a>,
    },
    DataInOut {
        control: navigationpanel::DataInOut,
        callback: navigationpanel::DataInOutValueCallback<'a>,
    },
    DatapanelKey0 {
        control: navigation::DatapanelKey0,
        callback: navigation::DatapanelKey0ValueCallback<'a>,
    },
    DatapanelKey1 {
        control: navigation::DatapanelKey1,
        callback: navigation::DatapanelKey1ValueCallback<'a>,
    },
    DatapanelKey2 {
        control: navigation::DatapanelKey2,
        callback: navigation::DatapanelKey2ValueCallback<'a>,
    },
    DatapanelKey3 {
        control: navigation::DatapanelKey3,
        callback: navigation::DatapanelKey3ValueCallback<'a>,
    },
    DatapanelKey4 {
        control: navigation::DatapanelKey4,
        callback: navigation::DatapanelKey4ValueCallback<'a>,
    },
    DatapanelKey5 {
        control: navigation::DatapanelKey5,
        callback: navigation::DatapanelKey5ValueCallback<'a>,
    },
    DatapanelKey6 {
        control: navigation::DatapanelKey6,
        callback: navigation::DatapanelKey6ValueCallback<'a>,
    },
    DatapanelKey7 {
        control: navigation::DatapanelKey7,
        callback: navigation::DatapanelKey7ValueCallback<'a>,
    },
    DatapanelKey8 {
        control: navigation::DatapanelKey8,
        callback: navigation::DatapanelKey8ValueCallback<'a>,
    },
    DatapanelKey9 {
        control: navigation::DatapanelKey9,
        callback: navigation::DatapanelKey9ValueCallback<'a>,
    },
    DatapanelSelector {
        control: navigationpanel::DatapanelSelector,
        callback: navigationpanel::DatapanelSelectorValueCallback<'a>,
    },
    DeIce {
        control: enginepanel::DeIce,
        callback: IntegerCallback<'a>,
    },
    DistanceIndicator {
        control: rawgaugevalues::DistanceIndicator,
        callback: IntegerCallback<'a>,
    },
    DmeSelector {
        control: enginepanel::DmeSelector,
        callback: enginepanel::DmeSelectorValueCallback<'a>,
    },
    DopplerLandSeaMode {
        control: doppler::DopplerLandSeaMode,
        callback: IntegerCallback<'a>,
    },
    Drysuit {
        control: enginepanel::Drysuit,
        callback: IntegerCallback<'a>,
    },
    EjRev {
        control: errorpanel::EjRev,
        callback: LedCallback<'a>,
    },
    EjectionSeatArm {
        control: flightdataunit::EjectionSeatArm,
        callback: flightdataunit::EjectionSeatArmValueCallback<'a>,
    },
    Elfel {
        control: errorpanel::Elfel,
        callback: LedCallback<'a>,
    },
    EmergencyLights {
        control: lights::EmergencyLights,
        callback: lights::EmergencyLightsValueCallback<'a>,
    },
    EmergencyPitchTrim {
        control: flightdataunit::EmergencyPitchTrim,
        callback: flightdataunit::EmergencyPitchTrimValueCallback<'a>,
    },
    EmergencyRollTrim {
        control: flightdataunit::EmergencyRollTrim,
        callback: flightdataunit::EmergencyRollTrimValueCallback<'a>,
    },
    EmergencyYawTrim {
        control: flightdataunit::EmergencyYawTrim,
        callback: flightdataunit::EmergencyYawTrimValueCallback<'a>,
    },
    EngineDeice {
        control: enginepanel::EngineDeice,
        callback: enginepanel::EngineDeiceValueCallback<'a>,
    },
    EngineRpm10 {
        control: rawgaugevalues::EngineRpm10,
        callback: IntegerCallback<'a>,
    },
    EngineRpm100 {
        control: rawgaugevalues::EngineRpm100,
        callback: IntegerCallback<'a>,
    },
    EngineRpmValue {
        control: gaugevalues::EngineRpmValue,
        callback: IntegerCallback<'a>,
    },
    EngineTemp {
        control: rawgaugevalues::EngineTemp,
        callback: IntegerCallback<'a>,
    },
    EngineTempValue {
        control: gaugevalues::EngineTempValue,
        callback: IntegerCallback<'a>,
    },
    ExtFormationLights {
        control: externalaircraftmodel::ExtFormationLights,
        callback: IntegerCallback<'a>,
    },
    ExtNavLightsTail {
        control: externalaircraftmodel::ExtNavLightsTail,
        callback: IntegerCallback<'a>,
    },
    ExtNavLightsWing {
        control: externalaircraftmodel::ExtNavLightsWing,
        callback: IntegerCallback<'a>,
    },
    ExtPositionLights {
        control: externalaircraftmodel::ExtPositionLights,
        callback: IntegerCallback<'a>,
    },
    ExtSpeedBrakeLeft {
        control: externalaircraftmodel::ExtSpeedBrakeLeft,
        callback: IntegerCallback<'a>,
    },
    ExtSpeedBrakeRight {
        control: externalaircraftmodel::ExtSpeedBrakeRight,
        callback: IntegerCallback<'a>,
    },
    ExtStrobeLights {
        control: externalaircraftmodel::ExtStrobeLights,
        callback: IntegerCallback<'a>,
    },
    FacklSl {
        control: errorpanel::FacklSl,
        callback: LedCallback<'a>,
    },
    FlightRecorder {
        control: enginepanel::FlightRecorder,
        callback: enginepanel::FlightRecorderValueCallback<'a>,
    },
    FloodLights {
        control: lights::FloodLights,
        callback: IntegerCallback<'a>,
    },
    FormationLights {
        control: lights::FormationLights,
        callback: lights::FormationLightsValueCallback<'a>,
    },
    ForvForb {
        control: errorpanel::ForvForb,
        callback: LedCallback<'a>,
    },
    Fr22Base {
        control: fr22radio::Fr22Base,
        callback: fr22radio::Fr22BaseValueCallback<'a>,
    },
    Fr22ChannelAg {
        control: fr22radio::Fr22ChannelAg,
        callback: fr22radio::Fr22ChannelAgValueCallback<'a>,
    },
    Fr22ChannelB {
        control: fr22radio::Fr22ChannelB,
        callback: fr22radio::Fr22ChannelBValueCallback<'a>,
    },
    Fr22ChannelC2 {
        control: fr22radio::Fr22ChannelC2,
        callback: fr22radio::Fr22ChannelC2ValueCallback<'a>,
    },
    Fr22ChannelCf {
        control: fr22radio::Fr22ChannelCf,
        callback: fr22radio::Fr22ChannelCfValueCallback<'a>,
    },
    Fr22ChannelDe {
        control: fr22radio::Fr22ChannelDe,
        callback: fr22radio::Fr22ChannelDeValueCallback<'a>,
    },
    Fr22ChannelH {
        control: fr22radio::Fr22ChannelH,
        callback: fr22radio::Fr22ChannelHValueCallback<'a>,
    },
    Fr22Flight0 {
        control: fr22radio::Fr22Flight0,
        callback: fr22radio::Fr22Flight0ValueCallback<'a>,
    },
    Fr22Flight1 {
        control: fr22radio::Fr22Flight1,
        callback: fr22radio::Fr22Flight1ValueCallback<'a>,
    },
    Fr22Flight2 {
        control: fr22radio::Fr22Flight2,
        callback: fr22radio::Fr22Flight2ValueCallback<'a>,
    },
    Fr22Flight3 {
        control: fr22radio::Fr22Flight3,
        callback: fr22radio::Fr22Flight3ValueCallback<'a>,
    },
    Fr22Flight4 {
        control: fr22radio::Fr22Flight4,
        callback: fr22radio::Fr22Flight4ValueCallback<'a>,
    },
    Fr22Flight5 {
        control: fr22radio::Fr22Flight5,
        callback: fr22radio::Fr22Flight5ValueCallback<'a>,
    },
    Fr22Flight6 {
        control: fr22radio::Fr22Flight6,
        callback: fr22radio::Fr22Flight6ValueCallback<'a>,
    },
    Fr22Flight7 {
        control: fr22radio::Fr22Flight7,
        callback: fr22radio::Fr22Flight7ValueCallback<'a>,
    },
    Fr22Flight8 {
        control: fr22radio::Fr22Flight8,
        callback: fr22radio::Fr22Flight8ValueCallback<'a>,
    },
    Fr22Flight9 {
        control: fr22radio::Fr22Flight9,
        callback: fr22radio::Fr22Flight9ValueCallback<'a>,
    },
    Fr22GroundCom {
        control: fr22radio::Fr22GroundCom,
        callback: fr22radio::Fr22GroundComValueCallback<'a>,
    },
    Fr22Group {
        control: fr22radio::Fr22Group,
        callback: fr22radio::Fr22GroupValueCallback<'a>,
    },
    Fr22InnerLeftKnob {
        control: fr22radio::Fr22InnerLeftKnob,
        callback: IntegerCallback<'a>,
    },
    Fr22InnerRightKnob {
        control: fr22radio::Fr22InnerRightKnob,
        callback: IntegerCallback<'a>,
    },
    Fr22Minus {
        control: fr22radio::Fr22Minus,
        callback: fr22radio::Fr22MinusValueCallback<'a>,
    },
    Fr22Mode {
        control: fr22radio::Fr22Mode,
        callback: fr22radio::Fr22ModeValueCallback<'a>,
    },
    Fr22OuterLeftKnob {
        control: fr22radio::Fr22OuterLeftKnob,
        callback: IntegerCallback<'a>,
    },
    Fr22OuterRightKnob {
        control: fr22radio::Fr22OuterRightKnob,
        callback: IntegerCallback<'a>,
    },
    Fr22SetModulation {
        control: fr22radio::Fr22SetModulation,
        callback: IntegerCallback<'a>,
    },
    Fr22Special1 {
        control: fr22radio::Fr22Special1,
        callback: fr22radio::Fr22Special1ValueCallback<'a>,
    },
    Fr22Special2 {
        control: fr22radio::Fr22Special2,
        callback: fr22radio::Fr22Special2ValueCallback<'a>,
    },
    Fr22Special3 {
        control: fr22radio::Fr22Special3,
        callback: fr22radio::Fr22Special3ValueCallback<'a>,
    },
    Fr22Vol {
        control: fr22radio::Fr22Vol,
        callback: IntegerCallback<'a>,
    },
    FuelLevel {
        control: rawgaugevalues::FuelLevel,
        callback: IntegerCallback<'a>,
    },
    FuelLevelValue {
        control: gaugevalues::FuelLevelValue,
        callback: IntegerCallback<'a>,
    },
    FuelNeeded {
        control: rawgaugevalues::FuelNeeded,
        callback: IntegerCallback<'a>,
    },
    FuelNeededValue {
        control: gaugevalues::FuelNeededValue,
        callback: IntegerCallback<'a>,
    },
    GearHandle {
        control: flightdataunit::GearHandle,
        callback: flightdataunit::GearHandleValueCallback<'a>,
    },
    Generator {
        control: electricsystem::Generator,
        callback: electricsystem::GeneratorValueCallback<'a>,
    },
    HallFunk {
        control: errorpanel::HallFunk,
        callback: LedCallback<'a>,
    },
    HighPresFuelValve {
        control: enginepanel::HighPresFuelValve,
        callback: enginepanel::HighPresFuelValveValueCallback<'a>,
    },
    HojdCisi {
        control: flightdataunit::HojdCisi,
        callback: flightdataunit::HojdCisiValueCallback<'a>,
    },
    HojdLamp {
        control: indicatorlights::HojdLamp,
        callback: LedCallback<'a>,
    },
    Hstall {
        control: errorpanel::Hstall,
        callback: LedCallback<'a>,
    },
    HudBright {
        control: flightdataunit::HudBright,
        callback: IntegerCallback<'a>,
    },
    HudGlassPosition {
        control: flightdataunit::HudGlassPosition,
        callback: IntegerCallback<'a>,
    },
    HuvOStol {
        control: errorpanel::HuvOStol,
        callback: LedCallback<'a>,
    },
    HydraTa1 {
        control: errorpanel::HydraTa1,
        callback: LedCallback<'a>,
    },
    HydraTa2 {
        control: errorpanel::HydraTa2,
        callback: LedCallback<'a>,
    },
    IffChannel {
        control: enginepanel::IffChannel,
        callback: enginepanel::IffChannelValueCallback<'a>,
    },
    IffCode {
        control: radar::IffCode,
        callback: radar::IffCodeValueCallback<'a>,
    },
    IffPower {
        control: radar::IffPower,
        callback: radar::IffPowerValueCallback<'a>,
    },
    IffTransponderPower {
        control: enginepanel::IffTransponderPower,
        callback: enginepanel::IffTransponderPowerValueCallback<'a>,
    },
    IgnitionSystem {
        control: enginepanel::IgnitionSystem,
        callback: enginepanel::IgnitionSystemValueCallback<'a>,
    },
    IndicatorSystemTest {
        control: warningpanel::IndicatorSystemTest,
        callback: warningpanel::IndicatorSystemTestValueCallback<'a>,
    },
    InstrumentLights {
        control: lights::InstrumentLights,
        callback: IntegerCallback<'a>,
    },
    JammerBandSelector {
        control: countermeasures::JammerBandSelector,
        callback: countermeasures::JammerBandSelectorValueCallback<'a>,
    },
    JammerModeSelector {
        control: countermeasures::JammerModeSelector,
        callback: countermeasures::JammerModeSelectorValueCallback<'a>,
    },
    Kabinhojd {
        control: errorpanel::Kabinhojd,
        callback: LedCallback<'a>,
    },
    KbHKaSl {
        control: errorpanel::KbHKaSl,
        callback: LedCallback<'a>,
    },
    KbVSlut {
        control: errorpanel::KbVSlut,
        callback: LedCallback<'a>,
    },
    Landstall {
        control: errorpanel::Landstall,
        callback: LedCallback<'a>,
    },
    LowPresFuelValve {
        control: enginepanel::LowPresFuelValve,
        callback: enginepanel::LowPresFuelValveValueCallback<'a>,
    },
    Luftbroms {
        control: errorpanel::Luftbroms,
        callback: LedCallback<'a>,
    },
    MachMeterFirstDecimal {
        control: rawgaugevalues::MachMeterFirstDecimal,
        callback: IntegerCallback<'a>,
    },
    MachMeterInteger {
        control: rawgaugevalues::MachMeterInteger,
        callback: IntegerCallback<'a>,
    },
    MachMeterSecondDecimal {
        control: rawgaugevalues::MachMeterSecondDecimal,
        callback: IntegerCallback<'a>,
    },
    MagCorrection {
        control: flightdataunit::MagCorrection,
        callback: IntegerCallback<'a>,
    },
    MagDecCover {
        control: flightdataunit::MagDecCover,
        callback: flightdataunit::MagDecCoverValueCallback<'a>,
    },
    MagneticHeading {
        control: rawgaugevalues::MagneticHeading,
        callback: IntegerCallback<'a>,
    },
    MainElectricPower {
        control: electricsystem::MainElectricPower,
        callback: electricsystem::MainElectricPowerValueCallback<'a>,
    },
    ManAfterburnFuelReg {
        control: enginepanel::ManAfterburnFuelReg,
        callback: enginepanel::ManAfterburnFuelRegValueCallback<'a>,
    },
    ManBgReg {
        control: errorpanel::ManBgReg,
        callback: LedCallback<'a>,
    },
    ManualFuelReg {
        control: enginepanel::ManualFuelReg,
        callback: enginepanel::ManualFuelRegValueCallback<'a>,
    },
    MasterCautionReset {
        control: warningpanel::MasterCautionReset,
        callback: warningpanel::MasterCautionResetValueCallback<'a>,
    },
    MasterModeSelect {
        control: flightdataunit::MasterModeSelect,
        callback: flightdataunit::MasterModeSelectValueCallback<'a>,
    },
    MasterVol {
        control: rwr::MasterVol,
        callback: IntegerCallback<'a>,
    },
    MaxGReset {
        control: flightdataunit::MaxGReset,
        callback: flightdataunit::MaxGResetValueCallback<'a>,
    },
    MissileSelectButton {
        control: enginepanel::MissileSelectButton,
        callback: enginepanel::MissileSelectButtonValueCallback<'a>,
    },
    Motverk {
        control: errorpanel::Motverk,
        callback: LedCallback<'a>,
    },
    NavSelectBtnB1 {
        control: navigation::NavSelectBtnB1,
        callback: navigation::NavSelectBtnB1ValueCallback<'a>,
    },
    NavSelectBtnB2 {
        control: navigation::NavSelectBtnB2,
        callback: navigation::NavSelectBtnB2ValueCallback<'a>,
    },
    NavSelectBtnB3 {
        control: navigation::NavSelectBtnB3,
        callback: navigation::NavSelectBtnB3ValueCallback<'a>,
    },
    NavSelectBtnB4 {
        control: navigation::NavSelectBtnB4,
        callback: navigation::NavSelectBtnB4ValueCallback<'a>,
    },
    NavSelectBtnB5 {
        control: navigation::NavSelectBtnB5,
        callback: navigation::NavSelectBtnB5ValueCallback<'a>,
    },
    NavSelectBtnB6 {
        control: navigation::NavSelectBtnB6,
        callback: navigation::NavSelectBtnB6ValueCallback<'a>,
    },
    NavSelectBtnB7 {
        control: navigation::NavSelectBtnB7,
        callback: navigation::NavSelectBtnB7ValueCallback<'a>,
    },
    NavSelectBtnB8 {
        control: navigation::NavSelectBtnB8,
        callback: navigation::NavSelectBtnB8ValueCallback<'a>,
    },
    NavSelectBtnB9 {
        control: navigation::NavSelectBtnB9,
        callback: navigation::NavSelectBtnB9ValueCallback<'a>,
    },
    NavSelectBtnBx {
        control: navigation::NavSelectBtnBx,
        callback: navigation::NavSelectBtnBxValueCallback<'a>,
    },
    NavSelectBtnLMal {
        control: navigation::NavSelectBtnLMal,
        callback: navigation::NavSelectBtnLMalValueCallback<'a>,
    },
    NavSelectBtnLs {
        control: navigation::NavSelectBtnLs,
        callback: navigation::NavSelectBtnLsValueCallback<'a>,
    },
    NavSyst {
        control: errorpanel::NavSyst,
        callback: LedCallback<'a>,
    },
    NavigationLights {
        control: lights::NavigationLights,
        callback: lights::NavigationLightsValueCallback<'a>,
    },
    Nosstall {
        control: errorpanel::Nosstall,
        callback: LedCallback<'a>,
    },
    Oljetryck {
        control: errorpanel::Oljetryck,
        callback: LedCallback<'a>,
    },
    OxygenLever {
        control: flightdataunit::OxygenLever,
        callback: flightdataunit::OxygenLeverValueCallback<'a>,
    },
    PanelLights {
        control: lights::PanelLights,
        callback: IntegerCallback<'a>,
    },
    ParkingBrake {
        control: flightdataunit::ParkingBrake,
        callback: IntegerCallback<'a>,
    },
    Pedals {
        control: rawgaugevalues::Pedals,
        callback: IntegerCallback<'a>,
    },
    PitchGearMode {
        control: flightdataunit::PitchGearMode,
        callback: flightdataunit::PitchGearModeValueCallback<'a>,
    },
    PositionLights {
        control: lights::PositionLights,
        callback: lights::PositionLightsValueCallback<'a>,
    },
    PositionLightsBrightness {
        control: lights::PositionLightsBrightness,
        callback: lights::PositionLightsBrightnessValueCallback<'a>,
    },
    RadarAltimeterPower {
        control: radaraltimeter::RadarAltimeterPower,
        callback: radaraltimeter::RadarAltimeterPowerValueCallback<'a>,
    },
    RadarBright {
        control: radar::RadarBright,
        callback: IntegerCallback<'a>,
    },
    RadarGain {
        control: radar::RadarGain,
        callback: radar::RadarGainValueCallback<'a>,
    },
    RadarIffId {
        control: radar::RadarIffId,
        callback: radar::RadarIffIdValueCallback<'a>,
    },
    RadarIgnitionCoils {
        control: radar::RadarIgnitionCoils,
        callback: radar::RadarIgnitionCoilsValueCallback<'a>,
    },
    RadarMaintenanceTest {
        control: radar::RadarMaintenanceTest,
        callback: radar::RadarMaintenanceTestValueCallback<'a>,
    },
    RadarPulseNormalShort {
        control: radar::RadarPulseNormalShort,
        callback: radar::RadarPulseNormalShortValueCallback<'a>,
    },
    RadarRecceOnOff {
        control: radar::RadarRecceOnOff,
        callback: radar::RadarRecceOnOffValueCallback<'a>,
    },
    RadarWarnSelect {
        control: rwr::RadarWarnSelect,
        callback: rwr::RadarWarnSelectValueCallback<'a>,
    },
    RbBkRelMode {
        control: weaponsystem::RbBkRelMode,
        callback: weaponsystem::RbBkRelModeValueCallback<'a>,
    },
    RensaButtonCover {
        control: navigationpanel::RensaButtonCover,
        callback: navigationpanel::RensaButtonCoverValueCallback<'a>,
    },
    Reserveff {
        control: errorpanel::Reserveff,
        callback: LedCallback<'a>,
    },
    Restart {
        control: enginepanel::Restart,
        callback: enginepanel::RestartValueCallback<'a>,
    },
    Reversal {
        control: thrustreverser::Reversal,
        callback: thrustreverser::ReversalValueCallback<'a>,
    },
    RhmFel {
        control: errorpanel::RhmFel,
        callback: LedCallback<'a>,
    },
    RollCenter {
        control: flightdataunit::RollCenter,
        callback: flightdataunit::RollCenterValueCallback<'a>,
    },
    RollVaxel {
        control: errorpanel::RollVaxel,
        callback: LedCallback<'a>,
    },
    SlavSi {
        control: flightdataunit::SlavSi,
        callback: flightdataunit::SlavSiValueCallback<'a>,
    },
    Spak {
        control: flightdataunit::Spak,
        callback: flightdataunit::SpakValueCallback<'a>,
    },
    SpakError {
        control: errorpanel::SpakError,
        callback: LedCallback<'a>,
    },
    SpakLamp {
        control: indicatorlights::SpakLamp,
        callback: LedCallback<'a>,
    },
    StartSystem {
        control: enginepanel::StartSystem,
        callback: enginepanel::StartSystemValueCallback<'a>,
    },
    Startsyst {
        control: errorpanel::Startsyst,
        callback: LedCallback<'a>,
    },
    StickPitch {
        control: rawgaugevalues::StickPitch,
        callback: IntegerCallback<'a>,
    },
    StickRoll {
        control: rawgaugevalues::StickRoll,
        callback: IntegerCallback<'a>,
    },
    Syrgas {
        control: errorpanel::Syrgas,
        callback: LedCallback<'a>,
    },
    Tandsyst {
        control: errorpanel::Tandsyst,
        callback: LedCallback<'a>,
    },
    TankPump {
        control: errorpanel::TankPump,
        callback: LedCallback<'a>,
    },
    TankRelease {
        control: weaponsystem::TankRelease,
        callback: weaponsystem::TankReleaseValueCallback<'a>,
    },
    TankReleaseCover {
        control: weaponsystem::TankReleaseCover,
        callback: weaponsystem::TankReleaseCoverValueCallback<'a>,
    },
    TaxiLandingLights {
        control: lights::TaxiLandingLights,
        callback: lights::TaxiLandingLightsValueCallback<'a>,
    },
    TestMode {
        control: enginepanel::TestMode,
        callback: IntegerCallback<'a>,
    },
    Throttle {
        control: rawgaugevalues::Throttle,
        callback: IntegerCallback<'a>,
    },
    Tils {
        control: errorpanel::Tils,
        callback: LedCallback<'a>,
    },
    TilsChannelLayer {
        control: flightdataunit::TilsChannelLayer,
        callback: flightdataunit::TilsChannelLayerValueCallback<'a>,
    },
    TilsChannelSelect {
        control: flightdataunit::TilsChannelSelect,
        callback: flightdataunit::TilsChannelSelectValueCallback<'a>,
    },
    Tippvaxel {
        control: errorpanel::Tippvaxel,
        callback: LedCallback<'a>,
    },
    TriggerSafetyBracket {
        control: weaponsystem::TriggerSafetyBracket,
        callback: weaponsystem::TriggerSafetyBracketValueCallback<'a>,
    },
    VerticalAcceleration {
        control: rawgaugevalues::VerticalAcceleration,
        callback: IntegerCallback<'a>,
    },
    Vstall {
        control: errorpanel::Vstall,
        callback: LedCallback<'a>,
    },
    WarningPanelTest {
        control: warningpanel::WarningPanelTest,
        callback: warningpanel::WarningPanelTestValueCallback<'a>,
    },
    WeaponInterval {
        control: weaponsystem::WeaponInterval,
        callback: weaponsystem::WeaponIntervalValueCallback<'a>,
    },
    WeaponRelMode {
        control: weaponsystem::WeaponRelMode,
        callback: weaponsystem::WeaponRelModeValueCallback<'a>,
    },
    WeaponRelease {
        control: weaponsystem::WeaponRelease,
        callback: weaponsystem::WeaponReleaseValueCallback<'a>,
    },
    WeaponReleaseCover {
        control: weaponsystem::WeaponReleaseCover,
        callback: weaponsystem::WeaponReleaseCoverValueCallback<'a>,
    },
    WeaponSelect {
        control: weaponsystem::WeaponSelect,
        callback: weaponsystem::WeaponSelectValueCallback<'a>,
    },
    XTankBra {
        control: errorpanel::XTankBra,
        callback: LedCallback<'a>,
    },
    YawTrimCover {
        control: flightdataunit::YawTrimCover,
        callback: flightdataunit::YawTrimCoverValueCallback<'a>,
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
                    if let Ok(value) =
                        <enginepanel::AfkLeverValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
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
                    if let Ok(value) =
                        <flightdataunit::AltitudeHoldValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AltitudeValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AntiCollisionLights { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <lights::AntiCollisionLightsValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AntiJamMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <radar::AntiJamModeValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AttLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::AttitudeHold { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <flightdataunit::AttitudeHoldValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AutoYawTrim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BackAdiCage { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <flightdataunit::BackAdiCageValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
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
                    if let Ok(value) = <electricsystem::BackupGeneratorValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
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
                    if let Ok(value) = <enginepanel::CabinAirValveValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CanopyJettison { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flightdataunit::CanopyJettisonValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CanopyOpenClose { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flightdataunit::CanopyOpenCloseValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CbAutopilot { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <enginepanel::CbAutopilotValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbCiSi { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <enginepanel::CbCiSiValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbEjectionSystem { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <enginepanel::CbEjectionSystemValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CbEngine { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <enginepanel::CbEngineValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbHighAlphaWarn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <enginepanel::CbHighAlphaWarnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbTrimSystem { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <enginepanel::CbTrimSystemValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
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
                    if let Ok(value) = <navigationpanel::Ck37RensaClearValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::ClockSet { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CountermeasureChaffFlaresSelector { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <countermeasures::CountermeasureChaffFlaresSelectorValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CountermeasureFastRelease { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <flightdataunit::CountermeasureFastReleaseValue as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CountermeasureModeSelector { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <countermeasures::CountermeasureModeSelectorValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CountermeasureReleaseMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <countermeasures::CountermeasureReleaseModeValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CountermeasureStreakModeSelector { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <countermeasures::CountermeasureStreakModeSelectorValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::DataCartridge { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <enginepanel::DataCartridgeValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::DataInOut { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigationpanel::DataInOutValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::DatapanelKey0 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigation::DatapanelKey0Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::DatapanelKey1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigation::DatapanelKey1Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::DatapanelKey2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigation::DatapanelKey2Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::DatapanelKey3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigation::DatapanelKey3Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::DatapanelKey4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigation::DatapanelKey4Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::DatapanelKey5 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigation::DatapanelKey5Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::DatapanelKey6 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigation::DatapanelKey6Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::DatapanelKey7 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigation::DatapanelKey7Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::DatapanelKey8 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigation::DatapanelKey8Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::DatapanelKey9 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigation::DatapanelKey9Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::DatapanelSelector { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigationpanel::DatapanelSelectorValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
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
                    if let Ok(value) = <enginepanel::DmeSelectorValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
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
                    if let Ok(value) = <flightdataunit::EjectionSeatArmValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::Elfel { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::EmergencyLights { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <lights::EmergencyLightsValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EmergencyPitchTrim { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flightdataunit::EmergencyPitchTrimValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::EmergencyRollTrim { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flightdataunit::EmergencyRollTrimValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::EmergencyYawTrim { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flightdataunit::EmergencyYawTrimValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::EngineDeice { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <enginepanel::EngineDeiceValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
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
                    if let Ok(value) =
                        <enginepanel::FlightRecorderValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::FloodLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FormationLights { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <lights::FormationLightsValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ForvForb { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Fr22Base { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <fr22radio::Fr22BaseValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Fr22ChannelAg { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fr22radio::Fr22ChannelAgValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Fr22ChannelB { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fr22radio::Fr22ChannelBValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Fr22ChannelC2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fr22radio::Fr22ChannelC2Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Fr22ChannelCf { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fr22radio::Fr22ChannelCfValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Fr22ChannelDe { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fr22radio::Fr22ChannelDeValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Fr22ChannelH { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fr22radio::Fr22ChannelHValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Fr22Flight0 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fr22radio::Fr22Flight0Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Fr22Flight1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fr22radio::Fr22Flight1Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Fr22Flight2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fr22radio::Fr22Flight2Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Fr22Flight3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fr22radio::Fr22Flight3Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Fr22Flight4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fr22radio::Fr22Flight4Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Fr22Flight5 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fr22radio::Fr22Flight5Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Fr22Flight6 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fr22radio::Fr22Flight6Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Fr22Flight7 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fr22radio::Fr22Flight7Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Fr22Flight8 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fr22radio::Fr22Flight8Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Fr22Flight9 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fr22radio::Fr22Flight9Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Fr22GroundCom { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fr22radio::Fr22GroundComValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Fr22Group { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <fr22radio::Fr22GroupValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
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
                    if let Ok(value) =
                        <fr22radio::Fr22MinusValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Fr22Mode { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <fr22radio::Fr22ModeValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
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
                    if let Ok(value) = <fr22radio::Fr22Special1Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Fr22Special2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fr22radio::Fr22Special2Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Fr22Special3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fr22radio::Fr22Special3Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
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
                    if let Ok(value) = <flightdataunit::GearHandleValue as core::convert::TryFrom<
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
            Self::HallFunk { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::HighPresFuelValve { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <enginepanel::HighPresFuelValveValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::HojdCisi { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flightdataunit::HojdCisiValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
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
                    if let Ok(value) = <enginepanel::IffChannelValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IffCode { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <radar::IffCodeValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffPower { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <radar::IffPowerValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffTransponderPower { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <enginepanel::IffTransponderPowerValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::IgnitionSystem { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <enginepanel::IgnitionSystemValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IndicatorSystemTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <warningpanel::IndicatorSystemTestValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::InstrumentLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::JammerBandSelector { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <countermeasures::JammerBandSelectorValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::JammerModeSelector { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <countermeasures::JammerModeSelectorValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
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
                    if let Ok(value) = <enginepanel::LowPresFuelValveValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
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
                    if let Ok(value) =
                        <flightdataunit::MagDecCoverValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MagneticHeading { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MainElectricPower { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricsystem::MainElectricPowerValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::ManAfterburnFuelReg { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <enginepanel::ManAfterburnFuelRegValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::ManBgReg { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ManualFuelReg { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <enginepanel::ManualFuelRegValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MasterCautionReset { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <warningpanel::MasterCautionResetValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::MasterModeSelect { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flightdataunit::MasterModeSelectValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::MasterVol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MaxGReset { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flightdataunit::MaxGResetValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MissileSelectButton { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <enginepanel::MissileSelectButtonValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::Motverk { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::NavSelectBtnB1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigation::NavSelectBtnB1Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::NavSelectBtnB2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigation::NavSelectBtnB2Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::NavSelectBtnB3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigation::NavSelectBtnB3Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::NavSelectBtnB4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigation::NavSelectBtnB4Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::NavSelectBtnB5 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigation::NavSelectBtnB5Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::NavSelectBtnB6 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigation::NavSelectBtnB6Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::NavSelectBtnB7 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigation::NavSelectBtnB7Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::NavSelectBtnB8 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigation::NavSelectBtnB8Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::NavSelectBtnB9 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigation::NavSelectBtnB9Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::NavSelectBtnBx { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigation::NavSelectBtnBxValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::NavSelectBtnLMal { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <navigation::NavSelectBtnLMalValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::NavSelectBtnLs { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigation::NavSelectBtnLsValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::NavSyst { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::NavigationLights { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <lights::NavigationLightsValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
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
                    if let Ok(value) =
                        <flightdataunit::OxygenLeverValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
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
                    if let Ok(value) = <flightdataunit::PitchGearModeValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::PositionLights { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <lights::PositionLightsValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::PositionLightsBrightness { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <lights::PositionLightsBrightnessValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::RadarAltimeterPower { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <radaraltimeter::RadarAltimeterPowerValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::RadarBright { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarGain { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <radar::RadarGainValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RadarIffId { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <radar::RadarIffIdValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RadarIgnitionCoils { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <radar::RadarIgnitionCoilsValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RadarMaintenanceTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <radar::RadarMaintenanceTestValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RadarPulseNormalShort { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <radar::RadarPulseNormalShortValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RadarRecceOnOff { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <radar::RadarRecceOnOffValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RadarWarnSelect { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rwr::RadarWarnSelectValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RbBkRelMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weaponsystem::RbBkRelModeValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RensaButtonCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navigationpanel::RensaButtonCoverValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::Reserveff { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Restart { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <enginepanel::RestartValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Reversal { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <thrustreverser::ReversalValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RhmFel { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RollCenter { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flightdataunit::RollCenterValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RollVaxel { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::SlavSi { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flightdataunit::SlavSiValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Spak { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <flightdataunit::SpakValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
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
                    if let Ok(value) = <enginepanel::StartSystemValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
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
                    if let Ok(value) = <weaponsystem::TankReleaseValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::TankReleaseCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weaponsystem::TankReleaseCoverValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::TaxiLandingLights { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <lights::TaxiLandingLightsValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
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
                    if let Ok(value) = <flightdataunit::TilsChannelLayerValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::TilsChannelSelect { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flightdataunit::TilsChannelSelectValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::Tippvaxel { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::TriggerSafetyBracket { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weaponsystem::TriggerSafetyBracketValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
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
                    if let Ok(value) = <warningpanel::WarningPanelTestValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::WeaponInterval { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <weaponsystem::WeaponIntervalValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::WeaponRelMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <weaponsystem::WeaponRelModeValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::WeaponRelease { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <weaponsystem::WeaponReleaseValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::WeaponReleaseCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weaponsystem::WeaponReleaseCoverValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::WeaponSelect { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weaponsystem::WeaponSelectValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::XTankBra { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::YawTrimCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <flightdataunit::YawTrimCoverValue as core::convert::TryFrom<u16>>::try_from(
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
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiAoaIndicator {
                control: rawgaugevalues::AdiAoaIndicator::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_heading<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiHeading {
                control: rawgaugevalues::AdiHeading::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_horizontal_ils<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiHorizontalIls {
                control: rawgaugevalues::AdiHorizontalIls::new(),
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
                control: rawgaugevalues::AdiPitch::new(),
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
                control: rawgaugevalues::AdiRoll::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_slipball<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiSlipball {
                control: rawgaugevalues::AdiSlipball::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_vertical_ils<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiVerticalIls {
                control: rawgaugevalues::AdiVerticalIls::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_vertical_velocity<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiVerticalVelocity {
                control: rawgaugevalues::AdiVerticalVelocity::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_afk15_deg_mode<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Afk15DegMode {
                control: flightdataunit::Afk15DegMode::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_afk_fel<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AfkFel {
                control: errorpanel::AfkFel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_afk_lever<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::AfkLeverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AfkLever {
                control: enginepanel::AfkLever::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_afk_mode3<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AfkMode3 {
                control: flightdataunit::AfkMode3::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_airspeed_m_s<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AirspeedMS {
                control: rawgaugevalues::AirspeedMS::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_airspeed_value<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AirspeedValue {
                control: gaugevalues::AirspeedValue::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ajs37_dest_indicator_data1<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ajs37DestIndicatorData1 {
                control: navigationpanel::Ajs37DestIndicatorData1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ajs37_dest_indicator_data2<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ajs37DestIndicatorData2 {
                control: navigationpanel::Ajs37DestIndicatorData2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ajs37_nav_indicator_data1<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ajs37NavIndicatorData1 {
                control: navigationpanel::Ajs37NavIndicatorData1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ajs37_nav_indicator_data2<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ajs37NavIndicatorData2 {
                control: navigationpanel::Ajs37NavIndicatorData2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ajs37_nav_indicator_data3<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ajs37NavIndicatorData3 {
                control: navigationpanel::Ajs37NavIndicatorData3::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ajs37_nav_indicator_data4<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ajs37NavIndicatorData4 {
                control: navigationpanel::Ajs37NavIndicatorData4::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ajs37_nav_indicator_data5<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ajs37NavIndicatorData5 {
                control: navigationpanel::Ajs37NavIndicatorData5::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ajs37_nav_indicator_data6<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ajs37NavIndicatorData6 {
                control: navigationpanel::Ajs37NavIndicatorData6::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_set<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltSet {
                control: flightdataunit::AltSet::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter10000m<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Altimeter10000m {
                control: rawgaugevalues::Altimeter10000m::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter1000m<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Altimeter1000m {
                control: rawgaugevalues::Altimeter1000m::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter_baro1000_hpa<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltimeterBaro1000Hpa {
                control: rawgaugevalues::AltimeterBaro1000Hpa::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter_baro100_hpa<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltimeterBaro100Hpa {
                control: rawgaugevalues::AltimeterBaro100Hpa::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter_baro10_hpa<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltimeterBaro10Hpa {
                control: rawgaugevalues::AltimeterBaro10Hpa::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altimeter_baro1_hpa<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltimeterBaro1Hpa {
                control: rawgaugevalues::AltimeterBaro1Hpa::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altitude_hold<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::AltitudeHoldValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltitudeHold {
                control: flightdataunit::AltitudeHold::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_altitude_value<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltitudeValue {
                control: gaugevalues::AltitudeValue::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_anti_collision_lights<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(lights::AntiCollisionLightsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AntiCollisionLights {
                control: lights::AntiCollisionLights::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_anti_jam_mode<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(radar::AntiJamModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AntiJamMode {
                control: radar::AntiJamMode::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_att_lamp<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AttLamp {
                control: indicatorlights::AttLamp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_attitude_hold<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::AttitudeHoldValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AttitudeHold {
                control: flightdataunit::AttitudeHold::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_auto_yaw_trim<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AutoYawTrim {
                control: flightdataunit::AutoYawTrim::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_back_adi_cage<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::BackAdiCageValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BackAdiCage {
                control: flightdataunit::BackAdiCage::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_backup_alt_setting<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BackupAltSetting {
                control: flightdataunit::BackupAltSetting::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_backup_altimeter10000m<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BackupAltimeter10000m {
                control: rawgaugevalues::BackupAltimeter10000m::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_backup_altimeter1000m<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BackupAltimeter1000m {
                control: rawgaugevalues::BackupAltimeter1000m::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_backup_generator<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::BackupGeneratorValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BackupGenerator {
                control: electricsystem::BackupGenerator::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_backup_indicated_airspeed<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BackupIndicatedAirspeed {
                control: rawgaugevalues::BackupIndicatedAirspeed::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_backup_pitch<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BackupPitch {
                control: rawgaugevalues::BackupPitch::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_backup_roll<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BackupRoll {
                control: rawgaugevalues::BackupRoll::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bra24<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Bra24 {
                control: errorpanel::Bra24::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bra_uppf<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::BraUppf {
                control: errorpanel::BraUppf::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brand1<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Brand1 {
                control: errorpanel::Brand1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brand2<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Brand2 {
                control: errorpanel::Brand2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brand_gts<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::BrandGts {
                control: errorpanel::BrandGts::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cabin_air_valve<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::CabinAirValveValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CabinAirValve {
                control: enginepanel::CabinAirValve::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_jettison<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::CanopyJettisonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyJettison {
                control: flightdataunit::CanopyJettison::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_open_close<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::CanopyOpenCloseValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyOpenClose {
                control: flightdataunit::CanopyOpenClose::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_autopilot<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::CbAutopilotValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbAutopilot {
                control: enginepanel::CbAutopilot::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_ci_si<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::CbCiSiValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbCiSi {
                control: enginepanel::CbCiSi::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_ejection_system<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::CbEjectionSystemValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbEjectionSystem {
                control: enginepanel::CbEjectionSystem::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_engine<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::CbEngineValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbEngine {
                control: enginepanel::CbEngine::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_high_alpha_warn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::CbHighAlphaWarnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbHighAlphaWarn {
                control: enginepanel::CbHighAlphaWarn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_trim_system<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::CbTrimSystemValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbTrimSystem {
                control: enginepanel::CbTrimSystem::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_command_heading_value<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiCommandHeadingValue {
                control: gaugevalues::CiCommandHeadingValue::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_commanded_heading<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiCommandedHeading {
                control: rawgaugevalues::CiCommandedHeading::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_heading<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiHeading {
                control: rawgaugevalues::CiHeading::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_heading_value<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiHeadingValue {
                control: gaugevalues::CiHeadingValue::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ck<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ck {
                control: errorpanel::Ck::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ck37_rensa_clear<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigationpanel::Ck37RensaClearValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ck37RensaClear {
                control: navigationpanel::Ck37RensaClear::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_set<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockSet {
                control: flightdataunit::ClockSet::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_countermeasure_chaff_flares_selector<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(countermeasures::CountermeasureChaffFlaresSelectorValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CountermeasureChaffFlaresSelector {
                control: countermeasures::CountermeasureChaffFlaresSelector::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_countermeasure_fast_release<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::CountermeasureFastReleaseValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CountermeasureFastRelease {
                control: flightdataunit::CountermeasureFastRelease::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_countermeasure_mode_selector<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(countermeasures::CountermeasureModeSelectorValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CountermeasureModeSelector {
                control: countermeasures::CountermeasureModeSelector::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_countermeasure_release_mode<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(countermeasures::CountermeasureReleaseModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CountermeasureReleaseMode {
                control: countermeasures::CountermeasureReleaseMode::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_countermeasure_streak_mode_selector<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(countermeasures::CountermeasureStreakModeSelectorValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CountermeasureStreakModeSelector {
                control: countermeasures::CountermeasureStreakModeSelector::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_data_cartridge<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::DataCartridgeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DataCartridge {
                control: enginepanel::DataCartridge::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_data_in_out<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigationpanel::DataInOutValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DataInOut {
                control: navigationpanel::DataInOut::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_key0<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::DatapanelKey0Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelKey0 {
                control: navigation::DatapanelKey0::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_key1<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::DatapanelKey1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelKey1 {
                control: navigation::DatapanelKey1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_key2<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::DatapanelKey2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelKey2 {
                control: navigation::DatapanelKey2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_key3<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::DatapanelKey3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelKey3 {
                control: navigation::DatapanelKey3::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_key4<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::DatapanelKey4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelKey4 {
                control: navigation::DatapanelKey4::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_key5<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::DatapanelKey5Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelKey5 {
                control: navigation::DatapanelKey5::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_key6<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::DatapanelKey6Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelKey6 {
                control: navigation::DatapanelKey6::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_key7<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::DatapanelKey7Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelKey7 {
                control: navigation::DatapanelKey7::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_key8<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::DatapanelKey8Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelKey8 {
                control: navigation::DatapanelKey8::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_key9<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::DatapanelKey9Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelKey9 {
                control: navigation::DatapanelKey9::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_datapanel_selector<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigationpanel::DatapanelSelectorValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DatapanelSelector {
                control: navigationpanel::DatapanelSelector::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_de_ice<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DeIce {
                control: enginepanel::DeIce::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_distance_indicator<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DistanceIndicator {
                control: rawgaugevalues::DistanceIndicator::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dme_selector<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::DmeSelectorValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DmeSelector {
                control: enginepanel::DmeSelector::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_doppler_land_sea_mode<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DopplerLandSeaMode {
                control: doppler::DopplerLandSeaMode::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_drysuit<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Drysuit {
                control: enginepanel::Drysuit::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ej_rev<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::EjRev {
                control: errorpanel::EjRev::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ejection_seat_arm<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::EjectionSeatArmValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EjectionSeatArm {
                control: flightdataunit::EjectionSeatArm::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_elfel<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Elfel {
                control: errorpanel::Elfel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emergency_lights<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(lights::EmergencyLightsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmergencyLights {
                control: lights::EmergencyLights::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emergency_pitch_trim<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::EmergencyPitchTrimValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmergencyPitchTrim {
                control: flightdataunit::EmergencyPitchTrim::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emergency_roll_trim<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::EmergencyRollTrimValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmergencyRollTrim {
                control: flightdataunit::EmergencyRollTrim::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emergency_yaw_trim<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::EmergencyYawTrimValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmergencyYawTrim {
                control: flightdataunit::EmergencyYawTrim::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_deice<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::EngineDeiceValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineDeice {
                control: enginepanel::EngineDeice::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_rpm10<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineRpm10 {
                control: rawgaugevalues::EngineRpm10::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_rpm100<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineRpm100 {
                control: rawgaugevalues::EngineRpm100::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_rpm_value<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineRpmValue {
                control: gaugevalues::EngineRpmValue::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_temp<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineTemp {
                control: rawgaugevalues::EngineTemp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_temp_value<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineTempValue {
                control: gaugevalues::EngineTempValue::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_formation_lights<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtFormationLights {
                control: externalaircraftmodel::ExtFormationLights::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_nav_lights_tail<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtNavLightsTail {
                control: externalaircraftmodel::ExtNavLightsTail::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_nav_lights_wing<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtNavLightsWing {
                control: externalaircraftmodel::ExtNavLightsWing::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_position_lights<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtPositionLights {
                control: externalaircraftmodel::ExtPositionLights::new(),
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
                control: externalaircraftmodel::ExtSpeedBrakeLeft::new(),
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
                control: externalaircraftmodel::ExtSpeedBrakeRight::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_strobe_lights<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtStrobeLights {
                control: externalaircraftmodel::ExtStrobeLights::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fackl_sl<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FacklSl {
                control: errorpanel::FacklSl::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flight_recorder<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::FlightRecorderValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlightRecorder {
                control: enginepanel::FlightRecorder::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flood_lights<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FloodLights {
                control: lights::FloodLights::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_formation_lights<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(lights::FormationLightsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FormationLights {
                control: lights::FormationLights::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_forv_forb<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ForvForb {
                control: errorpanel::ForvForb::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_base<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22BaseValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Base {
                control: fr22radio::Fr22Base::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_channel_ag<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22ChannelAgValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22ChannelAg {
                control: fr22radio::Fr22ChannelAg::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_channel_b<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22ChannelBValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22ChannelB {
                control: fr22radio::Fr22ChannelB::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_channel_c2<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22ChannelC2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22ChannelC2 {
                control: fr22radio::Fr22ChannelC2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_channel_cf<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22ChannelCfValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22ChannelCf {
                control: fr22radio::Fr22ChannelCf::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_channel_de<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22ChannelDeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22ChannelDe {
                control: fr22radio::Fr22ChannelDe::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_channel_h<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22ChannelHValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22ChannelH {
                control: fr22radio::Fr22ChannelH::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_flight0<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22Flight0Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Flight0 {
                control: fr22radio::Fr22Flight0::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_flight1<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22Flight1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Flight1 {
                control: fr22radio::Fr22Flight1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_flight2<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22Flight2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Flight2 {
                control: fr22radio::Fr22Flight2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_flight3<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22Flight3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Flight3 {
                control: fr22radio::Fr22Flight3::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_flight4<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22Flight4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Flight4 {
                control: fr22radio::Fr22Flight4::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_flight5<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22Flight5Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Flight5 {
                control: fr22radio::Fr22Flight5::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_flight6<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22Flight6Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Flight6 {
                control: fr22radio::Fr22Flight6::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_flight7<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22Flight7Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Flight7 {
                control: fr22radio::Fr22Flight7::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_flight8<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22Flight8Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Flight8 {
                control: fr22radio::Fr22Flight8::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_flight9<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22Flight9Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Flight9 {
                control: fr22radio::Fr22Flight9::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_ground_com<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22GroundComValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22GroundCom {
                control: fr22radio::Fr22GroundCom::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_group<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22GroupValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Group {
                control: fr22radio::Fr22Group::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_inner_left_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22InnerLeftKnob {
                control: fr22radio::Fr22InnerLeftKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_inner_right_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22InnerRightKnob {
                control: fr22radio::Fr22InnerRightKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_minus<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22MinusValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Minus {
                control: fr22radio::Fr22Minus::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_mode<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22ModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Mode {
                control: fr22radio::Fr22Mode::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_outer_left_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22OuterLeftKnob {
                control: fr22radio::Fr22OuterLeftKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_outer_right_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22OuterRightKnob {
                control: fr22radio::Fr22OuterRightKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_set_modulation<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22SetModulation {
                control: fr22radio::Fr22SetModulation::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_special1<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22Special1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Special1 {
                control: fr22radio::Fr22Special1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_special2<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22Special2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Special2 {
                control: fr22radio::Fr22Special2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_special3<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fr22radio::Fr22Special3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Special3 {
                control: fr22radio::Fr22Special3::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fr22_vol<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fr22Vol {
                control: fr22radio::Fr22Vol::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_level<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelLevel {
                control: rawgaugevalues::FuelLevel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_level_value<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelLevelValue {
                control: gaugevalues::FuelLevelValue::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_needed<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelNeeded {
                control: rawgaugevalues::FuelNeeded::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_needed_value<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelNeededValue {
                control: gaugevalues::FuelNeededValue::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gear_handle<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::GearHandleValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GearHandle {
                control: flightdataunit::GearHandle::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_generator<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::GeneratorValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Generator {
                control: electricsystem::Generator::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hall_funk<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::HallFunk {
                control: errorpanel::HallFunk::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_high_pres_fuel_valve<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::HighPresFuelValveValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HighPresFuelValve {
                control: enginepanel::HighPresFuelValve::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hojd_cisi<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::HojdCisiValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HojdCisi {
                control: flightdataunit::HojdCisi::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hojd_lamp<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::HojdLamp {
                control: indicatorlights::HojdLamp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hstall<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Hstall {
                control: errorpanel::Hstall::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_bright<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudBright {
                control: flightdataunit::HudBright::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_glass_position<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudGlassPosition {
                control: flightdataunit::HudGlassPosition::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_huv_o_stol<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::HuvOStol {
                control: errorpanel::HuvOStol::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hydra_ta1<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydraTa1 {
                control: errorpanel::HydraTa1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hydra_ta2<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydraTa2 {
                control: errorpanel::HydraTa2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_channel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::IffChannelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffChannel {
                control: enginepanel::IffChannel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_code<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(radar::IffCodeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffCode {
                control: radar::IffCode::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_power<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(radar::IffPowerValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffPower {
                control: radar::IffPower::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_transponder_power<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::IffTransponderPowerValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffTransponderPower {
                control: enginepanel::IffTransponderPower::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ignition_system<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::IgnitionSystemValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IgnitionSystem {
                control: enginepanel::IgnitionSystem::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_indicator_system_test<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(warningpanel::IndicatorSystemTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IndicatorSystemTest {
                control: warningpanel::IndicatorSystemTest::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_instrument_lights<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::InstrumentLights {
                control: lights::InstrumentLights::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jammer_band_selector<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(countermeasures::JammerBandSelectorValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::JammerBandSelector {
                control: countermeasures::JammerBandSelector::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jammer_mode_selector<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(countermeasures::JammerModeSelectorValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::JammerModeSelector {
                control: countermeasures::JammerModeSelector::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_kabinhojd<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Kabinhojd {
                control: errorpanel::Kabinhojd::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_kb_h_ka_sl<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::KbHKaSl {
                control: errorpanel::KbHKaSl::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_kb_v_slut<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::KbVSlut {
                control: errorpanel::KbVSlut::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_landstall<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Landstall {
                control: errorpanel::Landstall::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_low_pres_fuel_valve<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::LowPresFuelValveValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LowPresFuelValve {
                control: enginepanel::LowPresFuelValve::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_luftbroms<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Luftbroms {
                control: errorpanel::Luftbroms::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mach_meter_first_decimal<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MachMeterFirstDecimal {
                control: rawgaugevalues::MachMeterFirstDecimal::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mach_meter_integer<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MachMeterInteger {
                control: rawgaugevalues::MachMeterInteger::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mach_meter_second_decimal<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MachMeterSecondDecimal {
                control: rawgaugevalues::MachMeterSecondDecimal::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mag_correction<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MagCorrection {
                control: flightdataunit::MagCorrection::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mag_dec_cover<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::MagDecCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MagDecCover {
                control: flightdataunit::MagDecCover::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_magnetic_heading<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MagneticHeading {
                control: rawgaugevalues::MagneticHeading::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_main_electric_power<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::MainElectricPowerValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MainElectricPower {
                control: electricsystem::MainElectricPower::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_man_afterburn_fuel_reg<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::ManAfterburnFuelRegValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ManAfterburnFuelReg {
                control: enginepanel::ManAfterburnFuelReg::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_man_bg_reg<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ManBgReg {
                control: errorpanel::ManBgReg::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_manual_fuel_reg<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::ManualFuelRegValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ManualFuelReg {
                control: enginepanel::ManualFuelReg::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_caution_reset<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(warningpanel::MasterCautionResetValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterCautionReset {
                control: warningpanel::MasterCautionReset::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_mode_select<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::MasterModeSelectValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterModeSelect {
                control: flightdataunit::MasterModeSelect::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_vol<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterVol {
                control: rwr::MasterVol::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_max_g_reset<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::MaxGResetValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MaxGReset {
                control: flightdataunit::MaxGReset::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_missile_select_button<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::MissileSelectButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MissileSelectButton {
                control: enginepanel::MissileSelectButton::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_motverk<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Motverk {
                control: errorpanel::Motverk::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_b1<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::NavSelectBtnB1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnB1 {
                control: navigation::NavSelectBtnB1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_b2<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::NavSelectBtnB2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnB2 {
                control: navigation::NavSelectBtnB2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_b3<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::NavSelectBtnB3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnB3 {
                control: navigation::NavSelectBtnB3::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_b4<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::NavSelectBtnB4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnB4 {
                control: navigation::NavSelectBtnB4::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_b5<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::NavSelectBtnB5Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnB5 {
                control: navigation::NavSelectBtnB5::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_b6<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::NavSelectBtnB6Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnB6 {
                control: navigation::NavSelectBtnB6::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_b7<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::NavSelectBtnB7Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnB7 {
                control: navigation::NavSelectBtnB7::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_b8<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::NavSelectBtnB8Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnB8 {
                control: navigation::NavSelectBtnB8::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_b9<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::NavSelectBtnB9Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnB9 {
                control: navigation::NavSelectBtnB9::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_bx<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::NavSelectBtnBxValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnBx {
                control: navigation::NavSelectBtnBx::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_l_mal<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::NavSelectBtnLMalValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnLMal {
                control: navigation::NavSelectBtnLMal::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_select_btn_ls<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigation::NavSelectBtnLsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSelectBtnLs {
                control: navigation::NavSelectBtnLs::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_syst<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavSyst {
                control: errorpanel::NavSyst::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_navigation_lights<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(lights::NavigationLightsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavigationLights {
                control: lights::NavigationLights::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nosstall<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Nosstall {
                control: errorpanel::Nosstall::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oljetryck<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Oljetryck {
                control: errorpanel::Oljetryck::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxygen_lever<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::OxygenLeverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxygenLever {
                control: flightdataunit::OxygenLever::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_panel_lights<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PanelLights {
                control: lights::PanelLights::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_parking_brake<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ParkingBrake {
                control: flightdataunit::ParkingBrake::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pedals<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Pedals {
                control: rawgaugevalues::Pedals::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pitch_gear_mode<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::PitchGearModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PitchGearMode {
                control: flightdataunit::PitchGearMode::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_position_lights<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(lights::PositionLightsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PositionLights {
                control: lights::PositionLights::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_position_lights_brightness<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(lights::PositionLightsBrightnessValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PositionLightsBrightness {
                control: lights::PositionLightsBrightness::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_altimeter_power<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(radaraltimeter::RadarAltimeterPowerValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarAltimeterPower {
                control: radaraltimeter::RadarAltimeterPower::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_bright<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarBright {
                control: radar::RadarBright::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_gain<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(radar::RadarGainValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarGain {
                control: radar::RadarGain::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_iff_id<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(radar::RadarIffIdValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarIffId {
                control: radar::RadarIffId::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_ignition_coils<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(radar::RadarIgnitionCoilsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarIgnitionCoils {
                control: radar::RadarIgnitionCoils::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_maintenance_test<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(radar::RadarMaintenanceTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarMaintenanceTest {
                control: radar::RadarMaintenanceTest::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_pulse_normal_short<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(radar::RadarPulseNormalShortValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarPulseNormalShort {
                control: radar::RadarPulseNormalShort::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_recce_on_off<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(radar::RadarRecceOnOffValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarRecceOnOff {
                control: radar::RadarRecceOnOff::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_warn_select<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwr::RadarWarnSelectValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarWarnSelect {
                control: rwr::RadarWarnSelect::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rb_bk_rel_mode<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponsystem::RbBkRelModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RbBkRelMode {
                control: weaponsystem::RbBkRelMode::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rensa_button_cover<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navigationpanel::RensaButtonCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RensaButtonCover {
                control: navigationpanel::RensaButtonCover::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_reserveff<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Reserveff {
                control: errorpanel::Reserveff::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_restart<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::RestartValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Restart {
                control: enginepanel::Restart::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_reversal<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(thrustreverser::ReversalValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Reversal {
                control: thrustreverser::Reversal::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rhm_fel<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RhmFel {
                control: errorpanel::RhmFel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_roll_center<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::RollCenterValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RollCenter {
                control: flightdataunit::RollCenter::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_roll_vaxel<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RollVaxel {
                control: errorpanel::RollVaxel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_slav_si<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::SlavSiValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SlavSi {
                control: flightdataunit::SlavSi::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_spak<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::SpakValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Spak {
                control: flightdataunit::Spak::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_spak_error<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::SpakError {
                control: errorpanel::SpakError::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_spak_lamp<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::SpakLamp {
                control: indicatorlights::SpakLamp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_start_system<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::StartSystemValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::StartSystem {
                control: enginepanel::StartSystem::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_startsyst<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Startsyst {
                control: errorpanel::Startsyst::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stick_pitch<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StickPitch {
                control: rawgaugevalues::StickPitch::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stick_roll<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StickRoll {
                control: rawgaugevalues::StickRoll::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_syrgas<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Syrgas {
                control: errorpanel::Syrgas::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tandsyst<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Tandsyst {
                control: errorpanel::Tandsyst::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tank_pump<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::TankPump {
                control: errorpanel::TankPump::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tank_release<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponsystem::TankReleaseValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TankRelease {
                control: weaponsystem::TankRelease::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tank_release_cover<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponsystem::TankReleaseCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TankReleaseCover {
                control: weaponsystem::TankReleaseCover::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_taxi_landing_lights<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(lights::TaxiLandingLightsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TaxiLandingLights {
                control: lights::TaxiLandingLights::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_test_mode<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TestMode {
                control: enginepanel::TestMode::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_throttle<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Throttle {
                control: rawgaugevalues::Throttle::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tils<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Tils {
                control: errorpanel::Tils::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tils_channel_layer<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::TilsChannelLayerValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TilsChannelLayer {
                control: flightdataunit::TilsChannelLayer::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tils_channel_select<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::TilsChannelSelectValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TilsChannelSelect {
                control: flightdataunit::TilsChannelSelect::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tippvaxel<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Tippvaxel {
                control: errorpanel::Tippvaxel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_trigger_safety_bracket<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponsystem::TriggerSafetyBracketValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TriggerSafetyBracket {
                control: weaponsystem::TriggerSafetyBracket::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vertical_acceleration<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VerticalAcceleration {
                control: rawgaugevalues::VerticalAcceleration::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vstall<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vstall {
                control: errorpanel::Vstall::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_warning_panel_test<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(warningpanel::WarningPanelTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::WarningPanelTest {
                control: warningpanel::WarningPanelTest::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon_interval<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponsystem::WeaponIntervalValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::WeaponInterval {
                control: weaponsystem::WeaponInterval::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon_rel_mode<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponsystem::WeaponRelModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::WeaponRelMode {
                control: weaponsystem::WeaponRelMode::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon_release<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponsystem::WeaponReleaseValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::WeaponRelease {
                control: weaponsystem::WeaponRelease::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon_release_cover<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponsystem::WeaponReleaseCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::WeaponReleaseCover {
                control: weaponsystem::WeaponReleaseCover::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon_select<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponsystem::WeaponSelectValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::WeaponSelect {
                control: weaponsystem::WeaponSelect::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_x_tank_bra<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::XTankBra {
                control: errorpanel::XTankBra::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_yaw_trim_cover<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightdataunit::YawTrimCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::YawTrimCover {
                control: flightdataunit::YawTrimCover::new(),
                callback,
            })
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
