//! Generated from model-definitions/FA-18C_hornet.json.
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

pub mod ampcd {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Brightness Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct AmpcdBrtCtl(LimitedDial<29904, 65535, 0>);
    impl AmpcdBrtCtl {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdBrtCtl {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdBrtCtl {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdContSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type AmpcdContSwValueCallback<'a> = &'a mut dyn FnMut(AmpcdContSwValue);
    impl core::convert::TryFrom<u16> for AmpcdContSwValue {
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

    /// Contrast Control Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct AmpcdContSw(Selector<29798, 3072, 10>);
    impl AmpcdContSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdContSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdContSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdGainSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type AmpcdGainSwValueCallback<'a> = &'a mut dyn FnMut(AmpcdGainSwValue);
    impl core::convert::TryFrom<u16> for AmpcdGainSwValue {
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

    /// Gain Control Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct AmpcdGainSw(Selector<29798, 12288, 12>);
    impl AmpcdGainSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdGainSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdGainSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdNightDayValue {
        Select1,
        Select2,
        Select3,
    }
    pub type AmpcdNightDayValueCallback<'a> = &'a mut dyn FnMut(AmpcdNightDayValue);
    impl core::convert::TryFrom<u16> for AmpcdNightDayValue {
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

    /// Night/Day Brightness Selector
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct AmpcdNightDay(Selector<29792, 24576, 13>);
    impl AmpcdNightDay {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdNightDay {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdNightDay {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdPb01Value {
        Select1,
        Select2,
    }
    pub type AmpcdPb01ValueCallback<'a> = &'a mut dyn FnMut(AmpcdPb01Value);
    impl core::convert::TryFrom<u16> for AmpcdPb01Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 1
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AmpcdPb01(Selector<29792, 32768, 15>);
    impl AmpcdPb01 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdPb01 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdPb01 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdPb02Value {
        Select1,
        Select2,
    }
    pub type AmpcdPb02ValueCallback<'a> = &'a mut dyn FnMut(AmpcdPb02Value);
    impl core::convert::TryFrom<u16> for AmpcdPb02Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AmpcdPb02(Selector<29798, 16384, 14>);
    impl AmpcdPb02 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdPb02 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdPb02 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdPb03Value {
        Select1,
        Select2,
    }
    pub type AmpcdPb03ValueCallback<'a> = &'a mut dyn FnMut(AmpcdPb03Value);
    impl core::convert::TryFrom<u16> for AmpcdPb03Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 3
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AmpcdPb03(Selector<29798, 32768, 15>);
    impl AmpcdPb03 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdPb03 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdPb03 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdPb04Value {
        Select1,
        Select2,
    }
    pub type AmpcdPb04ValueCallback<'a> = &'a mut dyn FnMut(AmpcdPb04Value);
    impl core::convert::TryFrom<u16> for AmpcdPb04Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 4
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AmpcdPb04(Selector<29812, 256, 8>);
    impl AmpcdPb04 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdPb04 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdPb04 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdPb05Value {
        Select1,
        Select2,
    }
    pub type AmpcdPb05ValueCallback<'a> = &'a mut dyn FnMut(AmpcdPb05Value);
    impl core::convert::TryFrom<u16> for AmpcdPb05Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 5
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AmpcdPb05(Selector<29812, 512, 9>);
    impl AmpcdPb05 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdPb05 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdPb05 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdPb06Value {
        Select1,
        Select2,
    }
    pub type AmpcdPb06ValueCallback<'a> = &'a mut dyn FnMut(AmpcdPb06Value);
    impl core::convert::TryFrom<u16> for AmpcdPb06Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 6
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AmpcdPb06(Selector<29812, 1024, 10>);
    impl AmpcdPb06 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdPb06 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdPb06 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdPb07Value {
        Select1,
        Select2,
    }
    pub type AmpcdPb07ValueCallback<'a> = &'a mut dyn FnMut(AmpcdPb07Value);
    impl core::convert::TryFrom<u16> for AmpcdPb07Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 7
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AmpcdPb07(Selector<29812, 2048, 11>);
    impl AmpcdPb07 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdPb07 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdPb07 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdPb08Value {
        Select1,
        Select2,
    }
    pub type AmpcdPb08ValueCallback<'a> = &'a mut dyn FnMut(AmpcdPb08Value);
    impl core::convert::TryFrom<u16> for AmpcdPb08Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 8
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AmpcdPb08(Selector<29812, 4096, 12>);
    impl AmpcdPb08 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdPb08 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdPb08 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdPb09Value {
        Select1,
        Select2,
    }
    pub type AmpcdPb09ValueCallback<'a> = &'a mut dyn FnMut(AmpcdPb09Value);
    impl core::convert::TryFrom<u16> for AmpcdPb09Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 9
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AmpcdPb09(Selector<29812, 8192, 13>);
    impl AmpcdPb09 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdPb09 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdPb09 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdPb10Value {
        Select1,
        Select2,
    }
    pub type AmpcdPb10ValueCallback<'a> = &'a mut dyn FnMut(AmpcdPb10Value);
    impl core::convert::TryFrom<u16> for AmpcdPb10Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 10
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AmpcdPb10(Selector<29812, 16384, 14>);
    impl AmpcdPb10 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdPb10 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdPb10 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdPb11Value {
        Select1,
        Select2,
    }
    pub type AmpcdPb11ValueCallback<'a> = &'a mut dyn FnMut(AmpcdPb11Value);
    impl core::convert::TryFrom<u16> for AmpcdPb11Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 11
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AmpcdPb11(Selector<29812, 32768, 15>);
    impl AmpcdPb11 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdPb11 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdPb11 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdPb12Value {
        Select1,
        Select2,
    }
    pub type AmpcdPb12ValueCallback<'a> = &'a mut dyn FnMut(AmpcdPb12Value);
    impl core::convert::TryFrom<u16> for AmpcdPb12Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 12
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AmpcdPb12(Selector<29814, 256, 8>);
    impl AmpcdPb12 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdPb12 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdPb12 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdPb13Value {
        Select1,
        Select2,
    }
    pub type AmpcdPb13ValueCallback<'a> = &'a mut dyn FnMut(AmpcdPb13Value);
    impl core::convert::TryFrom<u16> for AmpcdPb13Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 13
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AmpcdPb13(Selector<29814, 512, 9>);
    impl AmpcdPb13 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdPb13 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdPb13 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdPb14Value {
        Select1,
        Select2,
    }
    pub type AmpcdPb14ValueCallback<'a> = &'a mut dyn FnMut(AmpcdPb14Value);
    impl core::convert::TryFrom<u16> for AmpcdPb14Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 14
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AmpcdPb14(Selector<29814, 1024, 10>);
    impl AmpcdPb14 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdPb14 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdPb14 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdPb15Value {
        Select1,
        Select2,
    }
    pub type AmpcdPb15ValueCallback<'a> = &'a mut dyn FnMut(AmpcdPb15Value);
    impl core::convert::TryFrom<u16> for AmpcdPb15Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 15
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AmpcdPb15(Selector<29814, 2048, 11>);
    impl AmpcdPb15 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdPb15 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdPb15 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdPb16Value {
        Select1,
        Select2,
    }
    pub type AmpcdPb16ValueCallback<'a> = &'a mut dyn FnMut(AmpcdPb16Value);
    impl core::convert::TryFrom<u16> for AmpcdPb16Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 16
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AmpcdPb16(Selector<29814, 4096, 12>);
    impl AmpcdPb16 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdPb16 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdPb16 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdPb17Value {
        Select1,
        Select2,
    }
    pub type AmpcdPb17ValueCallback<'a> = &'a mut dyn FnMut(AmpcdPb17Value);
    impl core::convert::TryFrom<u16> for AmpcdPb17Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 17
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AmpcdPb17(Selector<29814, 8192, 13>);
    impl AmpcdPb17 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdPb17 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdPb17 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdPb18Value {
        Select1,
        Select2,
    }
    pub type AmpcdPb18ValueCallback<'a> = &'a mut dyn FnMut(AmpcdPb18Value);
    impl core::convert::TryFrom<u16> for AmpcdPb18Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 18
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AmpcdPb18(Selector<29814, 16384, 14>);
    impl AmpcdPb18 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdPb18 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdPb18 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdPb19Value {
        Select1,
        Select2,
    }
    pub type AmpcdPb19ValueCallback<'a> = &'a mut dyn FnMut(AmpcdPb19Value);
    impl core::convert::TryFrom<u16> for AmpcdPb19Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 19
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AmpcdPb19(Selector<29814, 32768, 15>);
    impl AmpcdPb19 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdPb19 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdPb19 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdPb20Value {
        Select1,
        Select2,
    }
    pub type AmpcdPb20ValueCallback<'a> = &'a mut dyn FnMut(AmpcdPb20Value);
    impl core::convert::TryFrom<u16> for AmpcdPb20Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 20
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AmpcdPb20(Selector<29816, 256, 8>);
    impl AmpcdPb20 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdPb20 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdPb20 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmpcdSymSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type AmpcdSymSwValueCallback<'a> = &'a mut dyn FnMut(AmpcdSymSwValue);
    impl core::convert::TryFrom<u16> for AmpcdSymSwValue {
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

    /// Symbology Control Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct AmpcdSymSw(Selector<29798, 768, 8>);
    impl AmpcdSymSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmpcdSymSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmpcdSymSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod angleofattackindexerlights {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// AOA Indexer High
    /// Output: 0 if light is off, 1 if light is on
    pub struct AoaIndexerHigh(Led<29704, 8, 3>);
    impl AoaIndexerHigh {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for AoaIndexerHigh {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AoaIndexerHigh {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AOA Indexer Low
    /// Output: 0 if light is off, 1 if light is on
    pub struct AoaIndexerLow(Led<29704, 32, 5>);
    impl AoaIndexerLow {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for AoaIndexerLow {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AoaIndexerLow {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AOA Indexer Normal
    /// Output: 0 if light is off, 1 if light is on
    pub struct AoaIndexerNormal(Led<29704, 16, 4>);
    impl AoaIndexerNormal {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for AoaIndexerNormal {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AoaIndexerNormal {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod antennaselectpanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Comm1AntSelectSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type Comm1AntSelectSwValueCallback<'a> = &'a mut dyn FnMut(Comm1AntSelectSwValue);
    impl core::convert::TryFrom<u16> for Comm1AntSelectSwValue {
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

    /// COMM 1 Antenna Selector Switch, UPPER/AUTO/LOWER
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Comm1AntSelectSw(Selector<29882, 6144, 11>);
    impl Comm1AntSelectSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Comm1AntSelectSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Comm1AntSelectSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffAntSelectSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type IffAntSelectSwValueCallback<'a> = &'a mut dyn FnMut(IffAntSelectSwValue);
    impl core::convert::TryFrom<u16> for IffAntSelectSwValue {
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

    /// IFF Antenna Selector Switch, UPPER/BOTH/LOWER
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffAntSelectSw(Selector<29882, 24576, 13>);
    impl IffAntSelectSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffAntSelectSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffAntSelectSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod apufirewarningextinguisherlight {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ApuFireBtnValue {
        Select1,
        Select2,
    }
    pub type ApuFireBtnValueCallback<'a> = &'a mut dyn FnMut(ApuFireBtnValue);
    impl core::convert::TryFrom<u16> for ApuFireBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// APU Fire Warning/Extinguisher Light
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ApuFireBtn(Selector<29708, 8, 3>);
    impl ApuFireBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ApuFireBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ApuFireBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FIRE APU
    /// Output: 0 if light is off, 1 if light is on
    pub struct FireApuLt(Led<29708, 4, 2>);
    impl FireApuLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for FireApuLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FireApuLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod arrestinghookhandleandlight {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Hook Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct ArrestingHookLt(Led<29858, 512, 9>);
    impl ArrestingHookLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ArrestingHookLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ArrestingHookLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HookLeverValue {
        Select1,
        Select2,
    }
    pub type HookLeverValueCallback<'a> = &'a mut dyn FnMut(HookLeverValue);
    impl core::convert::TryFrom<u16> for HookLeverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Hook Lever
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct HookLever(Selector<29858, 256, 8>);
    impl HookLever {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HookLever {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HookLever {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod auxiliarypowerunitpanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// APU Control Switch, ON/OFF
    /// Input: set the switch position -- 0 = off, 1 = on
    /// Output: switch position -- 0 = off, 1 = on
    pub struct ApuControlSw(Selector<29882, 32768, 15>);
    impl ApuControlSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ApuControlSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ApuControlSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// APU Ready Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct ApuReadyLt(Led<29884, 1024, 10>);
    impl ApuReadyLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ApuReadyLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ApuReadyLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EngineCrankSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type EngineCrankSwValueCallback<'a> = &'a mut dyn FnMut(EngineCrankSwValue);
    impl core::convert::TryFrom<u16> for EngineCrankSwValue {
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

    /// Engine Crank Switch, LEFT/OFF/RIGHT
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct EngineCrankSw(Selector<29884, 768, 8>);
    impl EngineCrankSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EngineCrankSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EngineCrankSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod canopyinternaljettisonhandle {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CanopyJettHandlePullValue {
        Select1,
        Select2,
    }
    pub type CanopyJettHandlePullValueCallback<'a> = &'a mut dyn FnMut(CanopyJettHandlePullValue);
    impl core::convert::TryFrom<u16> for CanopyJettHandlePullValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Canopy Jettison Handle Unlock Button - Press to jettison
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CanopyJettHandlePull(Selector<29708, 256, 8>);
    impl CanopyJettHandlePull {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CanopyJettHandlePull {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CanopyJettHandlePull {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CanopyJettHandleUnlockValue {
        Select1,
        Select2,
    }
    pub type CanopyJettHandleUnlockValueCallback<'a> =
        &'a mut dyn FnMut(CanopyJettHandleUnlockValue);
    impl core::convert::TryFrom<u16> for CanopyJettHandleUnlockValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Canopy Jettison Handle Unlock Button - Press to unlock
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CanopyJettHandleUnlock(Selector<29708, 128, 7>);
    impl CanopyJettHandleUnlock {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CanopyJettHandleUnlock {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CanopyJettHandleUnlock {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod cautionlightpanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// APU ACC
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClipApuAccLt(Led<29858, 32768, 15>);
    impl ClipApuAccLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClipApuAccLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClipApuAccLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// BATT SW
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClipBattSwLt(Led<29862, 256, 8>);
    impl ClipBattSwLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClipBattSwLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClipBattSwLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CK SEAT
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClipCkSeatLt(Led<29858, 16384, 14>);
    impl ClipCkSeatLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClipCkSeatLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClipCkSeatLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FCES
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClipFcesLt(Led<29862, 8192, 13>);
    impl ClipFcesLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClipFcesLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClipFcesLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FCS HOT
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClipFcsHotLt(Led<29862, 512, 9>);
    impl ClipFcsHotLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClipFcsHotLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClipFcsHotLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FUEL LO`
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClipFuelLoLt(Led<29862, 4096, 12>);
    impl ClipFuelLoLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClipFuelLoLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClipFuelLoLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// GEN TIE
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClipGenTieLt(Led<29862, 1024, 10>);
    impl ClipGenTieLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClipGenTieLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClipGenTieLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// L GEN
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClipLGenLt(Led<29862, 32768, 15>);
    impl ClipLGenLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClipLGenLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClipLGenLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// R GEN
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClipRGenLt(Led<29870, 256, 8>);
    impl ClipRGenLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClipRGenLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClipRGenLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SPARE CTN1
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClipSpareCtn1Lt(Led<29862, 2048, 11>);
    impl ClipSpareCtn1Lt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClipSpareCtn1Lt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClipSpareCtn1Lt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SPARE CTN2
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClipSpareCtn2Lt(Led<29862, 16384, 14>);
    impl ClipSpareCtn2Lt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClipSpareCtn2Lt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClipSpareCtn2Lt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SPARE CTN3
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClipSpareCtn3Lt(Led<29870, 512, 9>);
    impl ClipSpareCtn3Lt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClipSpareCtn3Lt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClipSpareCtn3Lt {
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

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Elapsed Minutes
    /// Output: gauge position
    pub struct ClockElapsedMinutes(AnalogGauge<29952, 65535, 0>);
    impl ClockElapsedMinutes {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockElapsedMinutes {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockElapsedMinutes {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Elapsed Seconds
    /// Output: gauge position
    pub struct ClockElapsedSeconds(AnalogGauge<29954, 65535, 0>);
    impl ClockElapsedSeconds {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockElapsedSeconds {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockElapsedSeconds {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Hours
    /// Output: gauge position
    pub struct ClockHours(AnalogGauge<29948, 65535, 0>);
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

    /// Minutes
    /// Output: gauge position
    pub struct ClockMinutes(AnalogGauge<29950, 65535, 0>);
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
}

pub mod cockpitaltimeter {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Pressure Altitude
    /// Output: gauge position
    pub struct PressureAlt(AnalogGauge<29956, 65535, 0>);
    impl PressureAlt {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PressureAlt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PressureAlt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod commsfrequency {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Comm 1 Channel as number
    /// Output: gauge position
    pub struct Comm1ChannelNumeric(AnalogGauge<29700, 65535, 0>);
    impl Comm1ChannelNumeric {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Comm1ChannelNumeric {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Comm1ChannelNumeric {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// COMM1 FREQ
    /// Output: frequency
    pub struct Comm1Freq(Metadata<29696, 65535, 0>);
    impl Comm1Freq {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Comm1Freq {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Comm1Freq {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Comm 2 Channel as number
    /// Output: gauge position
    pub struct Comm2ChannelNumeric(AnalogGauge<29702, 65535, 0>);
    impl Comm2ChannelNumeric {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Comm2ChannelNumeric {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Comm2ChannelNumeric {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// COMM2 FREQ
    /// Output: frequency
    pub struct Comm2Freq(Metadata<29698, 65535, 0>);
    impl Comm2Freq {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Comm2Freq {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Comm2Freq {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod communicationpanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// AUX Volume Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct ComAux(LimitedDial<29992, 65535, 0>);
    impl ComAux {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ComAux {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ComAux {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ComCommGXmtSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type ComCommGXmtSwValueCallback<'a> = &'a mut dyn FnMut(ComCommGXmtSwValue);
    impl core::convert::TryFrom<u16> for ComCommGXmtSwValue {
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

    /// COMM G XMT Switch, COMM 1/OFF/COMM 2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct ComCommGXmtSw(Selector<29878, 3072, 10>);
    impl ComCommGXmtSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ComCommGXmtSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ComCommGXmtSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ComCommRelaySwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type ComCommRelaySwValueCallback<'a> = &'a mut dyn FnMut(ComCommRelaySwValue);
    impl core::convert::TryFrom<u16> for ComCommRelaySwValue {
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

    /// Comm Relay Switch, CIPHER/OFF/PLAIN
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct ComCommRelaySw(Selector<29878, 768, 8>);
    impl ComCommRelaySw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ComCommRelaySw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ComCommRelaySw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ComCryptoSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type ComCryptoSwValueCallback<'a> = &'a mut dyn FnMut(ComCryptoSwValue);
    impl core::convert::TryFrom<u16> for ComCryptoSwValue {
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

    /// CRYPTO Switch, HOLD/NORM/ZERO
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct ComCryptoSw(Selector<29880, 768, 8>);
    impl ComCryptoSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ComCryptoSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ComCryptoSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ICS Volume Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct ComIcs(LimitedDial<29980, 65535, 0>);
    impl ComIcs {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ComIcs {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ComIcs {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ComIffMasterSwValue {
        Select1,
        Select2,
    }
    pub type ComIffMasterSwValueCallback<'a> = &'a mut dyn FnMut(ComIffMasterSwValue);
    impl core::convert::TryFrom<u16> for ComIffMasterSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// IFF Master Switch, EMER/NORM
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ComIffMasterSw(Selector<29878, 4096, 12>);
    impl ComIffMasterSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ComIffMasterSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ComIffMasterSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ComIffMode4SwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type ComIffMode4SwValueCallback<'a> = &'a mut dyn FnMut(ComIffMode4SwValue);
    impl core::convert::TryFrom<u16> for ComIffMode4SwValue {
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

    /// IFF Mode 4 Switch, DIS/AUD /DIS/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct ComIffMode4Sw(Selector<29878, 24576, 13>);
    impl ComIffMode4Sw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ComIffMode4Sw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ComIffMode4Sw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ILS Channel Selector Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct ComIlsChannelSw(Selector<29880, 31744, 10>);
    impl ComIlsChannelSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ComIlsChannelSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ComIlsChannelSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ComIlsUfcManSwValue {
        Select1,
        Select2,
    }
    pub type ComIlsUfcManSwValueCallback<'a> = &'a mut dyn FnMut(ComIlsUfcManSwValue);
    impl core::convert::TryFrom<u16> for ComIlsUfcManSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ILS UFC/MAN Switch, UFC/MAN
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ComIlsUfcManSw(Selector<29878, 32768, 15>);
    impl ComIlsUfcManSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ComIlsUfcManSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ComIlsUfcManSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// MIDS A Volume Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct ComMidsA(LimitedDial<29986, 65535, 0>);
    impl ComMidsA {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ComMidsA {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ComMidsA {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// MIDS B Volume Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct ComMidsB(LimitedDial<29988, 65535, 0>);
    impl ComMidsB {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ComMidsB {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ComMidsB {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR Volume Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct ComRwr(LimitedDial<29982, 65535, 0>);
    impl ComRwr {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ComRwr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ComRwr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// TACAN Volume Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct ComTacan(LimitedDial<29990, 65535, 0>);
    impl ComTacan {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ComTacan {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ComTacan {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// VOX Volume Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct ComVox(LimitedDial<29978, 65535, 0>);
    impl ComVox {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ComVox {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ComVox {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// WPN Volume Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct ComWpn(LimitedDial<29984, 65535, 0>);
    impl ComWpn {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ComWpn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ComWpn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod defogpanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Defog Handle
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct DefogHandle(LimitedDial<30016, 65535, 0>);
    impl DefogHandle {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for DefogHandle {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DefogHandle {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WshieldAntiIceSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type WshieldAntiIceSwValueCallback<'a> = &'a mut dyn FnMut(WshieldAntiIceSwValue);
    impl core::convert::TryFrom<u16> for WshieldAntiIceSwValue {
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

    /// Windshield Anti-Ice/Rain Switch, ANTI ICE/OFF/RAIN
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct WshieldAntiIceSw(Selector<29896, 768, 8>);
    impl WshieldAntiIceSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for WshieldAntiIceSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for WshieldAntiIceSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod dispenseremcpanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AuxRelSwValue {
        Select1,
        Select2,
    }
    pub type AuxRelSwValueCallback<'a> = &'a mut dyn FnMut(AuxRelSwValue);
    impl core::convert::TryFrom<u16> for AuxRelSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Auxiliary Release Switch, ENABLE/NORM
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AuxRelSw(Selector<29822, 32768, 15>);
    impl AuxRelSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AuxRelSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AuxRelSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CmsdDispenseSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type CmsdDispenseSwValueCallback<'a> = &'a mut dyn FnMut(CmsdDispenseSwValue);
    impl core::convert::TryFrom<u16> for CmsdDispenseSwValue {
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

    /// DISPENSER Switch, BYPASS/ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct CmsdDispenseSw(Selector<29822, 12288, 12>);
    impl CmsdDispenseSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmsdDispenseSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmsdDispenseSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CmsdJetSelBtnValue {
        Select1,
        Select2,
    }
    pub type CmsdJetSelBtnValueCallback<'a> = &'a mut dyn FnMut(CmsdJetSelBtnValue);
    impl core::convert::TryFrom<u16> for CmsdJetSelBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ECM JETT JETT SEL Button - Push to jettison
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CmsdJetSelBtn(Selector<29822, 16384, 14>);
    impl CmsdJetSelBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmsdJetSelBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmsdJetSelBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EcmModeSwValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type EcmModeSwValueCallback<'a> = &'a mut dyn FnMut(EcmModeSwValue);
    impl core::convert::TryFrom<u16> for EcmModeSwValue {
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

    /// ECM Mode Switch, XMIT/REC/BIT/STBY/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct EcmModeSw(Selector<29826, 1792, 8>);
    impl EcmModeSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EcmModeSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EcmModeSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod ecmdispenserbutton {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CmsdDispenseBtnValue {
        Select1,
        Select2,
    }
    pub type CmsdDispenseBtnValueCallback<'a> = &'a mut dyn FnMut(CmsdDispenseBtnValue);
    impl core::convert::TryFrom<u16> for CmsdDispenseBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Dispense Button - Push to dispense flares and chaff
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CmsdDispenseBtn(Selector<29884, 8192, 13>);
    impl CmsdDispenseBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmsdDispenseBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmsdDispenseBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod ejectionseat {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Ejection Control Handle
    /// Input: set the switch position -- 0 = off, 1 = on
    /// Output: switch position -- 0 = off, 1 = on
    pub struct EjectionHandleSw(Selector<29896, 32768, 15>);
    impl EjectionHandleSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EjectionHandleSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EjectionHandleSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EjectionSeatArmedValue {
        Select1,
        Select2,
    }
    pub type EjectionSeatArmedValueCallback<'a> = &'a mut dyn FnMut(EjectionSeatArmedValue);
    impl core::convert::TryFrom<u16> for EjectionSeatArmedValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Ejection Seat SAFE/ARMED Handle, SAFE/ARMED
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EjectionSeatArmed(Selector<29898, 256, 8>);
    impl EjectionSeatArmed {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EjectionSeatArmed {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EjectionSeatArmed {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EjectionSeatMnlOvrdValue {
        Select1,
        Select2,
    }
    pub type EjectionSeatMnlOvrdValueCallback<'a> = &'a mut dyn FnMut(EjectionSeatMnlOvrdValue);
    impl core::convert::TryFrom<u16> for EjectionSeatMnlOvrdValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Ejection Seat Manual Override Handle, PULL/PUSH
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EjectionSeatMnlOvrd(Selector<29898, 512, 9>);
    impl EjectionSeatMnlOvrd {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EjectionSeatMnlOvrd {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EjectionSeatMnlOvrd {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HideStickToggleValue {
        Select1,
        Select2,
    }
    pub type HideStickToggleValueCallback<'a> = &'a mut dyn FnMut(HideStickToggleValue);
    impl core::convert::TryFrom<u16> for HideStickToggleValue {
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
    pub struct HideStickToggle(Selector<29898, 8192, 13>);
    impl HideStickToggle {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HideStickToggle {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HideStickToggle {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SeatHeightSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type SeatHeightSwValueCallback<'a> = &'a mut dyn FnMut(SeatHeightSwValue);
    impl core::convert::TryFrom<u16> for SeatHeightSwValue {
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

    /// Seat Height Adjustment Switch, UP/HOLD/DOWN
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct SeatHeightSw(Selector<29898, 6144, 11>);
    impl SeatHeightSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SeatHeightSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SeatHeightSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ShldrHarnessSwValue {
        Select1,
        Select2,
    }
    pub type ShldrHarnessSwValueCallback<'a> = &'a mut dyn FnMut(ShldrHarnessSwValue);
    impl core::convert::TryFrom<u16> for ShldrHarnessSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Shoulder Harness Control Handle, LOCK/UNLOCK
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ShldrHarnessSw(Selector<29898, 1024, 10>);
    impl ShldrHarnessSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ShldrHarnessSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ShldrHarnessSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod electricalpowerpanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BatterySwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type BatterySwValueCallback<'a> = &'a mut dyn FnMut(BatterySwValue);
    impl core::convert::TryFrom<u16> for BatterySwValue {
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

    /// Battery Switch, ON/OFF/ORIDE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct BatterySw(Selector<29886, 3072, 10>);
    impl BatterySw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BatterySw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BatterySw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LGenSwValue {
        Select1,
        Select2,
    }
    pub type LGenSwValueCallback<'a> = &'a mut dyn FnMut(LGenSwValue);
    impl core::convert::TryFrom<u16> for LGenSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Left Generator Control Switch, NORM/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LGenSw(Selector<29886, 4096, 12>);
    impl LGenSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LGenSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LGenSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RGenSwValue {
        Select1,
        Select2,
    }
    pub type RGenSwValueCallback<'a> = &'a mut dyn FnMut(RGenSwValue);
    impl core::convert::TryFrom<u16> for RGenSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Right Generator Control Switch, NORM/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RGenSw(Selector<29886, 8192, 13>);
    impl RGenSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RGenSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RGenSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Battery E Volts
    /// Output: gauge position
    pub struct VoltE(AnalogGauge<29998, 65535, 0>);
    impl VoltE {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VoltE {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VoltE {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Battery U Volts
    /// Output: gauge position
    pub struct VoltU(AnalogGauge<29996, 65535, 0>);
    impl VoltU {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VoltU {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VoltU {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod emergencyandparkingbrakehandle {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EmergencyParkingBrakePullValue {
        Select1,
        Select2,
    }
    pub type EmergencyParkingBrakePullValueCallback<'a> =
        &'a mut dyn FnMut(EmergencyParkingBrakePullValue);
    impl core::convert::TryFrom<u16> for EmergencyParkingBrakePullValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Emergency/Parking Brake Pull
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EmergencyParkingBrakePull(Selector<29822, 1024, 10>);
    impl EmergencyParkingBrakePull {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EmergencyParkingBrakePull {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EmergencyParkingBrakePull {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Emergency/Parking Brake Rotate
    /// Input: set the switch position -- 0 = emergency, 1 = parking
    /// Output: switch position -- 0 = emergency, 1 = parking
    pub struct EmergencyParkingBrakeRotate(Selector<29822, 2048, 11>);
    impl EmergencyParkingBrakeRotate {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EmergencyParkingBrakeRotate {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EmergencyParkingBrakeRotate {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod emergencyjettisonbutton {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EmerJettBtnValue {
        Select1,
        Select2,
    }
    pub type EmerJettBtnValueCallback<'a> = &'a mut dyn FnMut(EmerJettBtnValue);
    impl core::convert::TryFrom<u16> for EmerJettBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Emergency Jettison Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EmerJettBtn(Selector<29740, 256, 8>);
    impl EmerJettBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EmerJettBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EmerJettBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod environmentcontrollouver {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Left Louver
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct LeftLouver(LimitedDial<29938, 65535, 0>);
    impl LeftLouver {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftLouver {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftLouver {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Right Louver
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RightLouver(LimitedDial<29940, 65535, 0>);
    impl RightLouver {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightLouver {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightLouver {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod environmentcontrolsystempanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BleedAirKnobValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type BleedAirKnobValueCallback<'a> = &'a mut dyn FnMut(BleedAirKnobValue);
    impl core::convert::TryFrom<u16> for BleedAirKnobValue {
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

    /// Bleed Air Knob, R OFF/NORM/L OFF/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct BleedAirKnob(Selector<29886, 49152, 14>);
    impl BleedAirKnob {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BleedAirKnob {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BleedAirKnob {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BleedAirPullValue {
        Select1,
        Select2,
    }
    pub type BleedAirPullValueCallback<'a> = &'a mut dyn FnMut(BleedAirPullValue);
    impl core::convert::TryFrom<u16> for BleedAirPullValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Bleed Air Knob, AUG PULL
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct BleedAirPull(Selector<29888, 256, 8>);
    impl BleedAirPull {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BleedAirPull {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BleedAirPull {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CabinPressSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type CabinPressSwValueCallback<'a> = &'a mut dyn FnMut(CabinPressSwValue);
    impl core::convert::TryFrom<u16> for CabinPressSwValue {
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

    /// Cabin Pressure Switch, NORM/DUMP/ RAM/DUMP
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct CabinPressSw(Selector<29888, 6144, 11>);
    impl CabinPressSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CabinPressSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CabinPressSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Cabin Temperature Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct CabinTemp(LimitedDial<30000, 65535, 0>);
    impl CabinTemp {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CabinTemp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CabinTemp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EcsModeSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type EcsModeSwValueCallback<'a> = &'a mut dyn FnMut(EcsModeSwValue);
    impl core::convert::TryFrom<u16> for EcsModeSwValue {
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

    /// ECS Mode Switch, AUTO/MAN/ OFF/RAM
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct EcsModeSw(Selector<29888, 1536, 9>);
    impl EcsModeSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EcsModeSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EcsModeSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EngAntiiceSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type EngAntiiceSwValueCallback<'a> = &'a mut dyn FnMut(EngAntiiceSwValue);
    impl core::convert::TryFrom<u16> for EngAntiiceSwValue {
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

    /// Engine Anti-Ice Switch, ON/OFF/TEST
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct EngAntiiceSw(Selector<29888, 24576, 13>);
    impl EngAntiiceSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EngAntiiceSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EngAntiiceSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pitot Heater Switch, ON/AUTO
    /// Input: set the switch position -- 0 = off, 1 = on
    /// Output: switch position -- 0 = off, 1 = on
    pub struct PitotHeatSw(Selector<29888, 32768, 15>);
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

    /// Suit Temperature Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct SuitTemp(LimitedDial<30002, 65535, 0>);
    impl SuitTemp {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SuitTemp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SuitTemp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod exteriorlightspanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Formation Lights Dimmer
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct FormationDimmer(LimitedDial<29974, 65535, 0>);
    impl FormationDimmer {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FormationDimmer {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FormationDimmer {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IntWngTankSwValue {
        Select1,
        Select2,
    }
    pub type IntWngTankSwValueCallback<'a> = &'a mut dyn FnMut(IntWngTankSwValue);
    impl core::convert::TryFrom<u16> for IntWngTankSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Internal Wing Tank Fuel Control Switch, INHIBIT/NORM
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IntWngTankSw(Selector<29874, 4096, 12>);
    impl IntWngTankSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IntWngTankSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IntWngTankSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Position Lights Dimmer
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct PositionDimmer(LimitedDial<29972, 65535, 0>);
    impl PositionDimmer {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PositionDimmer {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PositionDimmer {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum StrobeSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type StrobeSwValueCallback<'a> = &'a mut dyn FnMut(StrobeSwValue);
    impl core::convert::TryFrom<u16> for StrobeSwValue {
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

    /// Strobe Lights Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct StrobeSw(Selector<29874, 3072, 10>);
    impl StrobeSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for StrobeSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for StrobeSw {
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
    pub struct ExtFormationLights(Metadata<30022, 65535, 0>);
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

    /// Left Position Light
    /// Output: Left Position Light
    pub struct ExtPositionLightLeft(Metadata<29900, 256, 8>);
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

    /// Right Position Light
    /// Output: Right Position Light
    pub struct ExtPositionLightRight(Metadata<29900, 512, 9>);
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

    /// Speed Brake
    /// Output: Speed Brake
    pub struct ExtSpeedBrake(Metadata<30020, 65535, 0>);
    impl ExtSpeedBrake {
        pub const fn new() -> Self {
            Self(Metadata::new())
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

    /// Strobe Lights
    /// Output: Strobe Lights
    pub struct ExtStrobeLights(Metadata<29900, 1024, 10>);
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

pub mod firesystems {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FireExtBtnValue {
        Select1,
        Select2,
    }
    pub type FireExtBtnValueCallback<'a> = &'a mut dyn FnMut(FireExtBtnValue);
    impl core::convert::TryFrom<u16> for FireExtBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Fire Extinguisher Pushbutton
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FireExtBtn(Selector<29710, 1, 0>);
    impl FireExtBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FireExtBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FireExtBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod firetestpanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FireTestSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type FireTestSwValueCallback<'a> = &'a mut dyn FnMut(FireTestSwValue);
    impl core::convert::TryFrom<u16> for FireTestSwValue {
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

    /// Fire and Bleed Air Test Switch, (RMB) TEST A/(LMB) TEST B
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct FireTestSw(Selector<29870, 49152, 14>);
    impl FireTestSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FireTestSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FireTestSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod flapslandinggearstoresindicatorpanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// FLAPS
    /// Output: 0 if light is off, 1 if light is on
    pub struct FlpLgFlapsLt(Led<29792, 1, 0>);
    impl FlpLgFlapsLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for FlpLgFlapsLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlpLgFlapsLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FULL FLAPS
    /// Output: 0 if light is off, 1 if light is on
    pub struct FlpLgFullFlapsLt(Led<29744, 32768, 15>);
    impl FlpLgFullFlapsLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for FlpLgFullFlapsLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlpLgFullFlapsLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HALF FLAPS
    /// Output: 0 if light is off, 1 if light is on
    pub struct FlpLgHalfFlapsLt(Led<29744, 16384, 14>);
    impl FlpLgHalfFlapsLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for FlpLgHalfFlapsLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlpLgHalfFlapsLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// LEFT GEAR
    /// Output: 0 if light is off, 1 if light is on
    pub struct FlpLgLeftGearLt(Led<29744, 4096, 12>);
    impl FlpLgLeftGearLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for FlpLgLeftGearLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlpLgLeftGearLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// NOSE GEAR
    /// Output: 0 if light is off, 1 if light is on
    pub struct FlpLgNoseGearLt(Led<29744, 2048, 11>);
    impl FlpLgNoseGearLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for FlpLgNoseGearLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlpLgNoseGearLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RIGHT GEAR
    /// Output: 0 if light is off, 1 if light is on
    pub struct FlpLgRightGearLt(Led<29744, 8192, 13>);
    impl FlpLgRightGearLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for FlpLgRightGearLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlpLgRightGearLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod flightcomputercoolswitch {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AvCoolSwValue {
        Select1,
        Select2,
    }
    pub type AvCoolSwValueCallback<'a> = &'a mut dyn FnMut(AvCoolSwValue);
    impl core::convert::TryFrom<u16> for AvCoolSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// AV COOL Switch, NORM/EMERG
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AvCoolSw(Selector<29858, 8192, 13>);
    impl AvCoolSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AvCoolSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AvCoolSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod flightcontrolsystempanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FcsResetBtnValue {
        Select1,
        Select2,
    }
    pub type FcsResetBtnValueCallback<'a> = &'a mut dyn FnMut(FcsResetBtnValue);
    impl core::convert::TryFrom<u16> for FcsResetBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FCS RESET Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FcsResetBtn(Selector<29876, 8192, 13>);
    impl FcsResetBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FcsResetBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FcsResetBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GainSwitchValue {
        Select1,
        Select2,
    }
    pub type GainSwitchValueCallback<'a> = &'a mut dyn FnMut(GainSwitchValue);
    impl core::convert::TryFrom<u16> for GainSwitchValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// GAIN Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct GainSwitch(Selector<29876, 32768, 15>);
    impl GainSwitch {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GainSwitch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GainSwitch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GainSwitchCoverValue {
        Select1,
        Select2,
    }
    pub type GainSwitchCoverValueCallback<'a> = &'a mut dyn FnMut(GainSwitchCoverValue);
    impl core::convert::TryFrom<u16> for GainSwitchCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// GAIN Switch Cover
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct GainSwitchCover(Selector<29876, 16384, 14>);
    impl GainSwitchCover {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GainSwitchCover {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GainSwitchCover {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RUD TRIM Control
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RudTrim(LimitedDial<29976, 65535, 0>);
    impl RudTrim {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RudTrim {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RudTrim {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ToTrimBtnValue {
        Select1,
        Select2,
    }
    pub type ToTrimBtnValueCallback<'a> = &'a mut dyn FnMut(ToTrimBtnValue);
    impl core::convert::TryFrom<u16> for ToTrimBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// T/O TRIM Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ToTrimBtn(Selector<29876, 4096, 12>);
    impl ToTrimBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ToTrimBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ToTrimBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod fuelpanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ExtCntTankSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type ExtCntTankSwValueCallback<'a> = &'a mut dyn FnMut(ExtCntTankSwValue);
    impl core::convert::TryFrom<u16> for ExtCntTankSwValue {
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

    /// External Centerline Tank Fuel Control Switch, STOP/NORM/ORIDE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct ExtCntTankSw(Selector<29876, 768, 8>);
    impl ExtCntTankSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtCntTankSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtCntTankSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ExtWngTankSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type ExtWngTankSwValueCallback<'a> = &'a mut dyn FnMut(ExtWngTankSwValue);
    impl core::convert::TryFrom<u16> for ExtWngTankSwValue {
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

    /// External Wing Tanks Fuel Control Switch, STOP/NORM/ORIDE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct ExtWngTankSw(Selector<29876, 3072, 10>);
    impl ExtWngTankSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtWngTankSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtWngTankSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Fuel Dump Switch, ON/OFF
    /// Input: set the switch position -- 0 = off, 1 = on
    /// Output: switch position -- 0 = off, 1 = on
    pub struct FuelDumpSw(Selector<29874, 32768, 15>);
    impl FuelDumpSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FuelDumpSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FuelDumpSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ProbeSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type ProbeSwValueCallback<'a> = &'a mut dyn FnMut(ProbeSwValue);
    impl core::convert::TryFrom<u16> for ProbeSwValue {
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

    /// Probe Control Switch, EXTEND/RETRACT/EMERG EXTD
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct ProbeSw(Selector<29874, 24576, 13>);
    impl ProbeSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ProbeSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ProbeSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod generatortiecontrolswitch {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GenTieCoverValue {
        Select1,
        Select2,
    }
    pub type GenTieCoverValueCallback<'a> = &'a mut dyn FnMut(GenTieCoverValue);
    impl core::convert::TryFrom<u16> for GenTieCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Generator TIE Control Switch Cover, OPEN/CLOSE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct GenTieCover(Selector<29884, 2048, 11>);
    impl GenTieCover {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GenTieCover {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GenTieCover {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GenTieSwValue {
        Select1,
        Select2,
    }
    pub type GenTieSwValueCallback<'a> = &'a mut dyn FnMut(GenTieSwValue);
    impl core::convert::TryFrom<u16> for GenTieSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Generator TIE Control Switch, NORM/RESET
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct GenTieSw(Selector<29884, 4096, 12>);
    impl GenTieSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GenTieSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GenTieSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod groundpowerpanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ExtPwrSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type ExtPwrSwValueCallback<'a> = &'a mut dyn FnMut(ExtPwrSwValue);
    impl core::convert::TryFrom<u16> for ExtPwrSwValue {
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

    /// External Power Switch, RESET/NORM/OFF
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct ExtPwrSw(Selector<29872, 768, 8>);
    impl ExtPwrSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtPwrSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtPwrSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GndPwr1SwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type GndPwr1SwValueCallback<'a> = &'a mut dyn FnMut(GndPwr1SwValue);
    impl core::convert::TryFrom<u16> for GndPwr1SwValue {
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

    /// Ground Power Switch 1, A ON/AUTO/B ON
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct GndPwr1Sw(Selector<29872, 3072, 10>);
    impl GndPwr1Sw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GndPwr1Sw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GndPwr1Sw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GndPwr2SwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type GndPwr2SwValueCallback<'a> = &'a mut dyn FnMut(GndPwr2SwValue);
    impl core::convert::TryFrom<u16> for GndPwr2SwValue {
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

    /// Ground Power Switch 2, A ON/AUTO/B ON
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct GndPwr2Sw(Selector<29872, 12288, 12>);
    impl GndPwr2Sw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GndPwr2Sw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GndPwr2Sw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GndPwr3SwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type GndPwr3SwValueCallback<'a> = &'a mut dyn FnMut(GndPwr3SwValue);
    impl core::convert::TryFrom<u16> for GndPwr3SwValue {
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

    /// Ground Power Switch 3, A ON/AUTO/B ON
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct GndPwr3Sw(Selector<29872, 49152, 14>);
    impl GndPwr3Sw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GndPwr3Sw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GndPwr3Sw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GndPwr4SwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type GndPwr4SwValueCallback<'a> = &'a mut dyn FnMut(GndPwr4SwValue);
    impl core::convert::TryFrom<u16> for GndPwr4SwValue {
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

    /// Ground Power Switch 4, A ON/AUTO/B ON
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct GndPwr4Sw(Selector<29874, 768, 8>);
    impl GndPwr4Sw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GndPwr4Sw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GndPwr4Sw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod headingandcoursesetswitches {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiCrsSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type LeftDdiCrsSwValueCallback<'a> = &'a mut dyn FnMut(LeftDdiCrsSwValue);
    impl core::convert::TryFrom<u16> for LeftDdiCrsSwValue {
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

    /// Course Set Switch
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct LeftDdiCrsSw(Selector<29870, 12288, 12>);
    impl LeftDdiCrsSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiCrsSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiCrsSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiHdgSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type LeftDdiHdgSwValueCallback<'a> = &'a mut dyn FnMut(LeftDdiHdgSwValue);
    impl core::convert::TryFrom<u16> for LeftDdiHdgSwValue {
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

    /// Heading Set Switch
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct LeftDdiHdgSw(Selector<29870, 3072, 10>);
    impl LeftDdiHdgSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiHdgSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiHdgSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod hudcontrolpanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HudAltSwValue {
        Select1,
        Select2,
    }
    pub type HudAltSwValueCallback<'a> = &'a mut dyn FnMut(HudAltSwValue);
    impl core::convert::TryFrom<u16> for HudAltSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Altitude Switch, BARO/RDR
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct HudAltSw(Selector<29740, 16384, 14>);
    impl HudAltSw {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// AOA Indexer Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct HudAoaIndexer(LimitedDial<29790, 65535, 0>);
    impl HudAoaIndexer {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HudAoaIndexer {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HudAoaIndexer {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HudAttSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type HudAttSwValueCallback<'a> = &'a mut dyn FnMut(HudAttSwValue);
    impl core::convert::TryFrom<u16> for HudAttSwValue {
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

    /// Attitude Selector Switch, INS/AUTO/STBY
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct HudAttSw(Selector<29742, 768, 8>);
    impl HudAttSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HudAttSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HudAttSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Balance Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct HudBalance(LimitedDial<29788, 65535, 0>);
    impl HudBalance {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HudBalance {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HudBalance {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Black Level Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct HudBlackLvl(LimitedDial<29786, 65535, 0>);
    impl HudBlackLvl {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HudBlackLvl {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HudBlackLvl {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HUD Symbology Brightness Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct HudSymBrt(LimitedDial<29784, 65535, 0>);
    impl HudSymBrt {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HudSymBrt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HudSymBrt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HudSymBrtSelectValue {
        Select1,
        Select2,
    }
    pub type HudSymBrtSelectValueCallback<'a> = &'a mut dyn FnMut(HudSymBrtSelectValue);
    impl core::convert::TryFrom<u16> for HudSymBrtSelectValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// HUD Symbology Brightness Selector Knob, DAY/NIGHT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct HudSymBrtSelect(Selector<29740, 2048, 11>);
    impl HudSymBrtSelect {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HudSymBrtSelect {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HudSymBrtSelect {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HudSymRejSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type HudSymRejSwValueCallback<'a> = &'a mut dyn FnMut(HudSymRejSwValue);
    impl core::convert::TryFrom<u16> for HudSymRejSwValue {
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

    /// HUD Symbology Reject Switch, NORM/REJ 1/REJ 2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct HudSymRejSw(Selector<29740, 1536, 9>);
    impl HudSymRejSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HudSymRejSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HudSymRejSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HudVideoControlSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type HudVideoControlSwValueCallback<'a> = &'a mut dyn FnMut(HudVideoControlSwValue);
    impl core::convert::TryFrom<u16> for HudVideoControlSwValue {
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

    /// HUD Video Control Switch, W/B /VID/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct HudVideoControlSw(Selector<29740, 12288, 12>);
    impl HudVideoControlSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HudVideoControlSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HudVideoControlSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod hudvideobitpanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HudVideoBitValue {
        Select1,
        Select2,
    }
    pub type HudVideoBitValueCallback<'a> = &'a mut dyn FnMut(HudVideoBitValue);
    impl core::convert::TryFrom<u16> for HudVideoBitValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// HUD Video BIT Initiate Pushbutton - Push to initiate BIT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct HudVideoBit(Selector<29706, 64, 6>);
    impl HudVideoBit {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HudVideoBit {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HudVideoBit {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod hudvideorecordpanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Brightness Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct Ifei(LimitedDial<29902, 65535, 0>);
    impl Ifei {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ifei {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ifei {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ModeSelectorSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type ModeSelectorSwValueCallback<'a> = &'a mut dyn FnMut(ModeSelectorSwValue);
    impl core::convert::TryFrom<u16> for ModeSelectorSwValue {
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

    /// Mode Selector Switch, MAN/OFF/AUTO
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct ModeSelectorSw(Selector<29792, 6144, 11>);
    impl ModeSelectorSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ModeSelectorSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ModeSelectorSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SelectHmdLddiRddiValue {
        Select1,
        Select2,
        Select3,
    }
    pub type SelectHmdLddiRddiValueCallback<'a> = &'a mut dyn FnMut(SelectHmdLddiRddiValue);
    impl core::convert::TryFrom<u16> for SelectHmdLddiRddiValue {
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

    /// Selector Switch, HMD/LDDI/RDDI
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct SelectHmdLddiRddi(Selector<29792, 384, 7>);
    impl SelectHmdLddiRddi {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SelectHmdLddiRddi {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SelectHmdLddiRddi {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SelectHudLddiRddiValue {
        Select1,
        Select2,
        Select3,
    }
    pub type SelectHudLddiRddiValueCallback<'a> = &'a mut dyn FnMut(SelectHudLddiRddiValue);
    impl core::convert::TryFrom<u16> for SelectHudLddiRddiValue {
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

    /// Selector Switch, HUD/LDIR/RDDI
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct SelectHudLddiRddi(Selector<29792, 1536, 9>);
    impl SelectHudLddiRddi {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SelectHudLddiRddi {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SelectHudLddiRddi {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod hyd1andhydpressureindicator {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// HYD Indicator Left
    /// Output: gauge position
    pub struct HydIndLeft(AnalogGauge<29966, 65535, 0>);
    impl HydIndLeft {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HydIndLeft {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HydIndLeft {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HYD Indicator Right
    /// Output: gauge position
    pub struct HydIndRight(AnalogGauge<29968, 65535, 0>);
    impl HydIndRight {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HydIndRight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HydIndRight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod integratedfuelengineindicatorifei {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// BINGO
    /// Output: BINGO
    pub struct IfeiBingo(Display<29794, 5>);
    impl IfeiBingo {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiBingo {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiBingo {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// BINGO Texture Visible: 1 = yes, 0 = no
    /// Output: BINGO Texture Visible: 1 = yes, 0 = no
    pub struct IfeiBingoTexture(Display<29878, 1>);
    impl IfeiBingoTexture {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiBingoTexture {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiBingoTexture {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CLOCK_H
    /// Output: CLOCK_H
    pub struct IfeiClockH(Display<29800, 2>);
    impl IfeiClockH {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiClockH {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiClockH {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CLOCK_M
    /// Output: CLOCK_M
    pub struct IfeiClockM(Display<29802, 2>);
    impl IfeiClockM {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiClockM {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiClockM {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CLOCK_S
    /// Output: CLOCK_S
    pub struct IfeiClockS(Display<29804, 2>);
    impl IfeiClockS {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiClockS {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiClockS {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Codes
    /// Output: Codes
    pub struct IfeiCodes(Display<29856, 3>);
    impl IfeiCodes {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiCodes {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiCodes {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DD_1
    /// Output: DD_1
    pub struct IfeiDd1(Display<29812, 1>);
    impl IfeiDd1 {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiDd1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiDd1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DD_2
    /// Output: DD_2
    pub struct IfeiDd2(Display<29814, 1>);
    impl IfeiDd2 {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiDd2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiDd2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DD_3
    /// Output: DD_3
    pub struct IfeiDd3(Display<29816, 1>);
    impl IfeiDd3 {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiDd3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiDd3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DD_4
    /// Output: DD_4
    pub struct IfeiDd4(Display<29818, 1>);
    impl IfeiDd4 {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiDd4 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiDd4 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IfeiDwnBtnValue {
        Select1,
        Select2,
    }
    pub type IfeiDwnBtnValueCallback<'a> = &'a mut dyn FnMut(IfeiDwnBtnValue);
    impl core::convert::TryFrom<u16> for IfeiDwnBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Down Arrow Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IfeiDwnBtn(Selector<29792, 16, 4>);
    impl IfeiDwnBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IfeiDwnBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiDwnBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IfeiEtBtnValue {
        Select1,
        Select2,
    }
    pub type IfeiEtBtnValueCallback<'a> = &'a mut dyn FnMut(IfeiEtBtnValue);
    impl core::convert::TryFrom<u16> for IfeiEtBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ET Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IfeiEtBtn(Selector<29792, 64, 6>);
    impl IfeiEtBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IfeiEtBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiEtBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FF_L
    /// Output: FF_L
    pub struct IfeiFfL(Display<29820, 3>);
    impl IfeiFfL {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiFfL {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiFfL {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FF_R
    /// Output: FF_R
    pub struct IfeiFfR(Display<29824, 3>);
    impl IfeiFfR {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiFfR {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiFfR {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FF Texture Visible: 1 = yes, 0 = no
    /// Output: FF Texture Visible: 1 = yes, 0 = no
    pub struct IfeiFfTexture(Display<29874, 1>);
    impl IfeiFfTexture {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiFfTexture {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiFfTexture {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FUEL_DOWN
    /// Output: FUEL_DOWN
    pub struct IfeiFuelDown(Display<29828, 6>);
    impl IfeiFuelDown {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiFuelDown {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiFuelDown {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FUEL_UP
    /// Output: FUEL_UP
    pub struct IfeiFuelUp(Display<29834, 6>);
    impl IfeiFuelUp {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiFuelUp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiFuelUp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Right 0 Texture Visible: 1 = yes, 0 = no
    /// Output: Right 0 Texture Visible: 1 = yes, 0 = no
    pub struct IfeiL0Texture(Display<29886, 1>);
    impl IfeiL0Texture {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiL0Texture {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiL0Texture {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Left 100 Texture Visible: 1 = yes, 0 = no
    /// Output: Left 100 Texture Visible: 1 = yes, 0 = no
    pub struct IfeiL100Texture(Display<29892, 1>);
    impl IfeiL100Texture {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiL100Texture {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiL100Texture {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Left 50 Texture Visible: 1 = yes, 0 = no
    /// Output: Left 50 Texture Visible: 1 = yes, 0 = no
    pub struct IfeiL50Texture(Display<29888, 1>);
    impl IfeiL50Texture {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiL50Texture {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiL50Texture {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Left Pointer Texture Visible: 1 = yes, 0 = no
    /// Output: Left Pointer Texture Visible: 1 = yes, 0 = no
    pub struct IfeiLpointerTexture(Display<29896, 1>);
    impl IfeiLpointerTexture {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiLpointerTexture {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiLpointerTexture {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Left Scale Texture Visible: 1 = yes, 0 = no
    /// Output: Left Scale Texture Visible: 1 = yes, 0 = no
    pub struct IfeiLscaleTexture(Display<29880, 1>);
    impl IfeiLscaleTexture {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiLscaleTexture {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiLscaleTexture {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IfeiModeBtnValue {
        Select1,
        Select2,
    }
    pub type IfeiModeBtnValueCallback<'a> = &'a mut dyn FnMut(IfeiModeBtnValue);
    impl core::convert::TryFrom<u16> for IfeiModeBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Mode Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IfeiModeBtn(Selector<29792, 2, 1>);
    impl IfeiModeBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IfeiModeBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiModeBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// OilPress_L
    /// Output: OilPress_L
    pub struct IfeiOilPressL(Display<29840, 2>);
    impl IfeiOilPressL {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiOilPressL {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiOilPressL {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// OilPress_R
    /// Output: OilPress_R
    pub struct IfeiOilPressR(Display<29842, 2>);
    impl IfeiOilPressR {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiOilPressR {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiOilPressR {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// OIL Texture Visible: 1 = yes, 0 = no
    /// Output: OIL Texture Visible: 1 = yes, 0 = no
    pub struct IfeiOilTexture(Display<29876, 1>);
    impl IfeiOilTexture {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiOilTexture {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiOilTexture {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IfeiQtyBtnValue {
        Select1,
        Select2,
    }
    pub type IfeiQtyBtnValueCallback<'a> = &'a mut dyn FnMut(IfeiQtyBtnValue);
    impl core::convert::TryFrom<u16> for IfeiQtyBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// QTY Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IfeiQtyBtn(Selector<29792, 4, 2>);
    impl IfeiQtyBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IfeiQtyBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiQtyBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Right 100 Texture Visible: 1 = yes, 0 = no
    /// Output: Right 100 Texture Visible: 1 = yes, 0 = no
    pub struct IfeiR100Texture(Display<29894, 1>);
    impl IfeiR100Texture {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiR100Texture {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiR100Texture {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Right 50 Texture Visible: 1 = yes, 0 = no
    /// Output: Right 50 Texture Visible: 1 = yes, 0 = no
    pub struct IfeiR50Texture(Display<29890, 1>);
    impl IfeiR50Texture {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiR50Texture {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiR50Texture {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RPM_L
    /// Output: RPM_L
    pub struct IfeiRpmL(Display<29844, 2>);
    impl IfeiRpmL {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiRpmL {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiRpmL {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RPM_R
    /// Output: RPM_R
    pub struct IfeiRpmR(Display<29846, 2>);
    impl IfeiRpmR {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiRpmR {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiRpmR {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RPM Texture Visible: 1 = yes, 0 = no
    /// Output: RPM Texture Visible: 1 = yes, 0 = no
    pub struct IfeiRpmTexture(Display<29870, 1>);
    impl IfeiRpmTexture {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiRpmTexture {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiRpmTexture {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Right Pointer Texture Visible: 1 = yes, 0 = no
    /// Output: Right Pointer Texture Visible: 1 = yes, 0 = no
    pub struct IfeiRpointerTexture(Display<29898, 1>);
    impl IfeiRpointerTexture {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiRpointerTexture {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiRpointerTexture {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Right Scale Texture Visible: 1 = yes, 0 = no
    /// Output: Right Scale Texture Visible: 1 = yes, 0 = no
    pub struct IfeiRscaleTexture(Display<29882, 1>);
    impl IfeiRscaleTexture {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiRscaleTexture {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiRscaleTexture {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SP
    /// Output: SP
    pub struct IfeiSp(Display<29860, 3>);
    impl IfeiSp {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiSp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiSp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// TEMP_L
    /// Output: TEMP_L
    pub struct IfeiTempL(Display<29848, 3>);
    impl IfeiTempL {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiTempL {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiTempL {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// TEMP_R
    /// Output: TEMP_R
    pub struct IfeiTempR(Display<29852, 3>);
    impl IfeiTempR {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiTempR {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiTempR {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Temp Texture Visible: 1 = yes, 0 = no
    /// Output: Temp Texture Visible: 1 = yes, 0 = no
    pub struct IfeiTempTexture(Display<29872, 1>);
    impl IfeiTempTexture {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiTempTexture {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiTempTexture {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Time Set Mode
    /// Output: Time Set Mode
    pub struct IfeiTimeSetMode(Display<29864, 6>);
    impl IfeiTimeSetMode {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiTimeSetMode {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiTimeSetMode {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// TIMER_H
    /// Output: TIMER_H
    pub struct IfeiTimerH(Display<29806, 2>);
    impl IfeiTimerH {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiTimerH {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiTimerH {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// TIMER_M
    /// Output: TIMER_M
    pub struct IfeiTimerM(Display<29808, 2>);
    impl IfeiTimerM {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiTimerM {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiTimerM {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// TIMER_S
    /// Output: TIMER_S
    pub struct IfeiTimerS(Display<29810, 2>);
    impl IfeiTimerS {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiTimerS {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiTimerS {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IfeiUpBtnValue {
        Select1,
        Select2,
    }
    pub type IfeiUpBtnValueCallback<'a> = &'a mut dyn FnMut(IfeiUpBtnValue);
    impl core::convert::TryFrom<u16> for IfeiUpBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Up Arrow Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IfeiUpBtn(Selector<29792, 8, 3>);
    impl IfeiUpBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IfeiUpBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiUpBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Zulu Texture Visible: 1 = yes, 0 = no
    /// Output: Zulu Texture Visible: 1 = yes, 0 = no
    pub struct IfeiZTexture(Display<29900, 1>);
    impl IfeiZTexture {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IfeiZTexture {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiZTexture {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IfeiZoneBtnValue {
        Select1,
        Select2,
    }
    pub type IfeiZoneBtnValueCallback<'a> = &'a mut dyn FnMut(IfeiZoneBtnValue);
    impl core::convert::TryFrom<u16> for IfeiZoneBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ZONE Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IfeiZoneBtn(Selector<29792, 32, 5>);
    impl IfeiZoneBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IfeiZoneBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IfeiZoneBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod interiorlightspanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// CHART Light Dimmer
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct ChartDimmer(LimitedDial<30010, 65535, 0>);
    impl ChartDimmer {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ChartDimmer {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ChartDimmer {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CockkpitLightModeSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type CockkpitLightModeSwValueCallback<'a> = &'a mut dyn FnMut(CockkpitLightModeSwValue);
    impl core::convert::TryFrom<u16> for CockkpitLightModeSwValue {
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

    /// MODE Switch, NVG/NITE/DAY
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct CockkpitLightModeSw(Selector<29890, 768, 8>);
    impl CockkpitLightModeSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CockkpitLightModeSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CockkpitLightModeSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CONSOLES Lights Dimmer
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct ConsolesDimmer(LimitedDial<30004, 65535, 0>);
    impl ConsolesDimmer {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ConsolesDimmer {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ConsolesDimmer {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FLOOD Light Dimmer
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct FloodDimmer(LimitedDial<30008, 65535, 0>);
    impl FloodDimmer {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FloodDimmer {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FloodDimmer {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// INST PNL Dimmer
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct InstPnlDimmer(LimitedDial<30006, 65535, 0>);
    impl InstPnlDimmer {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for InstPnlDimmer {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for InstPnlDimmer {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LightsTestSwValue {
        Select1,
        Select2,
    }
    pub type LightsTestSwValueCallback<'a> = &'a mut dyn FnMut(LightsTestSwValue);
    impl core::convert::TryFrom<u16> for LightsTestSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Lights Test Switch, TEST/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LightsTestSw(Selector<29890, 1024, 10>);
    impl LightsTestSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LightsTestSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightsTestSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// WARN/CAUTION Light Dimmer
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct WarnCautionDimmer(LimitedDial<30012, 65535, 0>);
    impl WarnCautionDimmer {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for WarnCautionDimmer {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for WarnCautionDimmer {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod internalcanopyswitch {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Canopy Position
    /// Output: gauge position
    pub struct CanopyPos(AnalogGauge<30018, 65535, 0>);
    impl CanopyPos {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CanopyPos {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CanopyPos {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CanopySwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type CanopySwValueCallback<'a> = &'a mut dyn FnMut(CanopySwValue);
    impl core::convert::TryFrom<u16> for CanopySwValue {
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

    /// Canopy Control Switch, OPEN/HOLD/CLOSE
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct CanopySw(Selector<29896, 3072, 10>);
    impl CanopySw {
        pub const fn new() -> Self {
            Self(Selector::new())
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
}

pub mod ky58control {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ky58FillSelectValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
        Select7,
        Select8,
    }
    pub type Ky58FillSelectValueCallback<'a> = &'a mut dyn FnMut(Ky58FillSelectValue);
    impl core::convert::TryFrom<u16> for Ky58FillSelectValue {
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

    /// KY-58 Fill Select Knob, Z 1-5/1/2/3/4/5/6/Z ALL
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Ky58FillSelect(Selector<29894, 7168, 10>);
    impl Ky58FillSelect {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ky58FillSelect {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ky58FillSelect {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ky58ModeSelectValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type Ky58ModeSelectValueCallback<'a> = &'a mut dyn FnMut(Ky58ModeSelectValue);
    impl core::convert::TryFrom<u16> for Ky58ModeSelectValue {
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

    /// KY-58 Mode Select Knob, P/C/LD/RV
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Ky58ModeSelect(Selector<29894, 768, 8>);
    impl Ky58ModeSelect {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ky58ModeSelect {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ky58ModeSelect {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ky58PowerSelectValue {
        Select1,
        Select2,
        Select3,
    }
    pub type Ky58PowerSelectValueCallback<'a> = &'a mut dyn FnMut(Ky58PowerSelectValue);
    impl core::convert::TryFrom<u16> for Ky58PowerSelectValue {
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

    /// KY-58 Power Select Knob, OFF/ON/TD
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Ky58PowerSelect(Selector<29894, 24576, 13>);
    impl Ky58PowerSelect {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ky58PowerSelect {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ky58PowerSelect {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// KY-58 Volume Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct Ky58Volume(LimitedDial<30014, 65535, 0>);
    impl Ky58Volume {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ky58Volume {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ky58Volume {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod landinggearhandleandwarningtonesilence {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EmergencyGearRotateValue {
        Select1,
        Select2,
    }
    pub type EmergencyGearRotateValueCallback<'a> = &'a mut dyn FnMut(EmergencyGearRotateValue);
    impl core::convert::TryFrom<u16> for EmergencyGearRotateValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Emergency Gear Rotate
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EmergencyGearRotate(Selector<29816, 8192, 13>);
    impl EmergencyGearRotate {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EmergencyGearRotate {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EmergencyGearRotate {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GearDownlockOverrideBtnValue {
        Select1,
        Select2,
    }
    pub type GearDownlockOverrideBtnValueCallback<'a> =
        &'a mut dyn FnMut(GearDownlockOverrideBtnValue);
    impl core::convert::TryFrom<u16> for GearDownlockOverrideBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Landing Gear Override
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct GearDownlockOverrideBtn(Selector<29816, 16384, 14>);
    impl GearDownlockOverrideBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GearDownlockOverrideBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GearDownlockOverrideBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GearLeverValue {
        Select1,
        Select2,
    }
    pub type GearLeverValueCallback<'a> = &'a mut dyn FnMut(GearLeverValue);
    impl core::convert::TryFrom<u16> for GearLeverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Gear Lever
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct GearLever(Selector<29816, 4096, 12>);
    impl GearLever {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GearLever {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GearLever {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GearSilenceBtnValue {
        Select1,
        Select2,
    }
    pub type GearSilenceBtnValueCallback<'a> = &'a mut dyn FnMut(GearSilenceBtnValue);
    impl core::convert::TryFrom<u16> for GearSilenceBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Warning Tone Silence Button - Push to silence
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct GearSilenceBtn(Selector<29816, 32768, 15>);
    impl GearSilenceBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GearSilenceBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GearSilenceBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Landing Gear Handle Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct LandingGearHandleLt(Led<29816, 2048, 11>);
    impl LandingGearHandleLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LandingGearHandleLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LandingGearHandleLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod leftddi {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Brightness Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct LeftDdiBrtCtl(LimitedDial<29712, 65535, 0>);
    impl LeftDdiBrtCtl {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiBrtCtl {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiBrtCtl {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiBrtSelectValue {
        Select1,
        Select2,
        Select3,
    }
    pub type LeftDdiBrtSelectValueCallback<'a> = &'a mut dyn FnMut(LeftDdiBrtSelectValue);
    impl core::convert::TryFrom<u16> for LeftDdiBrtSelectValue {
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

    /// Brightness Selector Knob, OFF/NIGHT/DAY
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct LeftDdiBrtSelect(Selector<29710, 6, 1>);
    impl LeftDdiBrtSelect {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiBrtSelect {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiBrtSelect {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Contrast Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct LeftDdiContCtl(LimitedDial<29714, 65535, 0>);
    impl LeftDdiContCtl {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiContCtl {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiContCtl {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiPb01Value {
        Select1,
        Select2,
    }
    pub type LeftDdiPb01ValueCallback<'a> = &'a mut dyn FnMut(LeftDdiPb01Value);
    impl core::convert::TryFrom<u16> for LeftDdiPb01Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 1
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftDdiPb01(Selector<29710, 8, 3>);
    impl LeftDdiPb01 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiPb01 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiPb01 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiPb02Value {
        Select1,
        Select2,
    }
    pub type LeftDdiPb02ValueCallback<'a> = &'a mut dyn FnMut(LeftDdiPb02Value);
    impl core::convert::TryFrom<u16> for LeftDdiPb02Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftDdiPb02(Selector<29710, 16, 4>);
    impl LeftDdiPb02 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiPb02 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiPb02 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiPb03Value {
        Select1,
        Select2,
    }
    pub type LeftDdiPb03ValueCallback<'a> = &'a mut dyn FnMut(LeftDdiPb03Value);
    impl core::convert::TryFrom<u16> for LeftDdiPb03Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 3
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftDdiPb03(Selector<29710, 32, 5>);
    impl LeftDdiPb03 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiPb03 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiPb03 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiPb04Value {
        Select1,
        Select2,
    }
    pub type LeftDdiPb04ValueCallback<'a> = &'a mut dyn FnMut(LeftDdiPb04Value);
    impl core::convert::TryFrom<u16> for LeftDdiPb04Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 4
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftDdiPb04(Selector<29710, 64, 6>);
    impl LeftDdiPb04 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiPb04 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiPb04 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiPb05Value {
        Select1,
        Select2,
    }
    pub type LeftDdiPb05ValueCallback<'a> = &'a mut dyn FnMut(LeftDdiPb05Value);
    impl core::convert::TryFrom<u16> for LeftDdiPb05Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 5
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftDdiPb05(Selector<29710, 128, 7>);
    impl LeftDdiPb05 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiPb05 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiPb05 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiPb06Value {
        Select1,
        Select2,
    }
    pub type LeftDdiPb06ValueCallback<'a> = &'a mut dyn FnMut(LeftDdiPb06Value);
    impl core::convert::TryFrom<u16> for LeftDdiPb06Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 6
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftDdiPb06(Selector<29710, 256, 8>);
    impl LeftDdiPb06 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiPb06 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiPb06 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiPb07Value {
        Select1,
        Select2,
    }
    pub type LeftDdiPb07ValueCallback<'a> = &'a mut dyn FnMut(LeftDdiPb07Value);
    impl core::convert::TryFrom<u16> for LeftDdiPb07Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 7
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftDdiPb07(Selector<29710, 512, 9>);
    impl LeftDdiPb07 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiPb07 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiPb07 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiPb08Value {
        Select1,
        Select2,
    }
    pub type LeftDdiPb08ValueCallback<'a> = &'a mut dyn FnMut(LeftDdiPb08Value);
    impl core::convert::TryFrom<u16> for LeftDdiPb08Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 8
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftDdiPb08(Selector<29710, 1024, 10>);
    impl LeftDdiPb08 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiPb08 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiPb08 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiPb09Value {
        Select1,
        Select2,
    }
    pub type LeftDdiPb09ValueCallback<'a> = &'a mut dyn FnMut(LeftDdiPb09Value);
    impl core::convert::TryFrom<u16> for LeftDdiPb09Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 9
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftDdiPb09(Selector<29710, 2048, 11>);
    impl LeftDdiPb09 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiPb09 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiPb09 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiPb10Value {
        Select1,
        Select2,
    }
    pub type LeftDdiPb10ValueCallback<'a> = &'a mut dyn FnMut(LeftDdiPb10Value);
    impl core::convert::TryFrom<u16> for LeftDdiPb10Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 10
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftDdiPb10(Selector<29710, 4096, 12>);
    impl LeftDdiPb10 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiPb10 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiPb10 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiPb11Value {
        Select1,
        Select2,
    }
    pub type LeftDdiPb11ValueCallback<'a> = &'a mut dyn FnMut(LeftDdiPb11Value);
    impl core::convert::TryFrom<u16> for LeftDdiPb11Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 11
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftDdiPb11(Selector<29710, 8192, 13>);
    impl LeftDdiPb11 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiPb11 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiPb11 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiPb12Value {
        Select1,
        Select2,
    }
    pub type LeftDdiPb12ValueCallback<'a> = &'a mut dyn FnMut(LeftDdiPb12Value);
    impl core::convert::TryFrom<u16> for LeftDdiPb12Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 12
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftDdiPb12(Selector<29710, 16384, 14>);
    impl LeftDdiPb12 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiPb12 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiPb12 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiPb13Value {
        Select1,
        Select2,
    }
    pub type LeftDdiPb13ValueCallback<'a> = &'a mut dyn FnMut(LeftDdiPb13Value);
    impl core::convert::TryFrom<u16> for LeftDdiPb13Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 13
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftDdiPb13(Selector<29710, 32768, 15>);
    impl LeftDdiPb13 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiPb13 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiPb13 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiPb14Value {
        Select1,
        Select2,
    }
    pub type LeftDdiPb14ValueCallback<'a> = &'a mut dyn FnMut(LeftDdiPb14Value);
    impl core::convert::TryFrom<u16> for LeftDdiPb14Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 14
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftDdiPb14(Selector<29716, 1, 0>);
    impl LeftDdiPb14 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiPb14 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiPb14 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiPb15Value {
        Select1,
        Select2,
    }
    pub type LeftDdiPb15ValueCallback<'a> = &'a mut dyn FnMut(LeftDdiPb15Value);
    impl core::convert::TryFrom<u16> for LeftDdiPb15Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 15
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftDdiPb15(Selector<29716, 2, 1>);
    impl LeftDdiPb15 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiPb15 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiPb15 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiPb16Value {
        Select1,
        Select2,
    }
    pub type LeftDdiPb16ValueCallback<'a> = &'a mut dyn FnMut(LeftDdiPb16Value);
    impl core::convert::TryFrom<u16> for LeftDdiPb16Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 16
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftDdiPb16(Selector<29716, 4, 2>);
    impl LeftDdiPb16 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiPb16 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiPb16 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiPb17Value {
        Select1,
        Select2,
    }
    pub type LeftDdiPb17ValueCallback<'a> = &'a mut dyn FnMut(LeftDdiPb17Value);
    impl core::convert::TryFrom<u16> for LeftDdiPb17Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 17
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftDdiPb17(Selector<29716, 8, 3>);
    impl LeftDdiPb17 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiPb17 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiPb17 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiPb18Value {
        Select1,
        Select2,
    }
    pub type LeftDdiPb18ValueCallback<'a> = &'a mut dyn FnMut(LeftDdiPb18Value);
    impl core::convert::TryFrom<u16> for LeftDdiPb18Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 18
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftDdiPb18(Selector<29716, 16, 4>);
    impl LeftDdiPb18 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiPb18 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiPb18 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiPb19Value {
        Select1,
        Select2,
    }
    pub type LeftDdiPb19ValueCallback<'a> = &'a mut dyn FnMut(LeftDdiPb19Value);
    impl core::convert::TryFrom<u16> for LeftDdiPb19Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 19
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftDdiPb19(Selector<29716, 32, 5>);
    impl LeftDdiPb19 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiPb19 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiPb19 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftDdiPb20Value {
        Select1,
        Select2,
    }
    pub type LeftDdiPb20ValueCallback<'a> = &'a mut dyn FnMut(LeftDdiPb20Value);
    impl core::convert::TryFrom<u16> for LeftDdiPb20Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 20
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftDdiPb20(Selector<29716, 64, 6>);
    impl LeftDdiPb20 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftDdiPb20 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftDdiPb20 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod leftenginefirewarningextinguisherlight {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// FIRE LEFT
    /// Output: 0 if light is off, 1 if light is on
    pub struct FireLeftLt(Led<29704, 64, 6>);
    impl FireLeftLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for FireLeftLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FireLeftLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftFireBtnValue {
        Select1,
        Select2,
    }
    pub type LeftFireBtnValueCallback<'a> = &'a mut dyn FnMut(LeftFireBtnValue);
    impl core::convert::TryFrom<u16> for LeftFireBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Left Engine/AMAD Fire Warning/Extinguisher Light
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftFireBtn(Selector<29704, 128, 7>);
    impl LeftFireBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftFireBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftFireBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftFireBtnCoverValue {
        Select1,
        Select2,
    }
    pub type LeftFireBtnCoverValueCallback<'a> = &'a mut dyn FnMut(LeftFireBtnCoverValue);
    impl core::convert::TryFrom<u16> for LeftFireBtnCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Left Engine/AMAD Fire Warning Cover
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftFireBtnCover(Selector<29704, 256, 8>);
    impl LeftFireBtnCover {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftFireBtnCover {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftFireBtnCover {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod leftessentialcircuitbreakers {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbFcsChan1Value {
        Select1,
        Select2,
    }
    pub type CbFcsChan1ValueCallback<'a> = &'a mut dyn FnMut(CbFcsChan1Value);
    impl core::convert::TryFrom<u16> for CbFcsChan1Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CB FCS CHAN 1, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbFcsChan1(Selector<29884, 16384, 14>);
    impl CbFcsChan1 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbFcsChan1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbFcsChan1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbFcsChan2Value {
        Select1,
        Select2,
    }
    pub type CbFcsChan2ValueCallback<'a> = &'a mut dyn FnMut(CbFcsChan2Value);
    impl core::convert::TryFrom<u16> for CbFcsChan2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CB FCS CHAN 2, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbFcsChan2(Selector<29884, 32768, 15>);
    impl CbFcsChan2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbFcsChan2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbFcsChan2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbLaunchBarValue {
        Select1,
        Select2,
    }
    pub type CbLaunchBarValueCallback<'a> = &'a mut dyn FnMut(CbLaunchBarValue);
    impl core::convert::TryFrom<u16> for CbLaunchBarValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CB LAUNCH BAR, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbLaunchBar(Selector<29886, 512, 9>);
    impl CbLaunchBar {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbLaunchBar {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbLaunchBar {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbSpdBrkValue {
        Select1,
        Select2,
    }
    pub type CbSpdBrkValueCallback<'a> = &'a mut dyn FnMut(CbSpdBrkValue);
    impl core::convert::TryFrom<u16> for CbSpdBrkValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CB SPD BRK, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbSpdBrk(Selector<29886, 256, 8>);
    impl CbSpdBrk {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbSpdBrk {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbSpdBrk {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod lhadvisorypanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// ASPJ OH
    /// Output: 0 if light is off, 1 if light is on
    pub struct LhAdvAspjOh(Led<29706, 8, 3>);
    impl LhAdvAspjOh {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LhAdvAspjOh {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LhAdvAspjOh {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// GO
    /// Output: 0 if light is off, 1 if light is on
    pub struct LhAdvGo(Led<29706, 16, 4>);
    impl LhAdvGo {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LhAdvGo {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LhAdvGo {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// L BAR GREEN
    /// Output: 0 if light is off, 1 if light is on
    pub struct LhAdvLBarGreen(Led<29706, 2, 1>);
    impl LhAdvLBarGreen {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LhAdvLBarGreen {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LhAdvLBarGreen {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// L BAR RED
    /// Output: 0 if light is off, 1 if light is on
    pub struct LhAdvLBarRed(Led<29704, 32768, 15>);
    impl LhAdvLBarRed {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LhAdvLBarRed {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LhAdvLBarRed {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// L BLEED
    /// Output: 0 if light is off, 1 if light is on
    pub struct LhAdvLBleed(Led<29704, 2048, 11>);
    impl LhAdvLBleed {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LhAdvLBleed {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LhAdvLBleed {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// NO GO
    /// Output: 0 if light is off, 1 if light is on
    pub struct LhAdvNoGo(Led<29706, 32, 5>);
    impl LhAdvNoGo {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LhAdvNoGo {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LhAdvNoGo {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// R BLEED
    /// Output: 0 if light is off, 1 if light is on
    pub struct LhAdvRBleed(Led<29704, 4096, 12>);
    impl LhAdvRBleed {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LhAdvRBleed {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LhAdvRBleed {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// REC
    /// Output: 0 if light is off, 1 if light is on
    pub struct LhAdvRec(Led<29706, 1, 0>);
    impl LhAdvRec {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LhAdvRec {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LhAdvRec {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SPD BRK
    /// Output: 0 if light is off, 1 if light is on
    pub struct LhAdvSpdBrk(Led<29704, 8192, 13>);
    impl LhAdvSpdBrk {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LhAdvSpdBrk {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LhAdvSpdBrk {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// STBY
    /// Output: 0 if light is off, 1 if light is on
    pub struct LhAdvStby(Led<29704, 16384, 14>);
    impl LhAdvStby {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LhAdvStby {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LhAdvStby {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// XMIT
    /// Output: 0 if light is off, 1 if light is on
    pub struct LhAdvXmit(Led<29706, 4, 2>);
    impl LhAdvXmit {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LhAdvXmit {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LhAdvXmit {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod lockshootlights {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// LOCK
    /// Output: 0 if light is off, 1 if light is on
    pub struct LsLock(Led<29704, 1, 0>);
    impl LsLock {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LsLock {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LsLock {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SHOOT
    /// Output: 0 if light is off, 1 if light is on
    pub struct LsShoot(Led<29704, 2, 1>);
    impl LsShoot {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LsShoot {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LsShoot {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SHOOT STROBE
    /// Output: 0 if light is off, 1 if light is on
    pub struct LsShootStrobe(Led<29704, 4, 2>);
    impl LsShootStrobe {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LsShootStrobe {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LsShootStrobe {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod loxindicator {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

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

    /// OBOGS Control Switch, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ObogsSw(Selector<29880, 32768, 15>);
    impl ObogsSw {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// OXY Flow Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct OxyFlow(LimitedDial<29994, 65535, 0>);
    impl OxyFlow {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for OxyFlow {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for OxyFlow {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod mapgainspinrecoverypanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// HMD OFF/BRT Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct HmdOffBrt(LimitedDial<29782, 65535, 0>);
    impl HmdOffBrt {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HmdOffBrt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HmdOffBrt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IrCoolSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type IrCoolSwValueCallback<'a> = &'a mut dyn FnMut(IrCoolSwValue);
    impl core::convert::TryFrom<u16> for IrCoolSwValue {
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

    /// IR Cooling Switch, ORIDE/NORM/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IrCoolSw(Selector<29738, 49152, 14>);
    impl IrCoolSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IrCoolSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IrCoolSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Spin Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct SpinLt(Led<29738, 2048, 11>);
    impl SpinLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for SpinLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SpinLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SpinRecoveryCoverValue {
        Select1,
        Select2,
    }
    pub type SpinRecoveryCoverValueCallback<'a> = &'a mut dyn FnMut(SpinRecoveryCoverValue);
    impl core::convert::TryFrom<u16> for SpinRecoveryCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Spin Recovery Switch Cover, OPEN/CLOSE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SpinRecoveryCover(Selector<29738, 4096, 12>);
    impl SpinRecoveryCover {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SpinRecoveryCover {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SpinRecoveryCover {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SpinRecoverySwValue {
        Select1,
        Select2,
    }
    pub type SpinRecoverySwValueCallback<'a> = &'a mut dyn FnMut(SpinRecoverySwValue);
    impl core::convert::TryFrom<u16> for SpinRecoverySwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Spin Recovery Switch, RCVY/NORM
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SpinRecoverySw(Selector<29738, 8192, 13>);
    impl SpinRecoverySw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SpinRecoverySw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SpinRecoverySw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod masterarmpanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

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

    /// Master Arm Switch, ARM/SAFE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MasterArmSw(Selector<29708, 8192, 13>);
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
    pub enum MasterModeAaValue {
        Select1,
        Select2,
    }
    pub type MasterModeAaValueCallback<'a> = &'a mut dyn FnMut(MasterModeAaValue);
    impl core::convert::TryFrom<u16> for MasterModeAaValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Master Mode Button, A/A
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MasterModeAa(Selector<29708, 2048, 11>);
    impl MasterModeAa {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MasterModeAa {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MasterModeAa {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AA Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct MasterModeAaLt(Led<29708, 512, 9>);
    impl MasterModeAaLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for MasterModeAaLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MasterModeAaLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MasterModeAgValue {
        Select1,
        Select2,
    }
    pub type MasterModeAgValueCallback<'a> = &'a mut dyn FnMut(MasterModeAgValue);
    impl core::convert::TryFrom<u16> for MasterModeAgValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Master Mode Button, A/G
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MasterModeAg(Selector<29708, 4096, 12>);
    impl MasterModeAg {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MasterModeAg {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MasterModeAg {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AG Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct MasterModeAgLt(Led<29708, 1024, 10>);
    impl MasterModeAgLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for MasterModeAgLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MasterModeAgLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DISCH Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct McDisch(Led<29708, 16384, 14>);
    impl McDisch {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for McDisch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for McDisch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// READY Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct McReady(Led<29708, 32768, 15>);
    impl McReady {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for McReady {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for McReady {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod mastercautionlight {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// MASTER CAUTION
    /// Output: 0 if light is off, 1 if light is on
    pub struct MasterCautionLt(Led<29704, 512, 9>);
    impl MasterCautionLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for MasterCautionLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MasterCautionLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MasterCautionResetSwValue {
        Select1,
        Select2,
    }
    pub type MasterCautionResetSwValueCallback<'a> = &'a mut dyn FnMut(MasterCautionResetSwValue);
    impl core::convert::TryFrom<u16> for MasterCautionResetSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MASTER CAUTION Reset Button - Press to reset
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MasterCautionResetSw(Selector<29704, 1024, 10>);
    impl MasterCautionResetSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MasterCautionResetSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MasterCautionResetSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod missioncomputerandhydraulicisolatepanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HydIsolateOverrideSwValue {
        Select1,
        Select2,
    }
    pub type HydIsolateOverrideSwValueCallback<'a> = &'a mut dyn FnMut(HydIsolateOverrideSwValue);
    impl core::convert::TryFrom<u16> for HydIsolateOverrideSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Hydraulic Isolate Override Switch, NORM/ORIDE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct HydIsolateOverrideSw(Selector<29882, 1024, 10>);
    impl HydIsolateOverrideSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HydIsolateOverrideSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HydIsolateOverrideSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// MC Switch, 1 OFF/NORM/2 OFF
    /// Input: set the switch position -- 0 = emergency, 1 = parking
    /// Output: switch position -- 0 = emergency, 1 = parking
    pub struct McSw(Selector<29882, 768, 8>);
    impl McSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for McSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for McSw {
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

    /// Low Alt Warning
    /// Output: 0 if light is off, 1 if light is on
    pub struct LowAltWarnLt(Led<29854, 16384, 14>);
    impl LowAltWarnLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LowAltWarnLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LowAltWarnLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Altitude Pointer
    /// Output: gauge position
    pub struct RadaltAltPtr(AnalogGauge<29962, 65535, 0>);
    impl RadaltAltPtr {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadaltAltPtr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadaltAltPtr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Green Lamp
    /// Output: 0 if light is off, 1 if light is on
    pub struct RadaltGreenLamp(Led<29854, 32768, 15>);
    impl RadaltGreenLamp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RadaltGreenLamp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadaltGreenLamp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Set low altitude pointer
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct RadaltHeight(AnalogDial<29958, 65535, 0>);
    impl RadaltHeight {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadaltHeight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadaltHeight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Min Height Pointer
    /// Output: gauge position
    pub struct RadaltMinHeightPtr(AnalogGauge<29960, 65535, 0>);
    impl RadaltMinHeightPtr {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadaltMinHeightPtr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadaltMinHeightPtr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// OFF Flag
    /// Output: gauge position
    pub struct RadaltOffFlag(AnalogGauge<29964, 65535, 0>);
    impl RadaltOffFlag {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadaltOffFlag {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadaltOffFlag {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RadaltTestSwValue {
        Select1,
        Select2,
    }
    pub type RadaltTestSwValueCallback<'a> = &'a mut dyn FnMut(RadaltTestSwValue);
    impl core::convert::TryFrom<u16> for RadaltTestSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Push to Test Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RadaltTestSw(Selector<29854, 8192, 13>);
    impl RadaltTestSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadaltTestSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadaltTestSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod rhadvisorypanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// AAA
    /// Output: 0 if light is off, 1 if light is on
    pub struct RhAdvAaa(Led<29706, 2048, 11>);
    impl RhAdvAaa {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RhAdvAaa {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RhAdvAaa {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AI
    /// Output: 0 if light is off, 1 if light is on
    pub struct RhAdvAi(Led<29706, 1024, 10>);
    impl RhAdvAi {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RhAdvAi {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RhAdvAi {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CW
    /// Output: 0 if light is off, 1 if light is on
    pub struct RhAdvCw(Led<29706, 4096, 12>);
    impl RhAdvCw {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RhAdvCw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RhAdvCw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DISP
    /// Output: 0 if light is off, 1 if light is on
    pub struct RhAdvDisp(Led<29706, 256, 8>);
    impl RhAdvDisp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RhAdvDisp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RhAdvDisp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RCDR ON
    /// Output: 0 if light is off, 1 if light is on
    pub struct RhAdvRcdrOn(Led<29706, 128, 7>);
    impl RhAdvRcdrOn {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RhAdvRcdrOn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RhAdvRcdrOn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SAM
    /// Output: 0 if light is off, 1 if light is on
    pub struct RhAdvSam(Led<29706, 512, 9>);
    impl RhAdvSam {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RhAdvSam {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RhAdvSam {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SPARE RH1
    /// Output: 0 if light is off, 1 if light is on
    pub struct RhAdvSpareRh1(Led<29706, 8192, 13>);
    impl RhAdvSpareRh1 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RhAdvSpareRh1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RhAdvSpareRh1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SPARE RH2
    /// Output: 0 if light is off, 1 if light is on
    pub struct RhAdvSpareRh2(Led<29706, 16384, 14>);
    impl RhAdvSpareRh2 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RhAdvSpareRh2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RhAdvSpareRh2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SPARE RH3
    /// Output: 0 if light is off, 1 if light is on
    pub struct RhAdvSpareRh3(Led<29706, 32768, 15>);
    impl RhAdvSpareRh3 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RhAdvSpareRh3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RhAdvSpareRh3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SPARE RH4
    /// Output: 0 if light is off, 1 if light is on
    pub struct RhAdvSpareRh4(Led<29708, 1, 0>);
    impl RhAdvSpareRh4 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RhAdvSpareRh4 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RhAdvSpareRh4 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SPARE RH5
    /// Output: 0 if light is off, 1 if light is on
    pub struct RhAdvSpareRh5(Led<29708, 2, 1>);
    impl RhAdvSpareRh5 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RhAdvSpareRh5 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RhAdvSpareRh5 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod rightddi {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Brightness Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RightDdiBrtCtl(LimitedDial<29778, 65535, 0>);
    impl RightDdiBrtCtl {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiBrtCtl {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiBrtCtl {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightDdiBrtSelectValue {
        Select1,
        Select2,
        Select3,
    }
    pub type RightDdiBrtSelectValueCallback<'a> = &'a mut dyn FnMut(RightDdiBrtSelectValue);
    impl core::convert::TryFrom<u16> for RightDdiBrtSelectValue {
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

    /// Brightness Selector Knob, OFF/NIGHT/DAY
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct RightDdiBrtSelect(Selector<29720, 96, 5>);
    impl RightDdiBrtSelect {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiBrtSelect {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiBrtSelect {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Contrast Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RightDdiContCtl(LimitedDial<29780, 65535, 0>);
    impl RightDdiContCtl {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiContCtl {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiContCtl {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightDdiPb01Value {
        Select1,
        Select2,
    }
    pub type RightDdiPb01ValueCallback<'a> = &'a mut dyn FnMut(RightDdiPb01Value);
    impl core::convert::TryFrom<u16> for RightDdiPb01Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 1
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightDdiPb01(Selector<29720, 128, 7>);
    impl RightDdiPb01 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiPb01 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiPb01 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightDdiPb02Value {
        Select1,
        Select2,
    }
    pub type RightDdiPb02ValueCallback<'a> = &'a mut dyn FnMut(RightDdiPb02Value);
    impl core::convert::TryFrom<u16> for RightDdiPb02Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightDdiPb02(Selector<29720, 256, 8>);
    impl RightDdiPb02 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiPb02 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiPb02 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightDdiPb03Value {
        Select1,
        Select2,
    }
    pub type RightDdiPb03ValueCallback<'a> = &'a mut dyn FnMut(RightDdiPb03Value);
    impl core::convert::TryFrom<u16> for RightDdiPb03Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 3
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightDdiPb03(Selector<29720, 512, 9>);
    impl RightDdiPb03 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiPb03 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiPb03 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightDdiPb04Value {
        Select1,
        Select2,
    }
    pub type RightDdiPb04ValueCallback<'a> = &'a mut dyn FnMut(RightDdiPb04Value);
    impl core::convert::TryFrom<u16> for RightDdiPb04Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 4
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightDdiPb04(Selector<29720, 1024, 10>);
    impl RightDdiPb04 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiPb04 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiPb04 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightDdiPb05Value {
        Select1,
        Select2,
    }
    pub type RightDdiPb05ValueCallback<'a> = &'a mut dyn FnMut(RightDdiPb05Value);
    impl core::convert::TryFrom<u16> for RightDdiPb05Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 5
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightDdiPb05(Selector<29720, 2048, 11>);
    impl RightDdiPb05 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiPb05 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiPb05 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightDdiPb06Value {
        Select1,
        Select2,
    }
    pub type RightDdiPb06ValueCallback<'a> = &'a mut dyn FnMut(RightDdiPb06Value);
    impl core::convert::TryFrom<u16> for RightDdiPb06Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 6
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightDdiPb06(Selector<29720, 4096, 12>);
    impl RightDdiPb06 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiPb06 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiPb06 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightDdiPb07Value {
        Select1,
        Select2,
    }
    pub type RightDdiPb07ValueCallback<'a> = &'a mut dyn FnMut(RightDdiPb07Value);
    impl core::convert::TryFrom<u16> for RightDdiPb07Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 7
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightDdiPb07(Selector<29720, 8192, 13>);
    impl RightDdiPb07 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiPb07 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiPb07 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightDdiPb08Value {
        Select1,
        Select2,
    }
    pub type RightDdiPb08ValueCallback<'a> = &'a mut dyn FnMut(RightDdiPb08Value);
    impl core::convert::TryFrom<u16> for RightDdiPb08Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 8
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightDdiPb08(Selector<29720, 16384, 14>);
    impl RightDdiPb08 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiPb08 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiPb08 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightDdiPb09Value {
        Select1,
        Select2,
    }
    pub type RightDdiPb09ValueCallback<'a> = &'a mut dyn FnMut(RightDdiPb09Value);
    impl core::convert::TryFrom<u16> for RightDdiPb09Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 9
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightDdiPb09(Selector<29720, 32768, 15>);
    impl RightDdiPb09 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiPb09 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiPb09 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightDdiPb10Value {
        Select1,
        Select2,
    }
    pub type RightDdiPb10ValueCallback<'a> = &'a mut dyn FnMut(RightDdiPb10Value);
    impl core::convert::TryFrom<u16> for RightDdiPb10Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 10
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightDdiPb10(Selector<29736, 256, 8>);
    impl RightDdiPb10 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiPb10 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiPb10 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightDdiPb11Value {
        Select1,
        Select2,
    }
    pub type RightDdiPb11ValueCallback<'a> = &'a mut dyn FnMut(RightDdiPb11Value);
    impl core::convert::TryFrom<u16> for RightDdiPb11Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 11
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightDdiPb11(Selector<29736, 512, 9>);
    impl RightDdiPb11 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiPb11 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiPb11 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightDdiPb12Value {
        Select1,
        Select2,
    }
    pub type RightDdiPb12ValueCallback<'a> = &'a mut dyn FnMut(RightDdiPb12Value);
    impl core::convert::TryFrom<u16> for RightDdiPb12Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 12
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightDdiPb12(Selector<29736, 1024, 10>);
    impl RightDdiPb12 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiPb12 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiPb12 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightDdiPb13Value {
        Select1,
        Select2,
    }
    pub type RightDdiPb13ValueCallback<'a> = &'a mut dyn FnMut(RightDdiPb13Value);
    impl core::convert::TryFrom<u16> for RightDdiPb13Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 13
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightDdiPb13(Selector<29736, 2048, 11>);
    impl RightDdiPb13 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiPb13 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiPb13 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightDdiPb14Value {
        Select1,
        Select2,
    }
    pub type RightDdiPb14ValueCallback<'a> = &'a mut dyn FnMut(RightDdiPb14Value);
    impl core::convert::TryFrom<u16> for RightDdiPb14Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 14
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightDdiPb14(Selector<29736, 4096, 12>);
    impl RightDdiPb14 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiPb14 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiPb14 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightDdiPb15Value {
        Select1,
        Select2,
    }
    pub type RightDdiPb15ValueCallback<'a> = &'a mut dyn FnMut(RightDdiPb15Value);
    impl core::convert::TryFrom<u16> for RightDdiPb15Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 15
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightDdiPb15(Selector<29736, 8192, 13>);
    impl RightDdiPb15 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiPb15 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiPb15 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightDdiPb16Value {
        Select1,
        Select2,
    }
    pub type RightDdiPb16ValueCallback<'a> = &'a mut dyn FnMut(RightDdiPb16Value);
    impl core::convert::TryFrom<u16> for RightDdiPb16Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 16
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightDdiPb16(Selector<29736, 16384, 14>);
    impl RightDdiPb16 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiPb16 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiPb16 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightDdiPb17Value {
        Select1,
        Select2,
    }
    pub type RightDdiPb17ValueCallback<'a> = &'a mut dyn FnMut(RightDdiPb17Value);
    impl core::convert::TryFrom<u16> for RightDdiPb17Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 17
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightDdiPb17(Selector<29736, 32768, 15>);
    impl RightDdiPb17 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiPb17 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiPb17 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightDdiPb18Value {
        Select1,
        Select2,
    }
    pub type RightDdiPb18ValueCallback<'a> = &'a mut dyn FnMut(RightDdiPb18Value);
    impl core::convert::TryFrom<u16> for RightDdiPb18Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 18
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightDdiPb18(Selector<29738, 256, 8>);
    impl RightDdiPb18 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiPb18 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiPb18 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightDdiPb19Value {
        Select1,
        Select2,
    }
    pub type RightDdiPb19ValueCallback<'a> = &'a mut dyn FnMut(RightDdiPb19Value);
    impl core::convert::TryFrom<u16> for RightDdiPb19Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 19
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightDdiPb19(Selector<29738, 512, 9>);
    impl RightDdiPb19 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiPb19 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiPb19 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightDdiPb20Value {
        Select1,
        Select2,
    }
    pub type RightDdiPb20ValueCallback<'a> = &'a mut dyn FnMut(RightDdiPb20Value);
    impl core::convert::TryFrom<u16> for RightDdiPb20Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pushbutton 20
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightDdiPb20(Selector<29738, 1024, 10>);
    impl RightDdiPb20 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightDdiPb20 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightDdiPb20 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod rightenginefirewarningextinguisherlight {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// FIRE RIGHT
    /// Output: 0 if light is off, 1 if light is on
    pub struct FireRightLt(Led<29708, 16, 4>);
    impl FireRightLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for FireRightLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FireRightLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightFireBtnValue {
        Select1,
        Select2,
    }
    pub type RightFireBtnValueCallback<'a> = &'a mut dyn FnMut(RightFireBtnValue);
    impl core::convert::TryFrom<u16> for RightFireBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Right Engine/AMAD Fire Warning/Extinguisher Light
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightFireBtn(Selector<29708, 32, 5>);
    impl RightFireBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightFireBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightFireBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightFireBtnCoverValue {
        Select1,
        Select2,
    }
    pub type RightFireBtnCoverValueCallback<'a> = &'a mut dyn FnMut(RightFireBtnCoverValue);
    impl core::convert::TryFrom<u16> for RightFireBtnCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Right Engine/AMAD Fire Warning Cover
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightFireBtnCover(Selector<29708, 64, 6>);
    impl RightFireBtnCover {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightFireBtnCover {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightFireBtnCover {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod rightessentialcircuitbreakers {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbFcsChan3Value {
        Select1,
        Select2,
    }
    pub type CbFcsChan3ValueCallback<'a> = &'a mut dyn FnMut(CbFcsChan3Value);
    impl core::convert::TryFrom<u16> for CbFcsChan3Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CB FCS CHAN 3, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbFcsChan3(Selector<29892, 32768, 15>);
    impl CbFcsChan3 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbFcsChan3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbFcsChan3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbFcsChan4Value {
        Select1,
        Select2,
    }
    pub type CbFcsChan4ValueCallback<'a> = &'a mut dyn FnMut(CbFcsChan4Value);
    impl core::convert::TryFrom<u16> for CbFcsChan4Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CB FCS CHAN 4, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbFcsChan4(Selector<29894, 32768, 15>);
    impl CbFcsChan4 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbFcsChan4 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbFcsChan4 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbHoookValue {
        Select1,
        Select2,
    }
    pub type CbHoookValueCallback<'a> = &'a mut dyn FnMut(CbHoookValue);
    impl core::convert::TryFrom<u16> for CbHoookValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CB HOOK, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbHoook(Selector<29896, 4096, 12>);
    impl CbHoook {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbHoook {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbHoook {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbLgValue {
        Select1,
        Select2,
    }
    pub type CbLgValueCallback<'a> = &'a mut dyn FnMut(CbLgValue);
    impl core::convert::TryFrom<u16> for CbLgValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CB LG, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbLg(Selector<29896, 8192, 13>);
    impl CbLg {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbLg {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbLg {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FcsBitSwValue {
        Select1,
        Select2,
    }
    pub type FcsBitSwValueCallback<'a> = &'a mut dyn FnMut(FcsBitSwValue);
    impl core::convert::TryFrom<u16> for FcsBitSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FCS BIT Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FcsBitSw(Selector<29896, 16384, 14>);
    impl FcsBitSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FcsBitSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FcsBitSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod rudderpedaladjustlevel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RudderPedalAdjustValue {
        Select1,
        Select2,
    }
    pub type RudderPedalAdjustValueCallback<'a> = &'a mut dyn FnMut(RudderPedalAdjustValue);
    impl core::convert::TryFrom<u16> for RudderPedalAdjustValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Rudder Pedal Adjust Lever
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RudderPedalAdjust(Selector<29854, 4096, 12>);
    impl RudderPedalAdjust {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RudderPedalAdjust {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RudderPedalAdjust {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod rwrcontrolindicator {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrBitBtnValue {
        Select1,
        Select2,
    }
    pub type RwrBitBtnValueCallback<'a> = &'a mut dyn FnMut(RwrBitBtnValue);
    impl core::convert::TryFrom<u16> for RwrBitBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ALR-67 BIT Pushbutton
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrBitBtn(Selector<29826, 32768, 15>);
    impl RwrBitBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrBitBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrBitBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ALR-67 BIT Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct RwrBitLt(Led<29854, 2048, 11>);
    impl RwrBitLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RwrBitLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrBitLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrDisTypeSwValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type RwrDisTypeSwValueCallback<'a> = &'a mut dyn FnMut(RwrDisTypeSwValue);
    impl core::convert::TryFrom<u16> for RwrDisTypeSwValue {
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

    /// ALR-67 DIS TYPE Switch, N/I/A/U/F
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct RwrDisTypeSw(Selector<29850, 1792, 8>);
    impl RwrDisTypeSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrDisTypeSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrDisTypeSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrDisplayBtnValue {
        Select1,
        Select2,
    }
    pub type RwrDisplayBtnValueCallback<'a> = &'a mut dyn FnMut(RwrDisplayBtnValue);
    impl core::convert::TryFrom<u16> for RwrDisplayBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ALR-67 DISPLAY Pushbutton
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrDisplayBtn(Selector<29826, 4096, 12>);
    impl RwrDisplayBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrDisplayBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrDisplayBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ALR-67 DISPLAY Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct RwrDisplayLt(Led<29850, 8192, 13>);
    impl RwrDisplayLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RwrDisplayLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrDisplayLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ALR-67 DMR Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RwrDmrCtrl(LimitedDial<29944, 65535, 0>);
    impl RwrDmrCtrl {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrDmrCtrl {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrDmrCtrl {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ALR-67 ENABLE Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct RwrEnableLt(Led<29854, 256, 8>);
    impl RwrEnableLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RwrEnableLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrEnableLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ALR-67 FAIL Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct RwrFailLt(Led<29854, 1024, 10>);
    impl RwrFailLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RwrFailLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrFailLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ALR-67 LIMIT Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct RwrLimitLt(Led<29850, 4096, 12>);
    impl RwrLimitLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RwrLimitLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrLimitLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ALR-67 LOWER Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct RwrLowerLt(Led<29850, 2048, 11>);
    impl RwrLowerLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RwrLowerLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrLowerLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrOffsetBtnValue {
        Select1,
        Select2,
    }
    pub type RwrOffsetBtnValueCallback<'a> = &'a mut dyn FnMut(RwrOffsetBtnValue);
    impl core::convert::TryFrom<u16> for RwrOffsetBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ALR-67 OFFSET Pushbutton
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrOffsetBtn(Selector<29826, 16384, 14>);
    impl RwrOffsetBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrOffsetBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrOffsetBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ALR-67 OFFSET Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct RwrOffsetLt(Led<29854, 512, 9>);
    impl RwrOffsetLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RwrOffsetLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrOffsetLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrPowerBtnValue {
        Select1,
        Select2,
    }
    pub type RwrPowerBtnValueCallback<'a> = &'a mut dyn FnMut(RwrPowerBtnValue);
    impl core::convert::TryFrom<u16> for RwrPowerBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ALR-67 POWER Pushbutton
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrPowerBtn(Selector<29826, 2048, 11>);
    impl RwrPowerBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrPowerBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrPowerBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR Intensity Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RwrRwrIntesity(LimitedDial<29946, 65535, 0>);
    impl RwrRwrIntesity {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrRwrIntesity {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrRwrIntesity {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrSpecialBtnValue {
        Select1,
        Select2,
    }
    pub type RwrSpecialBtnValueCallback<'a> = &'a mut dyn FnMut(RwrSpecialBtnValue);
    impl core::convert::TryFrom<u16> for RwrSpecialBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ALR-67 SPECIAL Pushbutton
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrSpecialBtn(Selector<29826, 8192, 13>);
    impl RwrSpecialBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrSpecialBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrSpecialBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ALR-67 SPECIAL ENABLE Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct RwrSpecialEnLt(Led<29850, 16384, 14>);
    impl RwrSpecialEnLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RwrSpecialEnLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrSpecialEnLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ALR-67 SPECIAL Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct RwrSpecialLt(Led<29850, 32768, 15>);
    impl RwrSpecialLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RwrSpecialLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrSpecialLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod selectjettisonbutton {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

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

    /// Anti Skid
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AntiSkidSw(Selector<29818, 4096, 12>);
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
    pub enum FlapSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type FlapSwValueCallback<'a> = &'a mut dyn FnMut(FlapSwValue);
    impl core::convert::TryFrom<u16> for FlapSwValue {
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

    /// FLAP Switch, AUTO/HALF/FULL
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct FlapSw(Selector<29822, 768, 8>);
    impl FlapSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FlapSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlapSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HOOK BYPASS Switch, FIELD/CARRIER
    /// Input: set the switch position -- 0 = off, 1 = on
    /// Output: switch position -- 0 = off, 1 = on
    pub struct HookBypassSw(Selector<29818, 16384, 14>);
    impl HookBypassSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HookBypassSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HookBypassSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HYD Indicator Brake
    /// Output: gauge position
    pub struct HydIndBrake(AnalogGauge<29942, 65535, 0>);
    impl HydIndBrake {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HydIndBrake {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HydIndBrake {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Launch Bar
    /// Input: set the switch position -- 0 = off, 1 = on
    /// Output: switch position -- 0 = off, 1 = on
    pub struct LaunchBarSw(Selector<29818, 8192, 13>);
    impl LaunchBarSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LaunchBarSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LaunchBarSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LdgTaxiSwValue {
        Select1,
        Select2,
    }
    pub type LdgTaxiSwValueCallback<'a> = &'a mut dyn FnMut(LdgTaxiSwValue);
    impl core::convert::TryFrom<u16> for LdgTaxiSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// LDG/TAXI LIGHT Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LdgTaxiSw(Selector<29818, 32768, 15>);
    impl LdgTaxiSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LdgTaxiSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LdgTaxiSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SelJettBtnValue {
        Select1,
        Select2,
    }
    pub type SelJettBtnValueCallback<'a> = &'a mut dyn FnMut(SelJettBtnValue);
    impl core::convert::TryFrom<u16> for SelJettBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Selective Jettison Pushbutton
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SelJettBtn(Selector<29818, 256, 8>);
    impl SelJettBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SelJettBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SelJettBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SelJettKnobValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type SelJettKnobValueCallback<'a> = &'a mut dyn FnMut(SelJettKnobValue);
    impl core::convert::TryFrom<u16> for SelJettKnobValue {
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

    /// Selective Jettison Knob, L FUS MSL/SAFE/R FUS MSL/ RACK/LCHR /STORES
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct SelJettKnob(Selector<29818, 3584, 9>);
    impl SelJettKnob {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SelJettKnob {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SelJettKnob {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod sensorpanel {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FlirSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type FlirSwValueCallback<'a> = &'a mut dyn FnMut(FlirSwValue);
    impl core::convert::TryFrom<u16> for FlirSwValue {
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

    /// FLIR Switch, ON/STBY/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct FlirSw(Selector<29890, 6144, 11>);
    impl FlirSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FlirSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlirSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum InsSwValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
        Select7,
        Select8,
    }
    pub type InsSwValueCallback<'a> = &'a mut dyn FnMut(InsSwValue);
    impl core::convert::TryFrom<u16> for InsSwValue {
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

    /// INS Switch, OFF/CV/GND/NAV/IFA/GYRO/GB/TEST
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct InsSw(Selector<29892, 28672, 12>);
    impl InsSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for InsSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for InsSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LstNflrSwValue {
        Select1,
        Select2,
    }
    pub type LstNflrSwValueCallback<'a> = &'a mut dyn FnMut(LstNflrSwValue);
    impl core::convert::TryFrom<u16> for LstNflrSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// LST/NFLR Switch, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LstNflrSw(Selector<29890, 32768, 15>);
    impl LstNflrSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LstNflrSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LstNflrSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LtdRArmValue {
        Select1,
        Select2,
    }
    pub type LtdRArmValueCallback<'a> = &'a mut dyn FnMut(LtdRArmValue);
    impl core::convert::TryFrom<u16> for LtdRArmValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// LTD/R Switch, ARM/SAFE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LtdRArm(Selector<29892, 256, 8>);
    impl LtdRArm {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LtdRArm {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LtdRArm {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LtdRSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type LtdRSwValueCallback<'a> = &'a mut dyn FnMut(LtdRSwValue);
    impl core::convert::TryFrom<u16> for LtdRSwValue {
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

    /// LTD/R Switch, ---/SAFE/AFT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct LtdRSw(Selector<29890, 24576, 13>);
    impl LtdRSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LtdRSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LtdRSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RadarSwValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type RadarSwValueCallback<'a> = &'a mut dyn FnMut(RadarSwValue);
    impl core::convert::TryFrom<u16> for RadarSwValue {
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

    /// RADAR Switch Change ,OFF/STBY/OPR/EMERG(PULL)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct RadarSw(Selector<29892, 1536, 9>);
    impl RadarSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RadarSwPullValue {
        Select1,
        Select2,
    }
    pub type RadarSwPullValueCallback<'a> = &'a mut dyn FnMut(RadarSwPullValue);
    impl core::convert::TryFrom<u16> for RadarSwPullValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RADAR Switch Pull (MW to pull), OFF/STBY/OPR/EMERG(PULL)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RadarSwPull(Selector<29892, 2048, 11>);
    impl RadarSwPull {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarSwPull {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarSwPull {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod standbyairspeedindicator {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Airspeed
    /// Output: gauge position
    pub struct StbyAsiAirspeed(AnalogGauge<29920, 65535, 0>);
    impl StbyAsiAirspeed {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for StbyAsiAirspeed {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for StbyAsiAirspeed {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod standbyaltimeter {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// 10000 ft count
    /// Output: gauge position
    pub struct StbyAlt10000FtCnt(AnalogGauge<29926, 65535, 0>);
    impl StbyAlt10000FtCnt {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for StbyAlt10000FtCnt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for StbyAlt10000FtCnt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// 1000 ft count
    /// Output: gauge position
    pub struct StbyAlt1000FtCnt(AnalogGauge<29928, 65535, 0>);
    impl StbyAlt1000FtCnt {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for StbyAlt1000FtCnt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for StbyAlt1000FtCnt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// 100 ft pointer
    /// Output: gauge position
    pub struct StbyAlt100FtPtr(AnalogGauge<29924, 65535, 0>);
    impl StbyAlt100FtPtr {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for StbyAlt100FtPtr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for StbyAlt100FtPtr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pressure Setting Knob
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct StbyPressAlt(AnalogDial<29922, 65535, 0>);
    impl StbyPressAlt {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for StbyPressAlt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for StbyPressAlt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pressure Setting 1
    /// Output: gauge position
    pub struct StbyPressSet0(AnalogGauge<29930, 65535, 0>);
    impl StbyPressSet0 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for StbyPressSet0 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for StbyPressSet0 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pressure Setting 2
    /// Output: gauge position
    pub struct StbyPressSet1(AnalogGauge<29932, 65535, 0>);
    impl StbyPressSet1 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for StbyPressSet1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for StbyPressSet1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pressure Setting 3
    /// Output: gauge position
    pub struct StbyPressSet2(AnalogGauge<29934, 65535, 0>);
    impl StbyPressSet2 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for StbyPressSet2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for StbyPressSet2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod standbyattitudereferenceindicator {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Attitude Warning Flag
    /// Output: gauge position
    pub struct SaiAttWarningFlag(AnalogGauge<29912, 65535, 0>);
    impl SaiAttWarningFlag {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SaiAttWarningFlag {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SaiAttWarningFlag {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Bank
    /// Output: gauge position
    pub struct SaiBank(AnalogGauge<29910, 65535, 0>);
    impl SaiBank {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Pull to uncage
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SaiCage(Selector<29816, 1024, 10>);
    impl SaiCage {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// Manual Pitch Adjustment
    /// Output: gauge position
    pub struct SaiManPitchAdj(AnalogGauge<29914, 65535, 0>);
    impl SaiManPitchAdj {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SaiManPitchAdj {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SaiManPitchAdj {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pitch
    /// Output: gauge position
    pub struct SaiPitch(AnalogGauge<29908, 65535, 0>);
    impl SaiPitch {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
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

    /// Rate Of Turn
    /// Output: gauge position
    pub struct SaiRateOfTurn(AnalogGauge<29918, 65535, 0>);
    impl SaiRateOfTurn {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SaiRateOfTurn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SaiRateOfTurn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Adjust Attitude
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct SaiSet(AnalogDial<29906, 65535, 0>);
    impl SaiSet {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SaiSet {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SaiSet {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Slip Ball
    /// Output: gauge position
    pub struct SaiSlipBall(AnalogGauge<29916, 65535, 0>);
    impl SaiSlipBall {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SaiSlipBall {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SaiSlipBall {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SaiTestBtnValue {
        Select1,
        Select2,
    }
    pub type SaiTestBtnValueCallback<'a> = &'a mut dyn FnMut(SaiTestBtnValue);
    impl core::convert::TryFrom<u16> for SaiTestBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Test Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SaiTestBtn(Selector<29816, 512, 9>);
    impl SaiTestBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SaiTestBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SaiTestBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod standbyrateofclimbindicator {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Vertical Speed
    /// Output: gauge position
    pub struct Vsi(AnalogGauge<29936, 65535, 0>);
    impl Vsi {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Vsi {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Vsi {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod stationjettisonselect {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SjCtrValue {
        Select1,
        Select2,
    }
    pub type SjCtrValueCallback<'a> = &'a mut dyn FnMut(SjCtrValue);
    impl core::convert::TryFrom<u16> for SjCtrValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Station Jettison Select Button, CENTER
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SjCtr(Selector<29740, 32768, 15>);
    impl SjCtr {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SjCtr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SjCtr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CTR Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct SjCtrLt(Led<29742, 16384, 14>);
    impl SjCtrLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for SjCtrLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SjCtrLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SjLiValue {
        Select1,
        Select2,
    }
    pub type SjLiValueCallback<'a> = &'a mut dyn FnMut(SjLiValue);
    impl core::convert::TryFrom<u16> for SjLiValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Station Jettison Select Button, LEFT IN
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SjLi(Selector<29742, 1024, 10>);
    impl SjLi {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SjLi {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SjLi {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// LI Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct SjLiLt(Led<29742, 32768, 15>);
    impl SjLiLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for SjLiLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SjLiLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SjLoValue {
        Select1,
        Select2,
    }
    pub type SjLoValueCallback<'a> = &'a mut dyn FnMut(SjLoValue);
    impl core::convert::TryFrom<u16> for SjLoValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Station Jettison Select Button, LEFT OUT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SjLo(Selector<29742, 2048, 11>);
    impl SjLo {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SjLo {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SjLo {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// LO Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct SjLoLt(Led<29744, 256, 8>);
    impl SjLoLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for SjLoLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SjLoLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SjRiValue {
        Select1,
        Select2,
    }
    pub type SjRiValueCallback<'a> = &'a mut dyn FnMut(SjRiValue);
    impl core::convert::TryFrom<u16> for SjRiValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Station Jettison Select Button, RIGHT IN
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SjRi(Selector<29742, 4096, 12>);
    impl SjRi {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SjRi {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SjRi {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RI Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct SjRiLt(Led<29744, 512, 9>);
    impl SjRiLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for SjRiLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SjRiLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SjRoValue {
        Select1,
        Select2,
    }
    pub type SjRoValueCallback<'a> = &'a mut dyn FnMut(SjRoValue);
    impl core::convert::TryFrom<u16> for SjRoValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Station Jettison Select Button, RIGHT OUT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SjRo(Selector<29742, 8192, 13>);
    impl SjRo {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SjRo {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SjRo {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RO Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct SjRoLt(Led<29744, 1024, 10>);
    impl SjRoLt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for SjRoLt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SjRoLt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod throttlequadrant {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Throttles Friction Adjusting Lever
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct ThrottleFriction(LimitedDial<29970, 65535, 0>);
    impl ThrottleFriction {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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
}

pub mod todo {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftVideoBitValue {
        Select1,
        Select2,
    }
    pub type LeftVideoBitValueCallback<'a> = &'a mut dyn FnMut(LeftVideoBitValue);
    impl core::convert::TryFrom<u16> for LeftVideoBitValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Left Video Sensor BIT Initiate Pushbutton - Push to initiate BIT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftVideoBit(Selector<29898, 16384, 14>);
    impl LeftVideoBit {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftVideoBit {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftVideoBit {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightVideoBitValue {
        Select1,
        Select2,
    }
    pub type RightVideoBitValueCallback<'a> = &'a mut dyn FnMut(RightVideoBitValue);
    impl core::convert::TryFrom<u16> for RightVideoBitValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Right Video Sensor BIT Initiate Pushbutton - Push to initiate BIT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightVideoBit(Selector<29898, 32768, 15>);
    impl RightVideoBit {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightVideoBit {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightVideoBit {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod upfrontcontrollerufc {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ufc0Value {
        Select1,
        Select2,
    }
    pub type Ufc0ValueCallback<'a> = &'a mut dyn FnMut(Ufc0Value);
    impl core::convert::TryFrom<u16> for Ufc0Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UFC Keyboard Pushbutton, 0
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Ufc0(Selector<29718, 512, 9>);
    impl Ufc0 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ufc0 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ufc0 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ufc1Value {
        Select1,
        Select2,
    }
    pub type Ufc1ValueCallback<'a> = &'a mut dyn FnMut(Ufc1Value);
    impl core::convert::TryFrom<u16> for Ufc1Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UFC Keyboard Pushbutton, 1
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Ufc1(Selector<29718, 1024, 10>);
    impl Ufc1 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ufc1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ufc1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ufc2Value {
        Select1,
        Select2,
    }
    pub type Ufc2ValueCallback<'a> = &'a mut dyn FnMut(Ufc2Value);
    impl core::convert::TryFrom<u16> for Ufc2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UFC Keyboard Pushbutton, 2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Ufc2(Selector<29718, 2048, 11>);
    impl Ufc2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ufc2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ufc2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ufc3Value {
        Select1,
        Select2,
    }
    pub type Ufc3ValueCallback<'a> = &'a mut dyn FnMut(Ufc3Value);
    impl core::convert::TryFrom<u16> for Ufc3Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UFC Keyboard Pushbutton, 3
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Ufc3(Selector<29718, 4096, 12>);
    impl Ufc3 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ufc3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ufc3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ufc4Value {
        Select1,
        Select2,
    }
    pub type Ufc4ValueCallback<'a> = &'a mut dyn FnMut(Ufc4Value);
    impl core::convert::TryFrom<u16> for Ufc4Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UFC Keyboard Pushbutton, 4
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Ufc4(Selector<29718, 8192, 13>);
    impl Ufc4 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ufc4 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ufc4 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ufc5Value {
        Select1,
        Select2,
    }
    pub type Ufc5ValueCallback<'a> = &'a mut dyn FnMut(Ufc5Value);
    impl core::convert::TryFrom<u16> for Ufc5Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UFC Keyboard Pushbutton, 5
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Ufc5(Selector<29718, 16384, 14>);
    impl Ufc5 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ufc5 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ufc5 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ufc6Value {
        Select1,
        Select2,
    }
    pub type Ufc6ValueCallback<'a> = &'a mut dyn FnMut(Ufc6Value);
    impl core::convert::TryFrom<u16> for Ufc6Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UFC Keyboard Pushbutton, 6
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Ufc6(Selector<29718, 32768, 15>);
    impl Ufc6 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ufc6 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ufc6 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ufc7Value {
        Select1,
        Select2,
    }
    pub type Ufc7ValueCallback<'a> = &'a mut dyn FnMut(Ufc7Value);
    impl core::convert::TryFrom<u16> for Ufc7Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UFC Keyboard Pushbutton, 7
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Ufc7(Selector<29720, 1, 0>);
    impl Ufc7 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ufc7 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ufc7 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ufc8Value {
        Select1,
        Select2,
    }
    pub type Ufc8ValueCallback<'a> = &'a mut dyn FnMut(Ufc8Value);
    impl core::convert::TryFrom<u16> for Ufc8Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UFC Keyboard Pushbutton, 8
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Ufc8(Selector<29720, 2, 1>);
    impl Ufc8 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ufc8 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ufc8 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ufc9Value {
        Select1,
        Select2,
    }
    pub type Ufc9ValueCallback<'a> = &'a mut dyn FnMut(Ufc9Value);
    impl core::convert::TryFrom<u16> for Ufc9Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UFC Keyboard Pushbutton, 9
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Ufc9(Selector<29720, 4, 2>);
    impl Ufc9 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ufc9 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ufc9 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UfcAdfValue {
        Select1,
        Select2,
        Select3,
    }
    pub type UfcAdfValueCallback<'a> = &'a mut dyn FnMut(UfcAdfValue);
    impl core::convert::TryFrom<u16> for UfcAdfValue {
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

    /// ADF Function Select Switch, 1/OFF/2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct UfcAdf(Selector<29718, 192, 6>);
    impl UfcAdf {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UfcAdf {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcAdf {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UfcApValue {
        Select1,
        Select2,
    }
    pub type UfcApValueCallback<'a> = &'a mut dyn FnMut(UfcApValue);
    impl core::convert::TryFrom<u16> for UfcApValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Function Selector Pushbutton, A/P
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UfcAp(Selector<29716, 128, 7>);
    impl UfcAp {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UfcAp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcAp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UfcBcnValue {
        Select1,
        Select2,
    }
    pub type UfcBcnValueCallback<'a> = &'a mut dyn FnMut(UfcBcnValue);
    impl core::convert::TryFrom<u16> for UfcBcnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Function Selector Pushbutton, BCN
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UfcBcn(Selector<29716, 4096, 12>);
    impl UfcBcn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UfcBcn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcBcn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Brightness Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct UfcBrt(LimitedDial<29726, 65535, 0>);
    impl UfcBrt {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UfcBrt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcBrt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UfcClrValue {
        Select1,
        Select2,
    }
    pub type UfcClrValueCallback<'a> = &'a mut dyn FnMut(UfcClrValue);
    impl core::convert::TryFrom<u16> for UfcClrValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Keyboard Pushbutton, CLR
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UfcClr(Selector<29720, 8, 3>);
    impl UfcClr {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UfcClr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcClr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Comm 1 Display
    /// Output: Comm 1 Display
    pub struct UfcComm1Display(Display<29732, 2>);
    impl UfcComm1Display {
        pub const fn new() -> Self {
            Self(Display::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UfcComm1PullValue {
        Select1,
        Select2,
    }
    pub type UfcComm1PullValueCallback<'a> = &'a mut dyn FnMut(UfcComm1PullValue);
    impl core::convert::TryFrom<u16> for UfcComm1PullValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// COMM 1 Channel Selector Knob Pull
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UfcComm1Pull(Selector<29716, 16384, 14>);
    impl UfcComm1Pull {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UfcComm1Pull {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcComm1Pull {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// COMM 1 Volume Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct UfcComm1Vol(LimitedDial<29722, 65535, 0>);
    impl UfcComm1Vol {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UfcComm1Vol {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcComm1Vol {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Comm 2 Display
    /// Output: Comm 2 Display
    pub struct UfcComm2Display(Display<29734, 2>);
    impl UfcComm2Display {
        pub const fn new() -> Self {
            Self(Display::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UfcComm2PullValue {
        Select1,
        Select2,
    }
    pub type UfcComm2PullValueCallback<'a> = &'a mut dyn FnMut(UfcComm2PullValue);
    impl core::convert::TryFrom<u16> for UfcComm2PullValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// COMM 2 Channel Selector Knob Pull
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UfcComm2Pull(Selector<29716, 32768, 15>);
    impl UfcComm2Pull {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UfcComm2Pull {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcComm2Pull {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// COMM 2 Volume Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct UfcComm2Vol(LimitedDial<29724, 65535, 0>);
    impl UfcComm2Vol {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UfcComm2Vol {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcComm2Vol {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UfcDlValue {
        Select1,
        Select2,
    }
    pub type UfcDlValueCallback<'a> = &'a mut dyn FnMut(UfcDlValue);
    impl core::convert::TryFrom<u16> for UfcDlValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Function Selector Pushbutton, D/L
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UfcDl(Selector<29716, 2048, 11>);
    impl UfcDl {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UfcDl {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcDl {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UfcEmconValue {
        Select1,
        Select2,
    }
    pub type UfcEmconValueCallback<'a> = &'a mut dyn FnMut(UfcEmconValue);
    impl core::convert::TryFrom<u16> for UfcEmconValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Emission Control Pushbutton
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UfcEmcon(Selector<29718, 256, 8>);
    impl UfcEmcon {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UfcEmcon {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcEmcon {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UfcEntValue {
        Select1,
        Select2,
    }
    pub type UfcEntValueCallback<'a> = &'a mut dyn FnMut(UfcEntValue);
    impl core::convert::TryFrom<u16> for UfcEntValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Keyboard Pushbutton, ENT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UfcEnt(Selector<29720, 16, 4>);
    impl UfcEnt {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UfcEnt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcEnt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UfcIffValue {
        Select1,
        Select2,
    }
    pub type UfcIffValueCallback<'a> = &'a mut dyn FnMut(UfcIffValue);
    impl core::convert::TryFrom<u16> for UfcIffValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Function Selector Pushbutton, IFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UfcIff(Selector<29716, 256, 8>);
    impl UfcIff {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UfcIlsValue {
        Select1,
        Select2,
    }
    pub type UfcIlsValueCallback<'a> = &'a mut dyn FnMut(UfcIlsValue);
    impl core::convert::TryFrom<u16> for UfcIlsValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Function Selector Pushbutton, ILS
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UfcIls(Selector<29716, 1024, 10>);
    impl UfcIls {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UfcIls {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcIls {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UfcIpValue {
        Select1,
        Select2,
    }
    pub type UfcIpValueCallback<'a> = &'a mut dyn FnMut(UfcIpValue);
    impl core::convert::TryFrom<u16> for UfcIpValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// I/P Pushbutton
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UfcIp(Selector<29718, 32, 5>);
    impl UfcIp {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UfcOnoffValue {
        Select1,
        Select2,
    }
    pub type UfcOnoffValueCallback<'a> = &'a mut dyn FnMut(UfcOnoffValue);
    impl core::convert::TryFrom<u16> for UfcOnoffValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Function Selector Pushbutton, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UfcOnoff(Selector<29716, 8192, 13>);
    impl UfcOnoff {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// Option Cueing 1
    /// Output: Option Cueing 1
    pub struct UfcOptionCueing1(Display<29736, 1>);
    impl UfcOptionCueing1 {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for UfcOptionCueing1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcOptionCueing1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Option Cueing 2
    /// Output: Option Cueing 2
    pub struct UfcOptionCueing2(Display<29738, 1>);
    impl UfcOptionCueing2 {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for UfcOptionCueing2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcOptionCueing2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Option Cueing 3
    /// Output: Option Cueing 3
    pub struct UfcOptionCueing3(Display<29740, 1>);
    impl UfcOptionCueing3 {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for UfcOptionCueing3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcOptionCueing3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Option Cueing 4
    /// Output: Option Cueing 4
    pub struct UfcOptionCueing4(Display<29742, 1>);
    impl UfcOptionCueing4 {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for UfcOptionCueing4 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcOptionCueing4 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Option Cueing 5
    /// Output: Option Cueing 5
    pub struct UfcOptionCueing5(Display<29744, 1>);
    impl UfcOptionCueing5 {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for UfcOptionCueing5 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcOptionCueing5 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Option Display 1
    /// Output: Option Display 1
    pub struct UfcOptionDisplay1(Display<29746, 4>);
    impl UfcOptionDisplay1 {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for UfcOptionDisplay1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcOptionDisplay1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Option Display 2
    /// Output: Option Display 2
    pub struct UfcOptionDisplay2(Display<29750, 4>);
    impl UfcOptionDisplay2 {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for UfcOptionDisplay2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcOptionDisplay2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Option Display 3
    /// Output: Option Display 3
    pub struct UfcOptionDisplay3(Display<29754, 4>);
    impl UfcOptionDisplay3 {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for UfcOptionDisplay3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcOptionDisplay3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Option Display 4
    /// Output: Option Display 4
    pub struct UfcOptionDisplay4(Display<29758, 4>);
    impl UfcOptionDisplay4 {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for UfcOptionDisplay4 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcOptionDisplay4 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Option Display 5
    /// Output: Option Display 5
    pub struct UfcOptionDisplay5(Display<29762, 4>);
    impl UfcOptionDisplay5 {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for UfcOptionDisplay5 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcOptionDisplay5 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UfcOs1Value {
        Select1,
        Select2,
    }
    pub type UfcOs1ValueCallback<'a> = &'a mut dyn FnMut(UfcOs1Value);
    impl core::convert::TryFrom<u16> for UfcOs1Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Option Select Pushbutton 1
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UfcOs1(Selector<29718, 1, 0>);
    impl UfcOs1 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UfcOs1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcOs1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UfcOs2Value {
        Select1,
        Select2,
    }
    pub type UfcOs2ValueCallback<'a> = &'a mut dyn FnMut(UfcOs2Value);
    impl core::convert::TryFrom<u16> for UfcOs2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Option Select Pushbutton 2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UfcOs2(Selector<29718, 2, 1>);
    impl UfcOs2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UfcOs2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcOs2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UfcOs3Value {
        Select1,
        Select2,
    }
    pub type UfcOs3ValueCallback<'a> = &'a mut dyn FnMut(UfcOs3Value);
    impl core::convert::TryFrom<u16> for UfcOs3Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Option Select Pushbutton 3
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UfcOs3(Selector<29718, 4, 2>);
    impl UfcOs3 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UfcOs3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcOs3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UfcOs4Value {
        Select1,
        Select2,
    }
    pub type UfcOs4ValueCallback<'a> = &'a mut dyn FnMut(UfcOs4Value);
    impl core::convert::TryFrom<u16> for UfcOs4Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Option Select Pushbutton 4
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UfcOs4(Selector<29718, 8, 3>);
    impl UfcOs4 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UfcOs4 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcOs4 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UfcOs5Value {
        Select1,
        Select2,
    }
    pub type UfcOs5ValueCallback<'a> = &'a mut dyn FnMut(UfcOs5Value);
    impl core::convert::TryFrom<u16> for UfcOs5Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Option Select Pushbutton 5
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UfcOs5(Selector<29718, 16, 4>);
    impl UfcOs5 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UfcOs5 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcOs5 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Scratchpad Number Display
    /// Output: Scratchpad Number Display
    pub struct UfcScratchpadNumberDisplay(Display<29766, 8>);
    impl UfcScratchpadNumberDisplay {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for UfcScratchpadNumberDisplay {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcScratchpadNumberDisplay {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Scratchpad String 1 Display
    /// Output: Scratchpad String 1 Display
    pub struct UfcScratchpadString1Display(Display<29774, 2>);
    impl UfcScratchpadString1Display {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for UfcScratchpadString1Display {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcScratchpadString1Display {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Scratchpad String 2 Display
    /// Output: Scratchpad String 2 Display
    pub struct UfcScratchpadString2Display(Display<29776, 2>);
    impl UfcScratchpadString2Display {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for UfcScratchpadString2Display {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcScratchpadString2Display {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UfcTcnValue {
        Select1,
        Select2,
    }
    pub type UfcTcnValueCallback<'a> = &'a mut dyn FnMut(UfcTcnValue);
    impl core::convert::TryFrom<u16> for UfcTcnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Function Selector Pushbutton, TCN
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UfcTcn(Selector<29716, 512, 9>);
    impl UfcTcn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UfcTcn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcTcn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod wingfoldswitch {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WingFoldPullValue {
        Select1,
        Select2,
    }
    pub type WingFoldPullValueCallback<'a> = &'a mut dyn FnMut(WingFoldPullValue);
    impl core::convert::TryFrom<u16> for WingFoldPullValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Wing Fold Control Handle Pull
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct WingFoldPull(Selector<29858, 1024, 10>);
    impl WingFoldPull {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for WingFoldPull {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for WingFoldPull {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WingFoldRotateValue {
        Select1,
        Select2,
        Select3,
    }
    pub type WingFoldRotateValueCallback<'a> = &'a mut dyn FnMut(WingFoldRotateValue);
    impl core::convert::TryFrom<u16> for WingFoldRotateValue {
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

    /// Wing Fold Control Handle, FOLD/HOLD/UNFOLD
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct WingFoldRotate(Selector<29858, 6144, 11>);
    impl WingFoldRotate {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for WingFoldRotate {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for WingFoldRotate {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub enum RegisteredControl<'a> {
    AmpcdBrtCtl {
        control: ampcd::AmpcdBrtCtl,
        callback: IntegerCallback<'a>,
    },
    AmpcdContSw {
        control: ampcd::AmpcdContSw,
        callback: ampcd::AmpcdContSwValueCallback<'a>,
    },
    AmpcdGainSw {
        control: ampcd::AmpcdGainSw,
        callback: ampcd::AmpcdGainSwValueCallback<'a>,
    },
    AmpcdNightDay {
        control: ampcd::AmpcdNightDay,
        callback: ampcd::AmpcdNightDayValueCallback<'a>,
    },
    AmpcdPb01 {
        control: ampcd::AmpcdPb01,
        callback: ampcd::AmpcdPb01ValueCallback<'a>,
    },
    AmpcdPb02 {
        control: ampcd::AmpcdPb02,
        callback: ampcd::AmpcdPb02ValueCallback<'a>,
    },
    AmpcdPb03 {
        control: ampcd::AmpcdPb03,
        callback: ampcd::AmpcdPb03ValueCallback<'a>,
    },
    AmpcdPb04 {
        control: ampcd::AmpcdPb04,
        callback: ampcd::AmpcdPb04ValueCallback<'a>,
    },
    AmpcdPb05 {
        control: ampcd::AmpcdPb05,
        callback: ampcd::AmpcdPb05ValueCallback<'a>,
    },
    AmpcdPb06 {
        control: ampcd::AmpcdPb06,
        callback: ampcd::AmpcdPb06ValueCallback<'a>,
    },
    AmpcdPb07 {
        control: ampcd::AmpcdPb07,
        callback: ampcd::AmpcdPb07ValueCallback<'a>,
    },
    AmpcdPb08 {
        control: ampcd::AmpcdPb08,
        callback: ampcd::AmpcdPb08ValueCallback<'a>,
    },
    AmpcdPb09 {
        control: ampcd::AmpcdPb09,
        callback: ampcd::AmpcdPb09ValueCallback<'a>,
    },
    AmpcdPb10 {
        control: ampcd::AmpcdPb10,
        callback: ampcd::AmpcdPb10ValueCallback<'a>,
    },
    AmpcdPb11 {
        control: ampcd::AmpcdPb11,
        callback: ampcd::AmpcdPb11ValueCallback<'a>,
    },
    AmpcdPb12 {
        control: ampcd::AmpcdPb12,
        callback: ampcd::AmpcdPb12ValueCallback<'a>,
    },
    AmpcdPb13 {
        control: ampcd::AmpcdPb13,
        callback: ampcd::AmpcdPb13ValueCallback<'a>,
    },
    AmpcdPb14 {
        control: ampcd::AmpcdPb14,
        callback: ampcd::AmpcdPb14ValueCallback<'a>,
    },
    AmpcdPb15 {
        control: ampcd::AmpcdPb15,
        callback: ampcd::AmpcdPb15ValueCallback<'a>,
    },
    AmpcdPb16 {
        control: ampcd::AmpcdPb16,
        callback: ampcd::AmpcdPb16ValueCallback<'a>,
    },
    AmpcdPb17 {
        control: ampcd::AmpcdPb17,
        callback: ampcd::AmpcdPb17ValueCallback<'a>,
    },
    AmpcdPb18 {
        control: ampcd::AmpcdPb18,
        callback: ampcd::AmpcdPb18ValueCallback<'a>,
    },
    AmpcdPb19 {
        control: ampcd::AmpcdPb19,
        callback: ampcd::AmpcdPb19ValueCallback<'a>,
    },
    AmpcdPb20 {
        control: ampcd::AmpcdPb20,
        callback: ampcd::AmpcdPb20ValueCallback<'a>,
    },
    AmpcdSymSw {
        control: ampcd::AmpcdSymSw,
        callback: ampcd::AmpcdSymSwValueCallback<'a>,
    },
    AntiSkidSw {
        control: selectjettisonbutton::AntiSkidSw,
        callback: selectjettisonbutton::AntiSkidSwValueCallback<'a>,
    },
    AoaIndexerHigh {
        control: angleofattackindexerlights::AoaIndexerHigh,
        callback: LedCallback<'a>,
    },
    AoaIndexerLow {
        control: angleofattackindexerlights::AoaIndexerLow,
        callback: LedCallback<'a>,
    },
    AoaIndexerNormal {
        control: angleofattackindexerlights::AoaIndexerNormal,
        callback: LedCallback<'a>,
    },
    ApuControlSw {
        control: auxiliarypowerunitpanel::ApuControlSw,
        callback: IntegerCallback<'a>,
    },
    ApuFireBtn {
        control: apufirewarningextinguisherlight::ApuFireBtn,
        callback: apufirewarningextinguisherlight::ApuFireBtnValueCallback<'a>,
    },
    ApuReadyLt {
        control: auxiliarypowerunitpanel::ApuReadyLt,
        callback: LedCallback<'a>,
    },
    ArrestingHookLt {
        control: arrestinghookhandleandlight::ArrestingHookLt,
        callback: LedCallback<'a>,
    },
    AuxRelSw {
        control: dispenseremcpanel::AuxRelSw,
        callback: dispenseremcpanel::AuxRelSwValueCallback<'a>,
    },
    AvCoolSw {
        control: flightcomputercoolswitch::AvCoolSw,
        callback: flightcomputercoolswitch::AvCoolSwValueCallback<'a>,
    },
    BatterySw {
        control: electricalpowerpanel::BatterySw,
        callback: electricalpowerpanel::BatterySwValueCallback<'a>,
    },
    BleedAirKnob {
        control: environmentcontrolsystempanel::BleedAirKnob,
        callback: environmentcontrolsystempanel::BleedAirKnobValueCallback<'a>,
    },
    BleedAirPull {
        control: environmentcontrolsystempanel::BleedAirPull,
        callback: environmentcontrolsystempanel::BleedAirPullValueCallback<'a>,
    },
    CabinPressSw {
        control: environmentcontrolsystempanel::CabinPressSw,
        callback: environmentcontrolsystempanel::CabinPressSwValueCallback<'a>,
    },
    CabinTemp {
        control: environmentcontrolsystempanel::CabinTemp,
        callback: IntegerCallback<'a>,
    },
    CanopyJettHandlePull {
        control: canopyinternaljettisonhandle::CanopyJettHandlePull,
        callback: canopyinternaljettisonhandle::CanopyJettHandlePullValueCallback<'a>,
    },
    CanopyJettHandleUnlock {
        control: canopyinternaljettisonhandle::CanopyJettHandleUnlock,
        callback: canopyinternaljettisonhandle::CanopyJettHandleUnlockValueCallback<'a>,
    },
    CanopyPos {
        control: internalcanopyswitch::CanopyPos,
        callback: IntegerCallback<'a>,
    },
    CanopySw {
        control: internalcanopyswitch::CanopySw,
        callback: internalcanopyswitch::CanopySwValueCallback<'a>,
    },
    CbFcsChan1 {
        control: leftessentialcircuitbreakers::CbFcsChan1,
        callback: leftessentialcircuitbreakers::CbFcsChan1ValueCallback<'a>,
    },
    CbFcsChan2 {
        control: leftessentialcircuitbreakers::CbFcsChan2,
        callback: leftessentialcircuitbreakers::CbFcsChan2ValueCallback<'a>,
    },
    CbFcsChan3 {
        control: rightessentialcircuitbreakers::CbFcsChan3,
        callback: rightessentialcircuitbreakers::CbFcsChan3ValueCallback<'a>,
    },
    CbFcsChan4 {
        control: rightessentialcircuitbreakers::CbFcsChan4,
        callback: rightessentialcircuitbreakers::CbFcsChan4ValueCallback<'a>,
    },
    CbHoook {
        control: rightessentialcircuitbreakers::CbHoook,
        callback: rightessentialcircuitbreakers::CbHoookValueCallback<'a>,
    },
    CbLaunchBar {
        control: leftessentialcircuitbreakers::CbLaunchBar,
        callback: leftessentialcircuitbreakers::CbLaunchBarValueCallback<'a>,
    },
    CbLg {
        control: rightessentialcircuitbreakers::CbLg,
        callback: rightessentialcircuitbreakers::CbLgValueCallback<'a>,
    },
    CbSpdBrk {
        control: leftessentialcircuitbreakers::CbSpdBrk,
        callback: leftessentialcircuitbreakers::CbSpdBrkValueCallback<'a>,
    },
    ChartDimmer {
        control: interiorlightspanel::ChartDimmer,
        callback: IntegerCallback<'a>,
    },
    ClipApuAccLt {
        control: cautionlightpanel::ClipApuAccLt,
        callback: LedCallback<'a>,
    },
    ClipBattSwLt {
        control: cautionlightpanel::ClipBattSwLt,
        callback: LedCallback<'a>,
    },
    ClipCkSeatLt {
        control: cautionlightpanel::ClipCkSeatLt,
        callback: LedCallback<'a>,
    },
    ClipFcesLt {
        control: cautionlightpanel::ClipFcesLt,
        callback: LedCallback<'a>,
    },
    ClipFcsHotLt {
        control: cautionlightpanel::ClipFcsHotLt,
        callback: LedCallback<'a>,
    },
    ClipFuelLoLt {
        control: cautionlightpanel::ClipFuelLoLt,
        callback: LedCallback<'a>,
    },
    ClipGenTieLt {
        control: cautionlightpanel::ClipGenTieLt,
        callback: LedCallback<'a>,
    },
    ClipLGenLt {
        control: cautionlightpanel::ClipLGenLt,
        callback: LedCallback<'a>,
    },
    ClipRGenLt {
        control: cautionlightpanel::ClipRGenLt,
        callback: LedCallback<'a>,
    },
    ClipSpareCtn1Lt {
        control: cautionlightpanel::ClipSpareCtn1Lt,
        callback: LedCallback<'a>,
    },
    ClipSpareCtn2Lt {
        control: cautionlightpanel::ClipSpareCtn2Lt,
        callback: LedCallback<'a>,
    },
    ClipSpareCtn3Lt {
        control: cautionlightpanel::ClipSpareCtn3Lt,
        callback: LedCallback<'a>,
    },
    ClockElapsedMinutes {
        control: clock::ClockElapsedMinutes,
        callback: IntegerCallback<'a>,
    },
    ClockElapsedSeconds {
        control: clock::ClockElapsedSeconds,
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
    CmsdDispenseBtn {
        control: ecmdispenserbutton::CmsdDispenseBtn,
        callback: ecmdispenserbutton::CmsdDispenseBtnValueCallback<'a>,
    },
    CmsdDispenseSw {
        control: dispenseremcpanel::CmsdDispenseSw,
        callback: dispenseremcpanel::CmsdDispenseSwValueCallback<'a>,
    },
    CmsdJetSelBtn {
        control: dispenseremcpanel::CmsdJetSelBtn,
        callback: dispenseremcpanel::CmsdJetSelBtnValueCallback<'a>,
    },
    CockkpitLightModeSw {
        control: interiorlightspanel::CockkpitLightModeSw,
        callback: interiorlightspanel::CockkpitLightModeSwValueCallback<'a>,
    },
    ComAux {
        control: communicationpanel::ComAux,
        callback: IntegerCallback<'a>,
    },
    ComCommGXmtSw {
        control: communicationpanel::ComCommGXmtSw,
        callback: communicationpanel::ComCommGXmtSwValueCallback<'a>,
    },
    ComCommRelaySw {
        control: communicationpanel::ComCommRelaySw,
        callback: communicationpanel::ComCommRelaySwValueCallback<'a>,
    },
    ComCryptoSw {
        control: communicationpanel::ComCryptoSw,
        callback: communicationpanel::ComCryptoSwValueCallback<'a>,
    },
    ComIcs {
        control: communicationpanel::ComIcs,
        callback: IntegerCallback<'a>,
    },
    ComIffMasterSw {
        control: communicationpanel::ComIffMasterSw,
        callback: communicationpanel::ComIffMasterSwValueCallback<'a>,
    },
    ComIffMode4Sw {
        control: communicationpanel::ComIffMode4Sw,
        callback: communicationpanel::ComIffMode4SwValueCallback<'a>,
    },
    ComIlsChannelSw {
        control: communicationpanel::ComIlsChannelSw,
        callback: IntegerCallback<'a>,
    },
    ComIlsUfcManSw {
        control: communicationpanel::ComIlsUfcManSw,
        callback: communicationpanel::ComIlsUfcManSwValueCallback<'a>,
    },
    ComMidsA {
        control: communicationpanel::ComMidsA,
        callback: IntegerCallback<'a>,
    },
    ComMidsB {
        control: communicationpanel::ComMidsB,
        callback: IntegerCallback<'a>,
    },
    ComRwr {
        control: communicationpanel::ComRwr,
        callback: IntegerCallback<'a>,
    },
    ComTacan {
        control: communicationpanel::ComTacan,
        callback: IntegerCallback<'a>,
    },
    ComVox {
        control: communicationpanel::ComVox,
        callback: IntegerCallback<'a>,
    },
    ComWpn {
        control: communicationpanel::ComWpn,
        callback: IntegerCallback<'a>,
    },
    Comm1AntSelectSw {
        control: antennaselectpanel::Comm1AntSelectSw,
        callback: antennaselectpanel::Comm1AntSelectSwValueCallback<'a>,
    },
    Comm1ChannelNumeric {
        control: commsfrequency::Comm1ChannelNumeric,
        callback: IntegerCallback<'a>,
    },
    Comm1Freq {
        control: commsfrequency::Comm1Freq,
        callback: IntegerCallback<'a>,
    },
    Comm2ChannelNumeric {
        control: commsfrequency::Comm2ChannelNumeric,
        callback: IntegerCallback<'a>,
    },
    Comm2Freq {
        control: commsfrequency::Comm2Freq,
        callback: IntegerCallback<'a>,
    },
    ConsolesDimmer {
        control: interiorlightspanel::ConsolesDimmer,
        callback: IntegerCallback<'a>,
    },
    DefogHandle {
        control: defogpanel::DefogHandle,
        callback: IntegerCallback<'a>,
    },
    EcmModeSw {
        control: dispenseremcpanel::EcmModeSw,
        callback: dispenseremcpanel::EcmModeSwValueCallback<'a>,
    },
    EcsModeSw {
        control: environmentcontrolsystempanel::EcsModeSw,
        callback: environmentcontrolsystempanel::EcsModeSwValueCallback<'a>,
    },
    EjectionHandleSw {
        control: ejectionseat::EjectionHandleSw,
        callback: IntegerCallback<'a>,
    },
    EjectionSeatArmed {
        control: ejectionseat::EjectionSeatArmed,
        callback: ejectionseat::EjectionSeatArmedValueCallback<'a>,
    },
    EjectionSeatMnlOvrd {
        control: ejectionseat::EjectionSeatMnlOvrd,
        callback: ejectionseat::EjectionSeatMnlOvrdValueCallback<'a>,
    },
    EmerJettBtn {
        control: emergencyjettisonbutton::EmerJettBtn,
        callback: emergencyjettisonbutton::EmerJettBtnValueCallback<'a>,
    },
    EmergencyGearRotate {
        control: landinggearhandleandwarningtonesilence::EmergencyGearRotate,
        callback: landinggearhandleandwarningtonesilence::EmergencyGearRotateValueCallback<'a>,
    },
    EmergencyParkingBrakePull {
        control: emergencyandparkingbrakehandle::EmergencyParkingBrakePull,
        callback: emergencyandparkingbrakehandle::EmergencyParkingBrakePullValueCallback<'a>,
    },
    EmergencyParkingBrakeRotate {
        control: emergencyandparkingbrakehandle::EmergencyParkingBrakeRotate,
        callback: IntegerCallback<'a>,
    },
    EngAntiiceSw {
        control: environmentcontrolsystempanel::EngAntiiceSw,
        callback: environmentcontrolsystempanel::EngAntiiceSwValueCallback<'a>,
    },
    EngineCrankSw {
        control: auxiliarypowerunitpanel::EngineCrankSw,
        callback: auxiliarypowerunitpanel::EngineCrankSwValueCallback<'a>,
    },
    ExtCntTankSw {
        control: fuelpanel::ExtCntTankSw,
        callback: fuelpanel::ExtCntTankSwValueCallback<'a>,
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
    ExtPwrSw {
        control: groundpowerpanel::ExtPwrSw,
        callback: groundpowerpanel::ExtPwrSwValueCallback<'a>,
    },
    ExtSpeedBrake {
        control: externalaircraftmodel::ExtSpeedBrake,
        callback: IntegerCallback<'a>,
    },
    ExtStrobeLights {
        control: externalaircraftmodel::ExtStrobeLights,
        callback: IntegerCallback<'a>,
    },
    ExtWngTankSw {
        control: fuelpanel::ExtWngTankSw,
        callback: fuelpanel::ExtWngTankSwValueCallback<'a>,
    },
    FcsBitSw {
        control: rightessentialcircuitbreakers::FcsBitSw,
        callback: rightessentialcircuitbreakers::FcsBitSwValueCallback<'a>,
    },
    FcsResetBtn {
        control: flightcontrolsystempanel::FcsResetBtn,
        callback: flightcontrolsystempanel::FcsResetBtnValueCallback<'a>,
    },
    FireApuLt {
        control: apufirewarningextinguisherlight::FireApuLt,
        callback: LedCallback<'a>,
    },
    FireExtBtn {
        control: firesystems::FireExtBtn,
        callback: firesystems::FireExtBtnValueCallback<'a>,
    },
    FireLeftLt {
        control: leftenginefirewarningextinguisherlight::FireLeftLt,
        callback: LedCallback<'a>,
    },
    FireRightLt {
        control: rightenginefirewarningextinguisherlight::FireRightLt,
        callback: LedCallback<'a>,
    },
    FireTestSw {
        control: firetestpanel::FireTestSw,
        callback: firetestpanel::FireTestSwValueCallback<'a>,
    },
    FlapSw {
        control: selectjettisonbutton::FlapSw,
        callback: selectjettisonbutton::FlapSwValueCallback<'a>,
    },
    FlirSw {
        control: sensorpanel::FlirSw,
        callback: sensorpanel::FlirSwValueCallback<'a>,
    },
    FloodDimmer {
        control: interiorlightspanel::FloodDimmer,
        callback: IntegerCallback<'a>,
    },
    FlpLgFlapsLt {
        control: flapslandinggearstoresindicatorpanel::FlpLgFlapsLt,
        callback: LedCallback<'a>,
    },
    FlpLgFullFlapsLt {
        control: flapslandinggearstoresindicatorpanel::FlpLgFullFlapsLt,
        callback: LedCallback<'a>,
    },
    FlpLgHalfFlapsLt {
        control: flapslandinggearstoresindicatorpanel::FlpLgHalfFlapsLt,
        callback: LedCallback<'a>,
    },
    FlpLgLeftGearLt {
        control: flapslandinggearstoresindicatorpanel::FlpLgLeftGearLt,
        callback: LedCallback<'a>,
    },
    FlpLgNoseGearLt {
        control: flapslandinggearstoresindicatorpanel::FlpLgNoseGearLt,
        callback: LedCallback<'a>,
    },
    FlpLgRightGearLt {
        control: flapslandinggearstoresindicatorpanel::FlpLgRightGearLt,
        callback: LedCallback<'a>,
    },
    FormationDimmer {
        control: exteriorlightspanel::FormationDimmer,
        callback: IntegerCallback<'a>,
    },
    FuelDumpSw {
        control: fuelpanel::FuelDumpSw,
        callback: IntegerCallback<'a>,
    },
    GainSwitch {
        control: flightcontrolsystempanel::GainSwitch,
        callback: flightcontrolsystempanel::GainSwitchValueCallback<'a>,
    },
    GainSwitchCover {
        control: flightcontrolsystempanel::GainSwitchCover,
        callback: flightcontrolsystempanel::GainSwitchCoverValueCallback<'a>,
    },
    GearDownlockOverrideBtn {
        control: landinggearhandleandwarningtonesilence::GearDownlockOverrideBtn,
        callback: landinggearhandleandwarningtonesilence::GearDownlockOverrideBtnValueCallback<'a>,
    },
    GearLever {
        control: landinggearhandleandwarningtonesilence::GearLever,
        callback: landinggearhandleandwarningtonesilence::GearLeverValueCallback<'a>,
    },
    GearSilenceBtn {
        control: landinggearhandleandwarningtonesilence::GearSilenceBtn,
        callback: landinggearhandleandwarningtonesilence::GearSilenceBtnValueCallback<'a>,
    },
    GenTieCover {
        control: generatortiecontrolswitch::GenTieCover,
        callback: generatortiecontrolswitch::GenTieCoverValueCallback<'a>,
    },
    GenTieSw {
        control: generatortiecontrolswitch::GenTieSw,
        callback: generatortiecontrolswitch::GenTieSwValueCallback<'a>,
    },
    GndPwr1Sw {
        control: groundpowerpanel::GndPwr1Sw,
        callback: groundpowerpanel::GndPwr1SwValueCallback<'a>,
    },
    GndPwr2Sw {
        control: groundpowerpanel::GndPwr2Sw,
        callback: groundpowerpanel::GndPwr2SwValueCallback<'a>,
    },
    GndPwr3Sw {
        control: groundpowerpanel::GndPwr3Sw,
        callback: groundpowerpanel::GndPwr3SwValueCallback<'a>,
    },
    GndPwr4Sw {
        control: groundpowerpanel::GndPwr4Sw,
        callback: groundpowerpanel::GndPwr4SwValueCallback<'a>,
    },
    HideStickToggle {
        control: ejectionseat::HideStickToggle,
        callback: ejectionseat::HideStickToggleValueCallback<'a>,
    },
    HmdOffBrt {
        control: mapgainspinrecoverypanel::HmdOffBrt,
        callback: IntegerCallback<'a>,
    },
    HookBypassSw {
        control: selectjettisonbutton::HookBypassSw,
        callback: IntegerCallback<'a>,
    },
    HookLever {
        control: arrestinghookhandleandlight::HookLever,
        callback: arrestinghookhandleandlight::HookLeverValueCallback<'a>,
    },
    HudAltSw {
        control: hudcontrolpanel::HudAltSw,
        callback: hudcontrolpanel::HudAltSwValueCallback<'a>,
    },
    HudAoaIndexer {
        control: hudcontrolpanel::HudAoaIndexer,
        callback: IntegerCallback<'a>,
    },
    HudAttSw {
        control: hudcontrolpanel::HudAttSw,
        callback: hudcontrolpanel::HudAttSwValueCallback<'a>,
    },
    HudBalance {
        control: hudcontrolpanel::HudBalance,
        callback: IntegerCallback<'a>,
    },
    HudBlackLvl {
        control: hudcontrolpanel::HudBlackLvl,
        callback: IntegerCallback<'a>,
    },
    HudSymBrt {
        control: hudcontrolpanel::HudSymBrt,
        callback: IntegerCallback<'a>,
    },
    HudSymBrtSelect {
        control: hudcontrolpanel::HudSymBrtSelect,
        callback: hudcontrolpanel::HudSymBrtSelectValueCallback<'a>,
    },
    HudSymRejSw {
        control: hudcontrolpanel::HudSymRejSw,
        callback: hudcontrolpanel::HudSymRejSwValueCallback<'a>,
    },
    HudVideoBit {
        control: hudvideobitpanel::HudVideoBit,
        callback: hudvideobitpanel::HudVideoBitValueCallback<'a>,
    },
    HudVideoControlSw {
        control: hudcontrolpanel::HudVideoControlSw,
        callback: hudcontrolpanel::HudVideoControlSwValueCallback<'a>,
    },
    HydIndBrake {
        control: selectjettisonbutton::HydIndBrake,
        callback: IntegerCallback<'a>,
    },
    HydIndLeft {
        control: hyd1andhydpressureindicator::HydIndLeft,
        callback: IntegerCallback<'a>,
    },
    HydIndRight {
        control: hyd1andhydpressureindicator::HydIndRight,
        callback: IntegerCallback<'a>,
    },
    HydIsolateOverrideSw {
        control: missioncomputerandhydraulicisolatepanel::HydIsolateOverrideSw,
        callback: missioncomputerandhydraulicisolatepanel::HydIsolateOverrideSwValueCallback<'a>,
    },
    Ifei {
        control: hudvideorecordpanel::Ifei,
        callback: IntegerCallback<'a>,
    },
    IfeiBingo {
        control: integratedfuelengineindicatorifei::IfeiBingo,
        callback: DisplayCallback<'a>,
    },
    IfeiBingoTexture {
        control: integratedfuelengineindicatorifei::IfeiBingoTexture,
        callback: DisplayCallback<'a>,
    },
    IfeiClockH {
        control: integratedfuelengineindicatorifei::IfeiClockH,
        callback: DisplayCallback<'a>,
    },
    IfeiClockM {
        control: integratedfuelengineindicatorifei::IfeiClockM,
        callback: DisplayCallback<'a>,
    },
    IfeiClockS {
        control: integratedfuelengineindicatorifei::IfeiClockS,
        callback: DisplayCallback<'a>,
    },
    IfeiCodes {
        control: integratedfuelengineindicatorifei::IfeiCodes,
        callback: DisplayCallback<'a>,
    },
    IfeiDd1 {
        control: integratedfuelengineindicatorifei::IfeiDd1,
        callback: DisplayCallback<'a>,
    },
    IfeiDd2 {
        control: integratedfuelengineindicatorifei::IfeiDd2,
        callback: DisplayCallback<'a>,
    },
    IfeiDd3 {
        control: integratedfuelengineindicatorifei::IfeiDd3,
        callback: DisplayCallback<'a>,
    },
    IfeiDd4 {
        control: integratedfuelengineindicatorifei::IfeiDd4,
        callback: DisplayCallback<'a>,
    },
    IfeiDwnBtn {
        control: integratedfuelengineindicatorifei::IfeiDwnBtn,
        callback: integratedfuelengineindicatorifei::IfeiDwnBtnValueCallback<'a>,
    },
    IfeiEtBtn {
        control: integratedfuelengineindicatorifei::IfeiEtBtn,
        callback: integratedfuelengineindicatorifei::IfeiEtBtnValueCallback<'a>,
    },
    IfeiFfL {
        control: integratedfuelengineindicatorifei::IfeiFfL,
        callback: DisplayCallback<'a>,
    },
    IfeiFfR {
        control: integratedfuelengineindicatorifei::IfeiFfR,
        callback: DisplayCallback<'a>,
    },
    IfeiFfTexture {
        control: integratedfuelengineindicatorifei::IfeiFfTexture,
        callback: DisplayCallback<'a>,
    },
    IfeiFuelDown {
        control: integratedfuelengineindicatorifei::IfeiFuelDown,
        callback: DisplayCallback<'a>,
    },
    IfeiFuelUp {
        control: integratedfuelengineindicatorifei::IfeiFuelUp,
        callback: DisplayCallback<'a>,
    },
    IfeiL0Texture {
        control: integratedfuelengineindicatorifei::IfeiL0Texture,
        callback: DisplayCallback<'a>,
    },
    IfeiL100Texture {
        control: integratedfuelengineindicatorifei::IfeiL100Texture,
        callback: DisplayCallback<'a>,
    },
    IfeiL50Texture {
        control: integratedfuelengineindicatorifei::IfeiL50Texture,
        callback: DisplayCallback<'a>,
    },
    IfeiLpointerTexture {
        control: integratedfuelengineindicatorifei::IfeiLpointerTexture,
        callback: DisplayCallback<'a>,
    },
    IfeiLscaleTexture {
        control: integratedfuelengineindicatorifei::IfeiLscaleTexture,
        callback: DisplayCallback<'a>,
    },
    IfeiModeBtn {
        control: integratedfuelengineindicatorifei::IfeiModeBtn,
        callback: integratedfuelengineindicatorifei::IfeiModeBtnValueCallback<'a>,
    },
    IfeiOilPressL {
        control: integratedfuelengineindicatorifei::IfeiOilPressL,
        callback: DisplayCallback<'a>,
    },
    IfeiOilPressR {
        control: integratedfuelengineindicatorifei::IfeiOilPressR,
        callback: DisplayCallback<'a>,
    },
    IfeiOilTexture {
        control: integratedfuelengineindicatorifei::IfeiOilTexture,
        callback: DisplayCallback<'a>,
    },
    IfeiQtyBtn {
        control: integratedfuelengineindicatorifei::IfeiQtyBtn,
        callback: integratedfuelengineindicatorifei::IfeiQtyBtnValueCallback<'a>,
    },
    IfeiR100Texture {
        control: integratedfuelengineindicatorifei::IfeiR100Texture,
        callback: DisplayCallback<'a>,
    },
    IfeiR50Texture {
        control: integratedfuelengineindicatorifei::IfeiR50Texture,
        callback: DisplayCallback<'a>,
    },
    IfeiRpmL {
        control: integratedfuelengineindicatorifei::IfeiRpmL,
        callback: DisplayCallback<'a>,
    },
    IfeiRpmR {
        control: integratedfuelengineindicatorifei::IfeiRpmR,
        callback: DisplayCallback<'a>,
    },
    IfeiRpmTexture {
        control: integratedfuelengineindicatorifei::IfeiRpmTexture,
        callback: DisplayCallback<'a>,
    },
    IfeiRpointerTexture {
        control: integratedfuelengineindicatorifei::IfeiRpointerTexture,
        callback: DisplayCallback<'a>,
    },
    IfeiRscaleTexture {
        control: integratedfuelengineindicatorifei::IfeiRscaleTexture,
        callback: DisplayCallback<'a>,
    },
    IfeiSp {
        control: integratedfuelengineindicatorifei::IfeiSp,
        callback: DisplayCallback<'a>,
    },
    IfeiTempL {
        control: integratedfuelengineindicatorifei::IfeiTempL,
        callback: DisplayCallback<'a>,
    },
    IfeiTempR {
        control: integratedfuelengineindicatorifei::IfeiTempR,
        callback: DisplayCallback<'a>,
    },
    IfeiTempTexture {
        control: integratedfuelengineindicatorifei::IfeiTempTexture,
        callback: DisplayCallback<'a>,
    },
    IfeiTimeSetMode {
        control: integratedfuelengineindicatorifei::IfeiTimeSetMode,
        callback: DisplayCallback<'a>,
    },
    IfeiTimerH {
        control: integratedfuelengineindicatorifei::IfeiTimerH,
        callback: DisplayCallback<'a>,
    },
    IfeiTimerM {
        control: integratedfuelengineindicatorifei::IfeiTimerM,
        callback: DisplayCallback<'a>,
    },
    IfeiTimerS {
        control: integratedfuelengineindicatorifei::IfeiTimerS,
        callback: DisplayCallback<'a>,
    },
    IfeiUpBtn {
        control: integratedfuelengineindicatorifei::IfeiUpBtn,
        callback: integratedfuelengineindicatorifei::IfeiUpBtnValueCallback<'a>,
    },
    IfeiZTexture {
        control: integratedfuelengineindicatorifei::IfeiZTexture,
        callback: DisplayCallback<'a>,
    },
    IfeiZoneBtn {
        control: integratedfuelengineindicatorifei::IfeiZoneBtn,
        callback: integratedfuelengineindicatorifei::IfeiZoneBtnValueCallback<'a>,
    },
    IffAntSelectSw {
        control: antennaselectpanel::IffAntSelectSw,
        callback: antennaselectpanel::IffAntSelectSwValueCallback<'a>,
    },
    InsSw {
        control: sensorpanel::InsSw,
        callback: sensorpanel::InsSwValueCallback<'a>,
    },
    InstPnlDimmer {
        control: interiorlightspanel::InstPnlDimmer,
        callback: IntegerCallback<'a>,
    },
    IntWngTankSw {
        control: exteriorlightspanel::IntWngTankSw,
        callback: exteriorlightspanel::IntWngTankSwValueCallback<'a>,
    },
    IrCoolSw {
        control: mapgainspinrecoverypanel::IrCoolSw,
        callback: mapgainspinrecoverypanel::IrCoolSwValueCallback<'a>,
    },
    Ky58FillSelect {
        control: ky58control::Ky58FillSelect,
        callback: ky58control::Ky58FillSelectValueCallback<'a>,
    },
    Ky58ModeSelect {
        control: ky58control::Ky58ModeSelect,
        callback: ky58control::Ky58ModeSelectValueCallback<'a>,
    },
    Ky58PowerSelect {
        control: ky58control::Ky58PowerSelect,
        callback: ky58control::Ky58PowerSelectValueCallback<'a>,
    },
    Ky58Volume {
        control: ky58control::Ky58Volume,
        callback: IntegerCallback<'a>,
    },
    LGenSw {
        control: electricalpowerpanel::LGenSw,
        callback: electricalpowerpanel::LGenSwValueCallback<'a>,
    },
    LandingGearHandleLt {
        control: landinggearhandleandwarningtonesilence::LandingGearHandleLt,
        callback: LedCallback<'a>,
    },
    LaunchBarSw {
        control: selectjettisonbutton::LaunchBarSw,
        callback: IntegerCallback<'a>,
    },
    LdgTaxiSw {
        control: selectjettisonbutton::LdgTaxiSw,
        callback: selectjettisonbutton::LdgTaxiSwValueCallback<'a>,
    },
    LeftDdiBrtCtl {
        control: leftddi::LeftDdiBrtCtl,
        callback: IntegerCallback<'a>,
    },
    LeftDdiBrtSelect {
        control: leftddi::LeftDdiBrtSelect,
        callback: leftddi::LeftDdiBrtSelectValueCallback<'a>,
    },
    LeftDdiContCtl {
        control: leftddi::LeftDdiContCtl,
        callback: IntegerCallback<'a>,
    },
    LeftDdiCrsSw {
        control: headingandcoursesetswitches::LeftDdiCrsSw,
        callback: headingandcoursesetswitches::LeftDdiCrsSwValueCallback<'a>,
    },
    LeftDdiHdgSw {
        control: headingandcoursesetswitches::LeftDdiHdgSw,
        callback: headingandcoursesetswitches::LeftDdiHdgSwValueCallback<'a>,
    },
    LeftDdiPb01 {
        control: leftddi::LeftDdiPb01,
        callback: leftddi::LeftDdiPb01ValueCallback<'a>,
    },
    LeftDdiPb02 {
        control: leftddi::LeftDdiPb02,
        callback: leftddi::LeftDdiPb02ValueCallback<'a>,
    },
    LeftDdiPb03 {
        control: leftddi::LeftDdiPb03,
        callback: leftddi::LeftDdiPb03ValueCallback<'a>,
    },
    LeftDdiPb04 {
        control: leftddi::LeftDdiPb04,
        callback: leftddi::LeftDdiPb04ValueCallback<'a>,
    },
    LeftDdiPb05 {
        control: leftddi::LeftDdiPb05,
        callback: leftddi::LeftDdiPb05ValueCallback<'a>,
    },
    LeftDdiPb06 {
        control: leftddi::LeftDdiPb06,
        callback: leftddi::LeftDdiPb06ValueCallback<'a>,
    },
    LeftDdiPb07 {
        control: leftddi::LeftDdiPb07,
        callback: leftddi::LeftDdiPb07ValueCallback<'a>,
    },
    LeftDdiPb08 {
        control: leftddi::LeftDdiPb08,
        callback: leftddi::LeftDdiPb08ValueCallback<'a>,
    },
    LeftDdiPb09 {
        control: leftddi::LeftDdiPb09,
        callback: leftddi::LeftDdiPb09ValueCallback<'a>,
    },
    LeftDdiPb10 {
        control: leftddi::LeftDdiPb10,
        callback: leftddi::LeftDdiPb10ValueCallback<'a>,
    },
    LeftDdiPb11 {
        control: leftddi::LeftDdiPb11,
        callback: leftddi::LeftDdiPb11ValueCallback<'a>,
    },
    LeftDdiPb12 {
        control: leftddi::LeftDdiPb12,
        callback: leftddi::LeftDdiPb12ValueCallback<'a>,
    },
    LeftDdiPb13 {
        control: leftddi::LeftDdiPb13,
        callback: leftddi::LeftDdiPb13ValueCallback<'a>,
    },
    LeftDdiPb14 {
        control: leftddi::LeftDdiPb14,
        callback: leftddi::LeftDdiPb14ValueCallback<'a>,
    },
    LeftDdiPb15 {
        control: leftddi::LeftDdiPb15,
        callback: leftddi::LeftDdiPb15ValueCallback<'a>,
    },
    LeftDdiPb16 {
        control: leftddi::LeftDdiPb16,
        callback: leftddi::LeftDdiPb16ValueCallback<'a>,
    },
    LeftDdiPb17 {
        control: leftddi::LeftDdiPb17,
        callback: leftddi::LeftDdiPb17ValueCallback<'a>,
    },
    LeftDdiPb18 {
        control: leftddi::LeftDdiPb18,
        callback: leftddi::LeftDdiPb18ValueCallback<'a>,
    },
    LeftDdiPb19 {
        control: leftddi::LeftDdiPb19,
        callback: leftddi::LeftDdiPb19ValueCallback<'a>,
    },
    LeftDdiPb20 {
        control: leftddi::LeftDdiPb20,
        callback: leftddi::LeftDdiPb20ValueCallback<'a>,
    },
    LeftFireBtn {
        control: leftenginefirewarningextinguisherlight::LeftFireBtn,
        callback: leftenginefirewarningextinguisherlight::LeftFireBtnValueCallback<'a>,
    },
    LeftFireBtnCover {
        control: leftenginefirewarningextinguisherlight::LeftFireBtnCover,
        callback: leftenginefirewarningextinguisherlight::LeftFireBtnCoverValueCallback<'a>,
    },
    LeftLouver {
        control: environmentcontrollouver::LeftLouver,
        callback: IntegerCallback<'a>,
    },
    LeftVideoBit {
        control: todo::LeftVideoBit,
        callback: todo::LeftVideoBitValueCallback<'a>,
    },
    LhAdvAspjOh {
        control: lhadvisorypanel::LhAdvAspjOh,
        callback: LedCallback<'a>,
    },
    LhAdvGo {
        control: lhadvisorypanel::LhAdvGo,
        callback: LedCallback<'a>,
    },
    LhAdvLBarGreen {
        control: lhadvisorypanel::LhAdvLBarGreen,
        callback: LedCallback<'a>,
    },
    LhAdvLBarRed {
        control: lhadvisorypanel::LhAdvLBarRed,
        callback: LedCallback<'a>,
    },
    LhAdvLBleed {
        control: lhadvisorypanel::LhAdvLBleed,
        callback: LedCallback<'a>,
    },
    LhAdvNoGo {
        control: lhadvisorypanel::LhAdvNoGo,
        callback: LedCallback<'a>,
    },
    LhAdvRBleed {
        control: lhadvisorypanel::LhAdvRBleed,
        callback: LedCallback<'a>,
    },
    LhAdvRec {
        control: lhadvisorypanel::LhAdvRec,
        callback: LedCallback<'a>,
    },
    LhAdvSpdBrk {
        control: lhadvisorypanel::LhAdvSpdBrk,
        callback: LedCallback<'a>,
    },
    LhAdvStby {
        control: lhadvisorypanel::LhAdvStby,
        callback: LedCallback<'a>,
    },
    LhAdvXmit {
        control: lhadvisorypanel::LhAdvXmit,
        callback: LedCallback<'a>,
    },
    LightsTestSw {
        control: interiorlightspanel::LightsTestSw,
        callback: interiorlightspanel::LightsTestSwValueCallback<'a>,
    },
    LowAltWarnLt {
        control: radaraltimeter::LowAltWarnLt,
        callback: LedCallback<'a>,
    },
    LsLock {
        control: lockshootlights::LsLock,
        callback: LedCallback<'a>,
    },
    LsShoot {
        control: lockshootlights::LsShoot,
        callback: LedCallback<'a>,
    },
    LsShootStrobe {
        control: lockshootlights::LsShootStrobe,
        callback: LedCallback<'a>,
    },
    LstNflrSw {
        control: sensorpanel::LstNflrSw,
        callback: sensorpanel::LstNflrSwValueCallback<'a>,
    },
    LtdRArm {
        control: sensorpanel::LtdRArm,
        callback: sensorpanel::LtdRArmValueCallback<'a>,
    },
    LtdRSw {
        control: sensorpanel::LtdRSw,
        callback: sensorpanel::LtdRSwValueCallback<'a>,
    },
    MasterArmSw {
        control: masterarmpanel::MasterArmSw,
        callback: masterarmpanel::MasterArmSwValueCallback<'a>,
    },
    MasterCautionLt {
        control: mastercautionlight::MasterCautionLt,
        callback: LedCallback<'a>,
    },
    MasterCautionResetSw {
        control: mastercautionlight::MasterCautionResetSw,
        callback: mastercautionlight::MasterCautionResetSwValueCallback<'a>,
    },
    MasterModeAa {
        control: masterarmpanel::MasterModeAa,
        callback: masterarmpanel::MasterModeAaValueCallback<'a>,
    },
    MasterModeAaLt {
        control: masterarmpanel::MasterModeAaLt,
        callback: LedCallback<'a>,
    },
    MasterModeAg {
        control: masterarmpanel::MasterModeAg,
        callback: masterarmpanel::MasterModeAgValueCallback<'a>,
    },
    MasterModeAgLt {
        control: masterarmpanel::MasterModeAgLt,
        callback: LedCallback<'a>,
    },
    McDisch {
        control: masterarmpanel::McDisch,
        callback: LedCallback<'a>,
    },
    McReady {
        control: masterarmpanel::McReady,
        callback: LedCallback<'a>,
    },
    McSw {
        control: missioncomputerandhydraulicisolatepanel::McSw,
        callback: IntegerCallback<'a>,
    },
    ModeSelectorSw {
        control: hudvideorecordpanel::ModeSelectorSw,
        callback: hudvideorecordpanel::ModeSelectorSwValueCallback<'a>,
    },
    ObogsSw {
        control: loxindicator::ObogsSw,
        callback: loxindicator::ObogsSwValueCallback<'a>,
    },
    OxyFlow {
        control: loxindicator::OxyFlow,
        callback: IntegerCallback<'a>,
    },
    PitotHeatSw {
        control: environmentcontrolsystempanel::PitotHeatSw,
        callback: IntegerCallback<'a>,
    },
    PositionDimmer {
        control: exteriorlightspanel::PositionDimmer,
        callback: IntegerCallback<'a>,
    },
    PressureAlt {
        control: cockpitaltimeter::PressureAlt,
        callback: IntegerCallback<'a>,
    },
    ProbeSw {
        control: fuelpanel::ProbeSw,
        callback: fuelpanel::ProbeSwValueCallback<'a>,
    },
    RGenSw {
        control: electricalpowerpanel::RGenSw,
        callback: electricalpowerpanel::RGenSwValueCallback<'a>,
    },
    RadaltAltPtr {
        control: radaraltimeter::RadaltAltPtr,
        callback: IntegerCallback<'a>,
    },
    RadaltGreenLamp {
        control: radaraltimeter::RadaltGreenLamp,
        callback: LedCallback<'a>,
    },
    RadaltHeight {
        control: radaraltimeter::RadaltHeight,
        callback: IntegerCallback<'a>,
    },
    RadaltMinHeightPtr {
        control: radaraltimeter::RadaltMinHeightPtr,
        callback: IntegerCallback<'a>,
    },
    RadaltOffFlag {
        control: radaraltimeter::RadaltOffFlag,
        callback: IntegerCallback<'a>,
    },
    RadaltTestSw {
        control: radaraltimeter::RadaltTestSw,
        callback: radaraltimeter::RadaltTestSwValueCallback<'a>,
    },
    RadarSw {
        control: sensorpanel::RadarSw,
        callback: sensorpanel::RadarSwValueCallback<'a>,
    },
    RadarSwPull {
        control: sensorpanel::RadarSwPull,
        callback: sensorpanel::RadarSwPullValueCallback<'a>,
    },
    RhAdvAaa {
        control: rhadvisorypanel::RhAdvAaa,
        callback: LedCallback<'a>,
    },
    RhAdvAi {
        control: rhadvisorypanel::RhAdvAi,
        callback: LedCallback<'a>,
    },
    RhAdvCw {
        control: rhadvisorypanel::RhAdvCw,
        callback: LedCallback<'a>,
    },
    RhAdvDisp {
        control: rhadvisorypanel::RhAdvDisp,
        callback: LedCallback<'a>,
    },
    RhAdvRcdrOn {
        control: rhadvisorypanel::RhAdvRcdrOn,
        callback: LedCallback<'a>,
    },
    RhAdvSam {
        control: rhadvisorypanel::RhAdvSam,
        callback: LedCallback<'a>,
    },
    RhAdvSpareRh1 {
        control: rhadvisorypanel::RhAdvSpareRh1,
        callback: LedCallback<'a>,
    },
    RhAdvSpareRh2 {
        control: rhadvisorypanel::RhAdvSpareRh2,
        callback: LedCallback<'a>,
    },
    RhAdvSpareRh3 {
        control: rhadvisorypanel::RhAdvSpareRh3,
        callback: LedCallback<'a>,
    },
    RhAdvSpareRh4 {
        control: rhadvisorypanel::RhAdvSpareRh4,
        callback: LedCallback<'a>,
    },
    RhAdvSpareRh5 {
        control: rhadvisorypanel::RhAdvSpareRh5,
        callback: LedCallback<'a>,
    },
    RightDdiBrtCtl {
        control: rightddi::RightDdiBrtCtl,
        callback: IntegerCallback<'a>,
    },
    RightDdiBrtSelect {
        control: rightddi::RightDdiBrtSelect,
        callback: rightddi::RightDdiBrtSelectValueCallback<'a>,
    },
    RightDdiContCtl {
        control: rightddi::RightDdiContCtl,
        callback: IntegerCallback<'a>,
    },
    RightDdiPb01 {
        control: rightddi::RightDdiPb01,
        callback: rightddi::RightDdiPb01ValueCallback<'a>,
    },
    RightDdiPb02 {
        control: rightddi::RightDdiPb02,
        callback: rightddi::RightDdiPb02ValueCallback<'a>,
    },
    RightDdiPb03 {
        control: rightddi::RightDdiPb03,
        callback: rightddi::RightDdiPb03ValueCallback<'a>,
    },
    RightDdiPb04 {
        control: rightddi::RightDdiPb04,
        callback: rightddi::RightDdiPb04ValueCallback<'a>,
    },
    RightDdiPb05 {
        control: rightddi::RightDdiPb05,
        callback: rightddi::RightDdiPb05ValueCallback<'a>,
    },
    RightDdiPb06 {
        control: rightddi::RightDdiPb06,
        callback: rightddi::RightDdiPb06ValueCallback<'a>,
    },
    RightDdiPb07 {
        control: rightddi::RightDdiPb07,
        callback: rightddi::RightDdiPb07ValueCallback<'a>,
    },
    RightDdiPb08 {
        control: rightddi::RightDdiPb08,
        callback: rightddi::RightDdiPb08ValueCallback<'a>,
    },
    RightDdiPb09 {
        control: rightddi::RightDdiPb09,
        callback: rightddi::RightDdiPb09ValueCallback<'a>,
    },
    RightDdiPb10 {
        control: rightddi::RightDdiPb10,
        callback: rightddi::RightDdiPb10ValueCallback<'a>,
    },
    RightDdiPb11 {
        control: rightddi::RightDdiPb11,
        callback: rightddi::RightDdiPb11ValueCallback<'a>,
    },
    RightDdiPb12 {
        control: rightddi::RightDdiPb12,
        callback: rightddi::RightDdiPb12ValueCallback<'a>,
    },
    RightDdiPb13 {
        control: rightddi::RightDdiPb13,
        callback: rightddi::RightDdiPb13ValueCallback<'a>,
    },
    RightDdiPb14 {
        control: rightddi::RightDdiPb14,
        callback: rightddi::RightDdiPb14ValueCallback<'a>,
    },
    RightDdiPb15 {
        control: rightddi::RightDdiPb15,
        callback: rightddi::RightDdiPb15ValueCallback<'a>,
    },
    RightDdiPb16 {
        control: rightddi::RightDdiPb16,
        callback: rightddi::RightDdiPb16ValueCallback<'a>,
    },
    RightDdiPb17 {
        control: rightddi::RightDdiPb17,
        callback: rightddi::RightDdiPb17ValueCallback<'a>,
    },
    RightDdiPb18 {
        control: rightddi::RightDdiPb18,
        callback: rightddi::RightDdiPb18ValueCallback<'a>,
    },
    RightDdiPb19 {
        control: rightddi::RightDdiPb19,
        callback: rightddi::RightDdiPb19ValueCallback<'a>,
    },
    RightDdiPb20 {
        control: rightddi::RightDdiPb20,
        callback: rightddi::RightDdiPb20ValueCallback<'a>,
    },
    RightFireBtn {
        control: rightenginefirewarningextinguisherlight::RightFireBtn,
        callback: rightenginefirewarningextinguisherlight::RightFireBtnValueCallback<'a>,
    },
    RightFireBtnCover {
        control: rightenginefirewarningextinguisherlight::RightFireBtnCover,
        callback: rightenginefirewarningextinguisherlight::RightFireBtnCoverValueCallback<'a>,
    },
    RightLouver {
        control: environmentcontrollouver::RightLouver,
        callback: IntegerCallback<'a>,
    },
    RightVideoBit {
        control: todo::RightVideoBit,
        callback: todo::RightVideoBitValueCallback<'a>,
    },
    RudTrim {
        control: flightcontrolsystempanel::RudTrim,
        callback: IntegerCallback<'a>,
    },
    RudderPedalAdjust {
        control: rudderpedaladjustlevel::RudderPedalAdjust,
        callback: rudderpedaladjustlevel::RudderPedalAdjustValueCallback<'a>,
    },
    RwrBitBtn {
        control: rwrcontrolindicator::RwrBitBtn,
        callback: rwrcontrolindicator::RwrBitBtnValueCallback<'a>,
    },
    RwrBitLt {
        control: rwrcontrolindicator::RwrBitLt,
        callback: LedCallback<'a>,
    },
    RwrDisTypeSw {
        control: rwrcontrolindicator::RwrDisTypeSw,
        callback: rwrcontrolindicator::RwrDisTypeSwValueCallback<'a>,
    },
    RwrDisplayBtn {
        control: rwrcontrolindicator::RwrDisplayBtn,
        callback: rwrcontrolindicator::RwrDisplayBtnValueCallback<'a>,
    },
    RwrDisplayLt {
        control: rwrcontrolindicator::RwrDisplayLt,
        callback: LedCallback<'a>,
    },
    RwrDmrCtrl {
        control: rwrcontrolindicator::RwrDmrCtrl,
        callback: IntegerCallback<'a>,
    },
    RwrEnableLt {
        control: rwrcontrolindicator::RwrEnableLt,
        callback: LedCallback<'a>,
    },
    RwrFailLt {
        control: rwrcontrolindicator::RwrFailLt,
        callback: LedCallback<'a>,
    },
    RwrLimitLt {
        control: rwrcontrolindicator::RwrLimitLt,
        callback: LedCallback<'a>,
    },
    RwrLowerLt {
        control: rwrcontrolindicator::RwrLowerLt,
        callback: LedCallback<'a>,
    },
    RwrOffsetBtn {
        control: rwrcontrolindicator::RwrOffsetBtn,
        callback: rwrcontrolindicator::RwrOffsetBtnValueCallback<'a>,
    },
    RwrOffsetLt {
        control: rwrcontrolindicator::RwrOffsetLt,
        callback: LedCallback<'a>,
    },
    RwrPowerBtn {
        control: rwrcontrolindicator::RwrPowerBtn,
        callback: rwrcontrolindicator::RwrPowerBtnValueCallback<'a>,
    },
    RwrRwrIntesity {
        control: rwrcontrolindicator::RwrRwrIntesity,
        callback: IntegerCallback<'a>,
    },
    RwrSpecialBtn {
        control: rwrcontrolindicator::RwrSpecialBtn,
        callback: rwrcontrolindicator::RwrSpecialBtnValueCallback<'a>,
    },
    RwrSpecialEnLt {
        control: rwrcontrolindicator::RwrSpecialEnLt,
        callback: LedCallback<'a>,
    },
    RwrSpecialLt {
        control: rwrcontrolindicator::RwrSpecialLt,
        callback: LedCallback<'a>,
    },
    SaiAttWarningFlag {
        control: standbyattitudereferenceindicator::SaiAttWarningFlag,
        callback: IntegerCallback<'a>,
    },
    SaiBank {
        control: standbyattitudereferenceindicator::SaiBank,
        callback: IntegerCallback<'a>,
    },
    SaiCage {
        control: standbyattitudereferenceindicator::SaiCage,
        callback: standbyattitudereferenceindicator::SaiCageValueCallback<'a>,
    },
    SaiManPitchAdj {
        control: standbyattitudereferenceindicator::SaiManPitchAdj,
        callback: IntegerCallback<'a>,
    },
    SaiPitch {
        control: standbyattitudereferenceindicator::SaiPitch,
        callback: IntegerCallback<'a>,
    },
    SaiRateOfTurn {
        control: standbyattitudereferenceindicator::SaiRateOfTurn,
        callback: IntegerCallback<'a>,
    },
    SaiSet {
        control: standbyattitudereferenceindicator::SaiSet,
        callback: IntegerCallback<'a>,
    },
    SaiSlipBall {
        control: standbyattitudereferenceindicator::SaiSlipBall,
        callback: IntegerCallback<'a>,
    },
    SaiTestBtn {
        control: standbyattitudereferenceindicator::SaiTestBtn,
        callback: standbyattitudereferenceindicator::SaiTestBtnValueCallback<'a>,
    },
    SeatHeightSw {
        control: ejectionseat::SeatHeightSw,
        callback: ejectionseat::SeatHeightSwValueCallback<'a>,
    },
    SelJettBtn {
        control: selectjettisonbutton::SelJettBtn,
        callback: selectjettisonbutton::SelJettBtnValueCallback<'a>,
    },
    SelJettKnob {
        control: selectjettisonbutton::SelJettKnob,
        callback: selectjettisonbutton::SelJettKnobValueCallback<'a>,
    },
    SelectHmdLddiRddi {
        control: hudvideorecordpanel::SelectHmdLddiRddi,
        callback: hudvideorecordpanel::SelectHmdLddiRddiValueCallback<'a>,
    },
    SelectHudLddiRddi {
        control: hudvideorecordpanel::SelectHudLddiRddi,
        callback: hudvideorecordpanel::SelectHudLddiRddiValueCallback<'a>,
    },
    ShldrHarnessSw {
        control: ejectionseat::ShldrHarnessSw,
        callback: ejectionseat::ShldrHarnessSwValueCallback<'a>,
    },
    SjCtr {
        control: stationjettisonselect::SjCtr,
        callback: stationjettisonselect::SjCtrValueCallback<'a>,
    },
    SjCtrLt {
        control: stationjettisonselect::SjCtrLt,
        callback: LedCallback<'a>,
    },
    SjLi {
        control: stationjettisonselect::SjLi,
        callback: stationjettisonselect::SjLiValueCallback<'a>,
    },
    SjLiLt {
        control: stationjettisonselect::SjLiLt,
        callback: LedCallback<'a>,
    },
    SjLo {
        control: stationjettisonselect::SjLo,
        callback: stationjettisonselect::SjLoValueCallback<'a>,
    },
    SjLoLt {
        control: stationjettisonselect::SjLoLt,
        callback: LedCallback<'a>,
    },
    SjRi {
        control: stationjettisonselect::SjRi,
        callback: stationjettisonselect::SjRiValueCallback<'a>,
    },
    SjRiLt {
        control: stationjettisonselect::SjRiLt,
        callback: LedCallback<'a>,
    },
    SjRo {
        control: stationjettisonselect::SjRo,
        callback: stationjettisonselect::SjRoValueCallback<'a>,
    },
    SjRoLt {
        control: stationjettisonselect::SjRoLt,
        callback: LedCallback<'a>,
    },
    SpinLt {
        control: mapgainspinrecoverypanel::SpinLt,
        callback: LedCallback<'a>,
    },
    SpinRecoveryCover {
        control: mapgainspinrecoverypanel::SpinRecoveryCover,
        callback: mapgainspinrecoverypanel::SpinRecoveryCoverValueCallback<'a>,
    },
    SpinRecoverySw {
        control: mapgainspinrecoverypanel::SpinRecoverySw,
        callback: mapgainspinrecoverypanel::SpinRecoverySwValueCallback<'a>,
    },
    StbyAlt10000FtCnt {
        control: standbyaltimeter::StbyAlt10000FtCnt,
        callback: IntegerCallback<'a>,
    },
    StbyAlt1000FtCnt {
        control: standbyaltimeter::StbyAlt1000FtCnt,
        callback: IntegerCallback<'a>,
    },
    StbyAlt100FtPtr {
        control: standbyaltimeter::StbyAlt100FtPtr,
        callback: IntegerCallback<'a>,
    },
    StbyAsiAirspeed {
        control: standbyairspeedindicator::StbyAsiAirspeed,
        callback: IntegerCallback<'a>,
    },
    StbyPressAlt {
        control: standbyaltimeter::StbyPressAlt,
        callback: IntegerCallback<'a>,
    },
    StbyPressSet0 {
        control: standbyaltimeter::StbyPressSet0,
        callback: IntegerCallback<'a>,
    },
    StbyPressSet1 {
        control: standbyaltimeter::StbyPressSet1,
        callback: IntegerCallback<'a>,
    },
    StbyPressSet2 {
        control: standbyaltimeter::StbyPressSet2,
        callback: IntegerCallback<'a>,
    },
    StrobeSw {
        control: exteriorlightspanel::StrobeSw,
        callback: exteriorlightspanel::StrobeSwValueCallback<'a>,
    },
    SuitTemp {
        control: environmentcontrolsystempanel::SuitTemp,
        callback: IntegerCallback<'a>,
    },
    ThrottleFriction {
        control: throttlequadrant::ThrottleFriction,
        callback: IntegerCallback<'a>,
    },
    ToTrimBtn {
        control: flightcontrolsystempanel::ToTrimBtn,
        callback: flightcontrolsystempanel::ToTrimBtnValueCallback<'a>,
    },
    Ufc0 {
        control: upfrontcontrollerufc::Ufc0,
        callback: upfrontcontrollerufc::Ufc0ValueCallback<'a>,
    },
    Ufc1 {
        control: upfrontcontrollerufc::Ufc1,
        callback: upfrontcontrollerufc::Ufc1ValueCallback<'a>,
    },
    Ufc2 {
        control: upfrontcontrollerufc::Ufc2,
        callback: upfrontcontrollerufc::Ufc2ValueCallback<'a>,
    },
    Ufc3 {
        control: upfrontcontrollerufc::Ufc3,
        callback: upfrontcontrollerufc::Ufc3ValueCallback<'a>,
    },
    Ufc4 {
        control: upfrontcontrollerufc::Ufc4,
        callback: upfrontcontrollerufc::Ufc4ValueCallback<'a>,
    },
    Ufc5 {
        control: upfrontcontrollerufc::Ufc5,
        callback: upfrontcontrollerufc::Ufc5ValueCallback<'a>,
    },
    Ufc6 {
        control: upfrontcontrollerufc::Ufc6,
        callback: upfrontcontrollerufc::Ufc6ValueCallback<'a>,
    },
    Ufc7 {
        control: upfrontcontrollerufc::Ufc7,
        callback: upfrontcontrollerufc::Ufc7ValueCallback<'a>,
    },
    Ufc8 {
        control: upfrontcontrollerufc::Ufc8,
        callback: upfrontcontrollerufc::Ufc8ValueCallback<'a>,
    },
    Ufc9 {
        control: upfrontcontrollerufc::Ufc9,
        callback: upfrontcontrollerufc::Ufc9ValueCallback<'a>,
    },
    UfcAdf {
        control: upfrontcontrollerufc::UfcAdf,
        callback: upfrontcontrollerufc::UfcAdfValueCallback<'a>,
    },
    UfcAp {
        control: upfrontcontrollerufc::UfcAp,
        callback: upfrontcontrollerufc::UfcApValueCallback<'a>,
    },
    UfcBcn {
        control: upfrontcontrollerufc::UfcBcn,
        callback: upfrontcontrollerufc::UfcBcnValueCallback<'a>,
    },
    UfcBrt {
        control: upfrontcontrollerufc::UfcBrt,
        callback: IntegerCallback<'a>,
    },
    UfcClr {
        control: upfrontcontrollerufc::UfcClr,
        callback: upfrontcontrollerufc::UfcClrValueCallback<'a>,
    },
    UfcComm1Display {
        control: upfrontcontrollerufc::UfcComm1Display,
        callback: DisplayCallback<'a>,
    },
    UfcComm1Pull {
        control: upfrontcontrollerufc::UfcComm1Pull,
        callback: upfrontcontrollerufc::UfcComm1PullValueCallback<'a>,
    },
    UfcComm1Vol {
        control: upfrontcontrollerufc::UfcComm1Vol,
        callback: IntegerCallback<'a>,
    },
    UfcComm2Display {
        control: upfrontcontrollerufc::UfcComm2Display,
        callback: DisplayCallback<'a>,
    },
    UfcComm2Pull {
        control: upfrontcontrollerufc::UfcComm2Pull,
        callback: upfrontcontrollerufc::UfcComm2PullValueCallback<'a>,
    },
    UfcComm2Vol {
        control: upfrontcontrollerufc::UfcComm2Vol,
        callback: IntegerCallback<'a>,
    },
    UfcDl {
        control: upfrontcontrollerufc::UfcDl,
        callback: upfrontcontrollerufc::UfcDlValueCallback<'a>,
    },
    UfcEmcon {
        control: upfrontcontrollerufc::UfcEmcon,
        callback: upfrontcontrollerufc::UfcEmconValueCallback<'a>,
    },
    UfcEnt {
        control: upfrontcontrollerufc::UfcEnt,
        callback: upfrontcontrollerufc::UfcEntValueCallback<'a>,
    },
    UfcIff {
        control: upfrontcontrollerufc::UfcIff,
        callback: upfrontcontrollerufc::UfcIffValueCallback<'a>,
    },
    UfcIls {
        control: upfrontcontrollerufc::UfcIls,
        callback: upfrontcontrollerufc::UfcIlsValueCallback<'a>,
    },
    UfcIp {
        control: upfrontcontrollerufc::UfcIp,
        callback: upfrontcontrollerufc::UfcIpValueCallback<'a>,
    },
    UfcOnoff {
        control: upfrontcontrollerufc::UfcOnoff,
        callback: upfrontcontrollerufc::UfcOnoffValueCallback<'a>,
    },
    UfcOptionCueing1 {
        control: upfrontcontrollerufc::UfcOptionCueing1,
        callback: DisplayCallback<'a>,
    },
    UfcOptionCueing2 {
        control: upfrontcontrollerufc::UfcOptionCueing2,
        callback: DisplayCallback<'a>,
    },
    UfcOptionCueing3 {
        control: upfrontcontrollerufc::UfcOptionCueing3,
        callback: DisplayCallback<'a>,
    },
    UfcOptionCueing4 {
        control: upfrontcontrollerufc::UfcOptionCueing4,
        callback: DisplayCallback<'a>,
    },
    UfcOptionCueing5 {
        control: upfrontcontrollerufc::UfcOptionCueing5,
        callback: DisplayCallback<'a>,
    },
    UfcOptionDisplay1 {
        control: upfrontcontrollerufc::UfcOptionDisplay1,
        callback: DisplayCallback<'a>,
    },
    UfcOptionDisplay2 {
        control: upfrontcontrollerufc::UfcOptionDisplay2,
        callback: DisplayCallback<'a>,
    },
    UfcOptionDisplay3 {
        control: upfrontcontrollerufc::UfcOptionDisplay3,
        callback: DisplayCallback<'a>,
    },
    UfcOptionDisplay4 {
        control: upfrontcontrollerufc::UfcOptionDisplay4,
        callback: DisplayCallback<'a>,
    },
    UfcOptionDisplay5 {
        control: upfrontcontrollerufc::UfcOptionDisplay5,
        callback: DisplayCallback<'a>,
    },
    UfcOs1 {
        control: upfrontcontrollerufc::UfcOs1,
        callback: upfrontcontrollerufc::UfcOs1ValueCallback<'a>,
    },
    UfcOs2 {
        control: upfrontcontrollerufc::UfcOs2,
        callback: upfrontcontrollerufc::UfcOs2ValueCallback<'a>,
    },
    UfcOs3 {
        control: upfrontcontrollerufc::UfcOs3,
        callback: upfrontcontrollerufc::UfcOs3ValueCallback<'a>,
    },
    UfcOs4 {
        control: upfrontcontrollerufc::UfcOs4,
        callback: upfrontcontrollerufc::UfcOs4ValueCallback<'a>,
    },
    UfcOs5 {
        control: upfrontcontrollerufc::UfcOs5,
        callback: upfrontcontrollerufc::UfcOs5ValueCallback<'a>,
    },
    UfcScratchpadNumberDisplay {
        control: upfrontcontrollerufc::UfcScratchpadNumberDisplay,
        callback: DisplayCallback<'a>,
    },
    UfcScratchpadString1Display {
        control: upfrontcontrollerufc::UfcScratchpadString1Display,
        callback: DisplayCallback<'a>,
    },
    UfcScratchpadString2Display {
        control: upfrontcontrollerufc::UfcScratchpadString2Display,
        callback: DisplayCallback<'a>,
    },
    UfcTcn {
        control: upfrontcontrollerufc::UfcTcn,
        callback: upfrontcontrollerufc::UfcTcnValueCallback<'a>,
    },
    VoltE {
        control: electricalpowerpanel::VoltE,
        callback: IntegerCallback<'a>,
    },
    VoltU {
        control: electricalpowerpanel::VoltU,
        callback: IntegerCallback<'a>,
    },
    Vsi {
        control: standbyrateofclimbindicator::Vsi,
        callback: IntegerCallback<'a>,
    },
    WarnCautionDimmer {
        control: interiorlightspanel::WarnCautionDimmer,
        callback: IntegerCallback<'a>,
    },
    WingFoldPull {
        control: wingfoldswitch::WingFoldPull,
        callback: wingfoldswitch::WingFoldPullValueCallback<'a>,
    },
    WingFoldRotate {
        control: wingfoldswitch::WingFoldRotate,
        callback: wingfoldswitch::WingFoldRotateValueCallback<'a>,
    },
    WshieldAntiIceSw {
        control: defogpanel::WshieldAntiIceSw,
        callback: defogpanel::WshieldAntiIceSwValueCallback<'a>,
    },
}

impl RegisteredControl<'_> {
    fn accept(&mut self, packet: Packet<'_>) {
        match self {
            Self::AmpcdBrtCtl { control, .. } => control.accept(packet),
            Self::AmpcdContSw { control, .. } => control.accept(packet),
            Self::AmpcdGainSw { control, .. } => control.accept(packet),
            Self::AmpcdNightDay { control, .. } => control.accept(packet),
            Self::AmpcdPb01 { control, .. } => control.accept(packet),
            Self::AmpcdPb02 { control, .. } => control.accept(packet),
            Self::AmpcdPb03 { control, .. } => control.accept(packet),
            Self::AmpcdPb04 { control, .. } => control.accept(packet),
            Self::AmpcdPb05 { control, .. } => control.accept(packet),
            Self::AmpcdPb06 { control, .. } => control.accept(packet),
            Self::AmpcdPb07 { control, .. } => control.accept(packet),
            Self::AmpcdPb08 { control, .. } => control.accept(packet),
            Self::AmpcdPb09 { control, .. } => control.accept(packet),
            Self::AmpcdPb10 { control, .. } => control.accept(packet),
            Self::AmpcdPb11 { control, .. } => control.accept(packet),
            Self::AmpcdPb12 { control, .. } => control.accept(packet),
            Self::AmpcdPb13 { control, .. } => control.accept(packet),
            Self::AmpcdPb14 { control, .. } => control.accept(packet),
            Self::AmpcdPb15 { control, .. } => control.accept(packet),
            Self::AmpcdPb16 { control, .. } => control.accept(packet),
            Self::AmpcdPb17 { control, .. } => control.accept(packet),
            Self::AmpcdPb18 { control, .. } => control.accept(packet),
            Self::AmpcdPb19 { control, .. } => control.accept(packet),
            Self::AmpcdPb20 { control, .. } => control.accept(packet),
            Self::AmpcdSymSw { control, .. } => control.accept(packet),
            Self::AntiSkidSw { control, .. } => control.accept(packet),
            Self::AoaIndexerHigh { control, .. } => control.accept(packet),
            Self::AoaIndexerLow { control, .. } => control.accept(packet),
            Self::AoaIndexerNormal { control, .. } => control.accept(packet),
            Self::ApuControlSw { control, .. } => control.accept(packet),
            Self::ApuFireBtn { control, .. } => control.accept(packet),
            Self::ApuReadyLt { control, .. } => control.accept(packet),
            Self::ArrestingHookLt { control, .. } => control.accept(packet),
            Self::AuxRelSw { control, .. } => control.accept(packet),
            Self::AvCoolSw { control, .. } => control.accept(packet),
            Self::BatterySw { control, .. } => control.accept(packet),
            Self::BleedAirKnob { control, .. } => control.accept(packet),
            Self::BleedAirPull { control, .. } => control.accept(packet),
            Self::CabinPressSw { control, .. } => control.accept(packet),
            Self::CabinTemp { control, .. } => control.accept(packet),
            Self::CanopyJettHandlePull { control, .. } => control.accept(packet),
            Self::CanopyJettHandleUnlock { control, .. } => control.accept(packet),
            Self::CanopyPos { control, .. } => control.accept(packet),
            Self::CanopySw { control, .. } => control.accept(packet),
            Self::CbFcsChan1 { control, .. } => control.accept(packet),
            Self::CbFcsChan2 { control, .. } => control.accept(packet),
            Self::CbFcsChan3 { control, .. } => control.accept(packet),
            Self::CbFcsChan4 { control, .. } => control.accept(packet),
            Self::CbHoook { control, .. } => control.accept(packet),
            Self::CbLaunchBar { control, .. } => control.accept(packet),
            Self::CbLg { control, .. } => control.accept(packet),
            Self::CbSpdBrk { control, .. } => control.accept(packet),
            Self::ChartDimmer { control, .. } => control.accept(packet),
            Self::ClipApuAccLt { control, .. } => control.accept(packet),
            Self::ClipBattSwLt { control, .. } => control.accept(packet),
            Self::ClipCkSeatLt { control, .. } => control.accept(packet),
            Self::ClipFcesLt { control, .. } => control.accept(packet),
            Self::ClipFcsHotLt { control, .. } => control.accept(packet),
            Self::ClipFuelLoLt { control, .. } => control.accept(packet),
            Self::ClipGenTieLt { control, .. } => control.accept(packet),
            Self::ClipLGenLt { control, .. } => control.accept(packet),
            Self::ClipRGenLt { control, .. } => control.accept(packet),
            Self::ClipSpareCtn1Lt { control, .. } => control.accept(packet),
            Self::ClipSpareCtn2Lt { control, .. } => control.accept(packet),
            Self::ClipSpareCtn3Lt { control, .. } => control.accept(packet),
            Self::ClockElapsedMinutes { control, .. } => control.accept(packet),
            Self::ClockElapsedSeconds { control, .. } => control.accept(packet),
            Self::ClockHours { control, .. } => control.accept(packet),
            Self::ClockMinutes { control, .. } => control.accept(packet),
            Self::CmsdDispenseBtn { control, .. } => control.accept(packet),
            Self::CmsdDispenseSw { control, .. } => control.accept(packet),
            Self::CmsdJetSelBtn { control, .. } => control.accept(packet),
            Self::CockkpitLightModeSw { control, .. } => control.accept(packet),
            Self::ComAux { control, .. } => control.accept(packet),
            Self::ComCommGXmtSw { control, .. } => control.accept(packet),
            Self::ComCommRelaySw { control, .. } => control.accept(packet),
            Self::ComCryptoSw { control, .. } => control.accept(packet),
            Self::ComIcs { control, .. } => control.accept(packet),
            Self::ComIffMasterSw { control, .. } => control.accept(packet),
            Self::ComIffMode4Sw { control, .. } => control.accept(packet),
            Self::ComIlsChannelSw { control, .. } => control.accept(packet),
            Self::ComIlsUfcManSw { control, .. } => control.accept(packet),
            Self::ComMidsA { control, .. } => control.accept(packet),
            Self::ComMidsB { control, .. } => control.accept(packet),
            Self::ComRwr { control, .. } => control.accept(packet),
            Self::ComTacan { control, .. } => control.accept(packet),
            Self::ComVox { control, .. } => control.accept(packet),
            Self::ComWpn { control, .. } => control.accept(packet),
            Self::Comm1AntSelectSw { control, .. } => control.accept(packet),
            Self::Comm1ChannelNumeric { control, .. } => control.accept(packet),
            Self::Comm1Freq { control, .. } => control.accept(packet),
            Self::Comm2ChannelNumeric { control, .. } => control.accept(packet),
            Self::Comm2Freq { control, .. } => control.accept(packet),
            Self::ConsolesDimmer { control, .. } => control.accept(packet),
            Self::DefogHandle { control, .. } => control.accept(packet),
            Self::EcmModeSw { control, .. } => control.accept(packet),
            Self::EcsModeSw { control, .. } => control.accept(packet),
            Self::EjectionHandleSw { control, .. } => control.accept(packet),
            Self::EjectionSeatArmed { control, .. } => control.accept(packet),
            Self::EjectionSeatMnlOvrd { control, .. } => control.accept(packet),
            Self::EmerJettBtn { control, .. } => control.accept(packet),
            Self::EmergencyGearRotate { control, .. } => control.accept(packet),
            Self::EmergencyParkingBrakePull { control, .. } => control.accept(packet),
            Self::EmergencyParkingBrakeRotate { control, .. } => control.accept(packet),
            Self::EngAntiiceSw { control, .. } => control.accept(packet),
            Self::EngineCrankSw { control, .. } => control.accept(packet),
            Self::ExtCntTankSw { control, .. } => control.accept(packet),
            Self::ExtFormationLights { control, .. } => control.accept(packet),
            Self::ExtPositionLightLeft { control, .. } => control.accept(packet),
            Self::ExtPositionLightRight { control, .. } => control.accept(packet),
            Self::ExtPwrSw { control, .. } => control.accept(packet),
            Self::ExtSpeedBrake { control, .. } => control.accept(packet),
            Self::ExtStrobeLights { control, .. } => control.accept(packet),
            Self::ExtWngTankSw { control, .. } => control.accept(packet),
            Self::FcsBitSw { control, .. } => control.accept(packet),
            Self::FcsResetBtn { control, .. } => control.accept(packet),
            Self::FireApuLt { control, .. } => control.accept(packet),
            Self::FireExtBtn { control, .. } => control.accept(packet),
            Self::FireLeftLt { control, .. } => control.accept(packet),
            Self::FireRightLt { control, .. } => control.accept(packet),
            Self::FireTestSw { control, .. } => control.accept(packet),
            Self::FlapSw { control, .. } => control.accept(packet),
            Self::FlirSw { control, .. } => control.accept(packet),
            Self::FloodDimmer { control, .. } => control.accept(packet),
            Self::FlpLgFlapsLt { control, .. } => control.accept(packet),
            Self::FlpLgFullFlapsLt { control, .. } => control.accept(packet),
            Self::FlpLgHalfFlapsLt { control, .. } => control.accept(packet),
            Self::FlpLgLeftGearLt { control, .. } => control.accept(packet),
            Self::FlpLgNoseGearLt { control, .. } => control.accept(packet),
            Self::FlpLgRightGearLt { control, .. } => control.accept(packet),
            Self::FormationDimmer { control, .. } => control.accept(packet),
            Self::FuelDumpSw { control, .. } => control.accept(packet),
            Self::GainSwitch { control, .. } => control.accept(packet),
            Self::GainSwitchCover { control, .. } => control.accept(packet),
            Self::GearDownlockOverrideBtn { control, .. } => control.accept(packet),
            Self::GearLever { control, .. } => control.accept(packet),
            Self::GearSilenceBtn { control, .. } => control.accept(packet),
            Self::GenTieCover { control, .. } => control.accept(packet),
            Self::GenTieSw { control, .. } => control.accept(packet),
            Self::GndPwr1Sw { control, .. } => control.accept(packet),
            Self::GndPwr2Sw { control, .. } => control.accept(packet),
            Self::GndPwr3Sw { control, .. } => control.accept(packet),
            Self::GndPwr4Sw { control, .. } => control.accept(packet),
            Self::HideStickToggle { control, .. } => control.accept(packet),
            Self::HmdOffBrt { control, .. } => control.accept(packet),
            Self::HookBypassSw { control, .. } => control.accept(packet),
            Self::HookLever { control, .. } => control.accept(packet),
            Self::HudAltSw { control, .. } => control.accept(packet),
            Self::HudAoaIndexer { control, .. } => control.accept(packet),
            Self::HudAttSw { control, .. } => control.accept(packet),
            Self::HudBalance { control, .. } => control.accept(packet),
            Self::HudBlackLvl { control, .. } => control.accept(packet),
            Self::HudSymBrt { control, .. } => control.accept(packet),
            Self::HudSymBrtSelect { control, .. } => control.accept(packet),
            Self::HudSymRejSw { control, .. } => control.accept(packet),
            Self::HudVideoBit { control, .. } => control.accept(packet),
            Self::HudVideoControlSw { control, .. } => control.accept(packet),
            Self::HydIndBrake { control, .. } => control.accept(packet),
            Self::HydIndLeft { control, .. } => control.accept(packet),
            Self::HydIndRight { control, .. } => control.accept(packet),
            Self::HydIsolateOverrideSw { control, .. } => control.accept(packet),
            Self::Ifei { control, .. } => control.accept(packet),
            Self::IfeiBingo { control, .. } => control.accept(packet),
            Self::IfeiBingoTexture { control, .. } => control.accept(packet),
            Self::IfeiClockH { control, .. } => control.accept(packet),
            Self::IfeiClockM { control, .. } => control.accept(packet),
            Self::IfeiClockS { control, .. } => control.accept(packet),
            Self::IfeiCodes { control, .. } => control.accept(packet),
            Self::IfeiDd1 { control, .. } => control.accept(packet),
            Self::IfeiDd2 { control, .. } => control.accept(packet),
            Self::IfeiDd3 { control, .. } => control.accept(packet),
            Self::IfeiDd4 { control, .. } => control.accept(packet),
            Self::IfeiDwnBtn { control, .. } => control.accept(packet),
            Self::IfeiEtBtn { control, .. } => control.accept(packet),
            Self::IfeiFfL { control, .. } => control.accept(packet),
            Self::IfeiFfR { control, .. } => control.accept(packet),
            Self::IfeiFfTexture { control, .. } => control.accept(packet),
            Self::IfeiFuelDown { control, .. } => control.accept(packet),
            Self::IfeiFuelUp { control, .. } => control.accept(packet),
            Self::IfeiL0Texture { control, .. } => control.accept(packet),
            Self::IfeiL100Texture { control, .. } => control.accept(packet),
            Self::IfeiL50Texture { control, .. } => control.accept(packet),
            Self::IfeiLpointerTexture { control, .. } => control.accept(packet),
            Self::IfeiLscaleTexture { control, .. } => control.accept(packet),
            Self::IfeiModeBtn { control, .. } => control.accept(packet),
            Self::IfeiOilPressL { control, .. } => control.accept(packet),
            Self::IfeiOilPressR { control, .. } => control.accept(packet),
            Self::IfeiOilTexture { control, .. } => control.accept(packet),
            Self::IfeiQtyBtn { control, .. } => control.accept(packet),
            Self::IfeiR100Texture { control, .. } => control.accept(packet),
            Self::IfeiR50Texture { control, .. } => control.accept(packet),
            Self::IfeiRpmL { control, .. } => control.accept(packet),
            Self::IfeiRpmR { control, .. } => control.accept(packet),
            Self::IfeiRpmTexture { control, .. } => control.accept(packet),
            Self::IfeiRpointerTexture { control, .. } => control.accept(packet),
            Self::IfeiRscaleTexture { control, .. } => control.accept(packet),
            Self::IfeiSp { control, .. } => control.accept(packet),
            Self::IfeiTempL { control, .. } => control.accept(packet),
            Self::IfeiTempR { control, .. } => control.accept(packet),
            Self::IfeiTempTexture { control, .. } => control.accept(packet),
            Self::IfeiTimeSetMode { control, .. } => control.accept(packet),
            Self::IfeiTimerH { control, .. } => control.accept(packet),
            Self::IfeiTimerM { control, .. } => control.accept(packet),
            Self::IfeiTimerS { control, .. } => control.accept(packet),
            Self::IfeiUpBtn { control, .. } => control.accept(packet),
            Self::IfeiZTexture { control, .. } => control.accept(packet),
            Self::IfeiZoneBtn { control, .. } => control.accept(packet),
            Self::IffAntSelectSw { control, .. } => control.accept(packet),
            Self::InsSw { control, .. } => control.accept(packet),
            Self::InstPnlDimmer { control, .. } => control.accept(packet),
            Self::IntWngTankSw { control, .. } => control.accept(packet),
            Self::IrCoolSw { control, .. } => control.accept(packet),
            Self::Ky58FillSelect { control, .. } => control.accept(packet),
            Self::Ky58ModeSelect { control, .. } => control.accept(packet),
            Self::Ky58PowerSelect { control, .. } => control.accept(packet),
            Self::Ky58Volume { control, .. } => control.accept(packet),
            Self::LGenSw { control, .. } => control.accept(packet),
            Self::LandingGearHandleLt { control, .. } => control.accept(packet),
            Self::LaunchBarSw { control, .. } => control.accept(packet),
            Self::LdgTaxiSw { control, .. } => control.accept(packet),
            Self::LeftDdiBrtCtl { control, .. } => control.accept(packet),
            Self::LeftDdiBrtSelect { control, .. } => control.accept(packet),
            Self::LeftDdiContCtl { control, .. } => control.accept(packet),
            Self::LeftDdiCrsSw { control, .. } => control.accept(packet),
            Self::LeftDdiHdgSw { control, .. } => control.accept(packet),
            Self::LeftDdiPb01 { control, .. } => control.accept(packet),
            Self::LeftDdiPb02 { control, .. } => control.accept(packet),
            Self::LeftDdiPb03 { control, .. } => control.accept(packet),
            Self::LeftDdiPb04 { control, .. } => control.accept(packet),
            Self::LeftDdiPb05 { control, .. } => control.accept(packet),
            Self::LeftDdiPb06 { control, .. } => control.accept(packet),
            Self::LeftDdiPb07 { control, .. } => control.accept(packet),
            Self::LeftDdiPb08 { control, .. } => control.accept(packet),
            Self::LeftDdiPb09 { control, .. } => control.accept(packet),
            Self::LeftDdiPb10 { control, .. } => control.accept(packet),
            Self::LeftDdiPb11 { control, .. } => control.accept(packet),
            Self::LeftDdiPb12 { control, .. } => control.accept(packet),
            Self::LeftDdiPb13 { control, .. } => control.accept(packet),
            Self::LeftDdiPb14 { control, .. } => control.accept(packet),
            Self::LeftDdiPb15 { control, .. } => control.accept(packet),
            Self::LeftDdiPb16 { control, .. } => control.accept(packet),
            Self::LeftDdiPb17 { control, .. } => control.accept(packet),
            Self::LeftDdiPb18 { control, .. } => control.accept(packet),
            Self::LeftDdiPb19 { control, .. } => control.accept(packet),
            Self::LeftDdiPb20 { control, .. } => control.accept(packet),
            Self::LeftFireBtn { control, .. } => control.accept(packet),
            Self::LeftFireBtnCover { control, .. } => control.accept(packet),
            Self::LeftLouver { control, .. } => control.accept(packet),
            Self::LeftVideoBit { control, .. } => control.accept(packet),
            Self::LhAdvAspjOh { control, .. } => control.accept(packet),
            Self::LhAdvGo { control, .. } => control.accept(packet),
            Self::LhAdvLBarGreen { control, .. } => control.accept(packet),
            Self::LhAdvLBarRed { control, .. } => control.accept(packet),
            Self::LhAdvLBleed { control, .. } => control.accept(packet),
            Self::LhAdvNoGo { control, .. } => control.accept(packet),
            Self::LhAdvRBleed { control, .. } => control.accept(packet),
            Self::LhAdvRec { control, .. } => control.accept(packet),
            Self::LhAdvSpdBrk { control, .. } => control.accept(packet),
            Self::LhAdvStby { control, .. } => control.accept(packet),
            Self::LhAdvXmit { control, .. } => control.accept(packet),
            Self::LightsTestSw { control, .. } => control.accept(packet),
            Self::LowAltWarnLt { control, .. } => control.accept(packet),
            Self::LsLock { control, .. } => control.accept(packet),
            Self::LsShoot { control, .. } => control.accept(packet),
            Self::LsShootStrobe { control, .. } => control.accept(packet),
            Self::LstNflrSw { control, .. } => control.accept(packet),
            Self::LtdRArm { control, .. } => control.accept(packet),
            Self::LtdRSw { control, .. } => control.accept(packet),
            Self::MasterArmSw { control, .. } => control.accept(packet),
            Self::MasterCautionLt { control, .. } => control.accept(packet),
            Self::MasterCautionResetSw { control, .. } => control.accept(packet),
            Self::MasterModeAa { control, .. } => control.accept(packet),
            Self::MasterModeAaLt { control, .. } => control.accept(packet),
            Self::MasterModeAg { control, .. } => control.accept(packet),
            Self::MasterModeAgLt { control, .. } => control.accept(packet),
            Self::McDisch { control, .. } => control.accept(packet),
            Self::McReady { control, .. } => control.accept(packet),
            Self::McSw { control, .. } => control.accept(packet),
            Self::ModeSelectorSw { control, .. } => control.accept(packet),
            Self::ObogsSw { control, .. } => control.accept(packet),
            Self::OxyFlow { control, .. } => control.accept(packet),
            Self::PitotHeatSw { control, .. } => control.accept(packet),
            Self::PositionDimmer { control, .. } => control.accept(packet),
            Self::PressureAlt { control, .. } => control.accept(packet),
            Self::ProbeSw { control, .. } => control.accept(packet),
            Self::RGenSw { control, .. } => control.accept(packet),
            Self::RadaltAltPtr { control, .. } => control.accept(packet),
            Self::RadaltGreenLamp { control, .. } => control.accept(packet),
            Self::RadaltHeight { control, .. } => control.accept(packet),
            Self::RadaltMinHeightPtr { control, .. } => control.accept(packet),
            Self::RadaltOffFlag { control, .. } => control.accept(packet),
            Self::RadaltTestSw { control, .. } => control.accept(packet),
            Self::RadarSw { control, .. } => control.accept(packet),
            Self::RadarSwPull { control, .. } => control.accept(packet),
            Self::RhAdvAaa { control, .. } => control.accept(packet),
            Self::RhAdvAi { control, .. } => control.accept(packet),
            Self::RhAdvCw { control, .. } => control.accept(packet),
            Self::RhAdvDisp { control, .. } => control.accept(packet),
            Self::RhAdvRcdrOn { control, .. } => control.accept(packet),
            Self::RhAdvSam { control, .. } => control.accept(packet),
            Self::RhAdvSpareRh1 { control, .. } => control.accept(packet),
            Self::RhAdvSpareRh2 { control, .. } => control.accept(packet),
            Self::RhAdvSpareRh3 { control, .. } => control.accept(packet),
            Self::RhAdvSpareRh4 { control, .. } => control.accept(packet),
            Self::RhAdvSpareRh5 { control, .. } => control.accept(packet),
            Self::RightDdiBrtCtl { control, .. } => control.accept(packet),
            Self::RightDdiBrtSelect { control, .. } => control.accept(packet),
            Self::RightDdiContCtl { control, .. } => control.accept(packet),
            Self::RightDdiPb01 { control, .. } => control.accept(packet),
            Self::RightDdiPb02 { control, .. } => control.accept(packet),
            Self::RightDdiPb03 { control, .. } => control.accept(packet),
            Self::RightDdiPb04 { control, .. } => control.accept(packet),
            Self::RightDdiPb05 { control, .. } => control.accept(packet),
            Self::RightDdiPb06 { control, .. } => control.accept(packet),
            Self::RightDdiPb07 { control, .. } => control.accept(packet),
            Self::RightDdiPb08 { control, .. } => control.accept(packet),
            Self::RightDdiPb09 { control, .. } => control.accept(packet),
            Self::RightDdiPb10 { control, .. } => control.accept(packet),
            Self::RightDdiPb11 { control, .. } => control.accept(packet),
            Self::RightDdiPb12 { control, .. } => control.accept(packet),
            Self::RightDdiPb13 { control, .. } => control.accept(packet),
            Self::RightDdiPb14 { control, .. } => control.accept(packet),
            Self::RightDdiPb15 { control, .. } => control.accept(packet),
            Self::RightDdiPb16 { control, .. } => control.accept(packet),
            Self::RightDdiPb17 { control, .. } => control.accept(packet),
            Self::RightDdiPb18 { control, .. } => control.accept(packet),
            Self::RightDdiPb19 { control, .. } => control.accept(packet),
            Self::RightDdiPb20 { control, .. } => control.accept(packet),
            Self::RightFireBtn { control, .. } => control.accept(packet),
            Self::RightFireBtnCover { control, .. } => control.accept(packet),
            Self::RightLouver { control, .. } => control.accept(packet),
            Self::RightVideoBit { control, .. } => control.accept(packet),
            Self::RudTrim { control, .. } => control.accept(packet),
            Self::RudderPedalAdjust { control, .. } => control.accept(packet),
            Self::RwrBitBtn { control, .. } => control.accept(packet),
            Self::RwrBitLt { control, .. } => control.accept(packet),
            Self::RwrDisTypeSw { control, .. } => control.accept(packet),
            Self::RwrDisplayBtn { control, .. } => control.accept(packet),
            Self::RwrDisplayLt { control, .. } => control.accept(packet),
            Self::RwrDmrCtrl { control, .. } => control.accept(packet),
            Self::RwrEnableLt { control, .. } => control.accept(packet),
            Self::RwrFailLt { control, .. } => control.accept(packet),
            Self::RwrLimitLt { control, .. } => control.accept(packet),
            Self::RwrLowerLt { control, .. } => control.accept(packet),
            Self::RwrOffsetBtn { control, .. } => control.accept(packet),
            Self::RwrOffsetLt { control, .. } => control.accept(packet),
            Self::RwrPowerBtn { control, .. } => control.accept(packet),
            Self::RwrRwrIntesity { control, .. } => control.accept(packet),
            Self::RwrSpecialBtn { control, .. } => control.accept(packet),
            Self::RwrSpecialEnLt { control, .. } => control.accept(packet),
            Self::RwrSpecialLt { control, .. } => control.accept(packet),
            Self::SaiAttWarningFlag { control, .. } => control.accept(packet),
            Self::SaiBank { control, .. } => control.accept(packet),
            Self::SaiCage { control, .. } => control.accept(packet),
            Self::SaiManPitchAdj { control, .. } => control.accept(packet),
            Self::SaiPitch { control, .. } => control.accept(packet),
            Self::SaiRateOfTurn { control, .. } => control.accept(packet),
            Self::SaiSet { control, .. } => control.accept(packet),
            Self::SaiSlipBall { control, .. } => control.accept(packet),
            Self::SaiTestBtn { control, .. } => control.accept(packet),
            Self::SeatHeightSw { control, .. } => control.accept(packet),
            Self::SelJettBtn { control, .. } => control.accept(packet),
            Self::SelJettKnob { control, .. } => control.accept(packet),
            Self::SelectHmdLddiRddi { control, .. } => control.accept(packet),
            Self::SelectHudLddiRddi { control, .. } => control.accept(packet),
            Self::ShldrHarnessSw { control, .. } => control.accept(packet),
            Self::SjCtr { control, .. } => control.accept(packet),
            Self::SjCtrLt { control, .. } => control.accept(packet),
            Self::SjLi { control, .. } => control.accept(packet),
            Self::SjLiLt { control, .. } => control.accept(packet),
            Self::SjLo { control, .. } => control.accept(packet),
            Self::SjLoLt { control, .. } => control.accept(packet),
            Self::SjRi { control, .. } => control.accept(packet),
            Self::SjRiLt { control, .. } => control.accept(packet),
            Self::SjRo { control, .. } => control.accept(packet),
            Self::SjRoLt { control, .. } => control.accept(packet),
            Self::SpinLt { control, .. } => control.accept(packet),
            Self::SpinRecoveryCover { control, .. } => control.accept(packet),
            Self::SpinRecoverySw { control, .. } => control.accept(packet),
            Self::StbyAlt10000FtCnt { control, .. } => control.accept(packet),
            Self::StbyAlt1000FtCnt { control, .. } => control.accept(packet),
            Self::StbyAlt100FtPtr { control, .. } => control.accept(packet),
            Self::StbyAsiAirspeed { control, .. } => control.accept(packet),
            Self::StbyPressAlt { control, .. } => control.accept(packet),
            Self::StbyPressSet0 { control, .. } => control.accept(packet),
            Self::StbyPressSet1 { control, .. } => control.accept(packet),
            Self::StbyPressSet2 { control, .. } => control.accept(packet),
            Self::StrobeSw { control, .. } => control.accept(packet),
            Self::SuitTemp { control, .. } => control.accept(packet),
            Self::ThrottleFriction { control, .. } => control.accept(packet),
            Self::ToTrimBtn { control, .. } => control.accept(packet),
            Self::Ufc0 { control, .. } => control.accept(packet),
            Self::Ufc1 { control, .. } => control.accept(packet),
            Self::Ufc2 { control, .. } => control.accept(packet),
            Self::Ufc3 { control, .. } => control.accept(packet),
            Self::Ufc4 { control, .. } => control.accept(packet),
            Self::Ufc5 { control, .. } => control.accept(packet),
            Self::Ufc6 { control, .. } => control.accept(packet),
            Self::Ufc7 { control, .. } => control.accept(packet),
            Self::Ufc8 { control, .. } => control.accept(packet),
            Self::Ufc9 { control, .. } => control.accept(packet),
            Self::UfcAdf { control, .. } => control.accept(packet),
            Self::UfcAp { control, .. } => control.accept(packet),
            Self::UfcBcn { control, .. } => control.accept(packet),
            Self::UfcBrt { control, .. } => control.accept(packet),
            Self::UfcClr { control, .. } => control.accept(packet),
            Self::UfcComm1Display { control, .. } => control.accept(packet),
            Self::UfcComm1Pull { control, .. } => control.accept(packet),
            Self::UfcComm1Vol { control, .. } => control.accept(packet),
            Self::UfcComm2Display { control, .. } => control.accept(packet),
            Self::UfcComm2Pull { control, .. } => control.accept(packet),
            Self::UfcComm2Vol { control, .. } => control.accept(packet),
            Self::UfcDl { control, .. } => control.accept(packet),
            Self::UfcEmcon { control, .. } => control.accept(packet),
            Self::UfcEnt { control, .. } => control.accept(packet),
            Self::UfcIff { control, .. } => control.accept(packet),
            Self::UfcIls { control, .. } => control.accept(packet),
            Self::UfcIp { control, .. } => control.accept(packet),
            Self::UfcOnoff { control, .. } => control.accept(packet),
            Self::UfcOptionCueing1 { control, .. } => control.accept(packet),
            Self::UfcOptionCueing2 { control, .. } => control.accept(packet),
            Self::UfcOptionCueing3 { control, .. } => control.accept(packet),
            Self::UfcOptionCueing4 { control, .. } => control.accept(packet),
            Self::UfcOptionCueing5 { control, .. } => control.accept(packet),
            Self::UfcOptionDisplay1 { control, .. } => control.accept(packet),
            Self::UfcOptionDisplay2 { control, .. } => control.accept(packet),
            Self::UfcOptionDisplay3 { control, .. } => control.accept(packet),
            Self::UfcOptionDisplay4 { control, .. } => control.accept(packet),
            Self::UfcOptionDisplay5 { control, .. } => control.accept(packet),
            Self::UfcOs1 { control, .. } => control.accept(packet),
            Self::UfcOs2 { control, .. } => control.accept(packet),
            Self::UfcOs3 { control, .. } => control.accept(packet),
            Self::UfcOs4 { control, .. } => control.accept(packet),
            Self::UfcOs5 { control, .. } => control.accept(packet),
            Self::UfcScratchpadNumberDisplay { control, .. } => control.accept(packet),
            Self::UfcScratchpadString1Display { control, .. } => control.accept(packet),
            Self::UfcScratchpadString2Display { control, .. } => control.accept(packet),
            Self::UfcTcn { control, .. } => control.accept(packet),
            Self::VoltE { control, .. } => control.accept(packet),
            Self::VoltU { control, .. } => control.accept(packet),
            Self::Vsi { control, .. } => control.accept(packet),
            Self::WarnCautionDimmer { control, .. } => control.accept(packet),
            Self::WingFoldPull { control, .. } => control.accept(packet),
            Self::WingFoldRotate { control, .. } => control.accept(packet),
            Self::WshieldAntiIceSw { control, .. } => control.accept(packet),
        }
    }

    fn commit(&mut self) {
        match self {
            Self::AmpcdBrtCtl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AmpcdContSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdContSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdGainSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdGainSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdNightDay { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdNightDayValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdPb01 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdPb01Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdPb02 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdPb02Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdPb03 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdPb03Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdPb04 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdPb04Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdPb05 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdPb05Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdPb06 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdPb06Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdPb07 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdPb07Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdPb08 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdPb08Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdPb09 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdPb09Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdPb10 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdPb10Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdPb11 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdPb11Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdPb12 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdPb12Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdPb13 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdPb13Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdPb14 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdPb14Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdPb15 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdPb15Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdPb16 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdPb16Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdPb17 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdPb17Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdPb18 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdPb18Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdPb19 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdPb19Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdPb20 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdPb20Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmpcdSymSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ampcd::AmpcdSymSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AntiSkidSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <selectjettisonbutton::AntiSkidSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::AoaIndexerHigh { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::AoaIndexerLow { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::AoaIndexerNormal { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ApuControlSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ApuFireBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <apufirewarningextinguisherlight::ApuFireBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::ApuReadyLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ArrestingHookLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::AuxRelSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <dispenseremcpanel::AuxRelSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AvCoolSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flightcomputercoolswitch::AvCoolSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::BatterySw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricalpowerpanel::BatterySwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::BleedAirKnob { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <environmentcontrolsystempanel::BleedAirKnobValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::BleedAirPull { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <environmentcontrolsystempanel::BleedAirPullValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CabinPressSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <environmentcontrolsystempanel::CabinPressSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CabinTemp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CanopyJettHandlePull { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <canopyinternaljettisonhandle::CanopyJettHandlePullValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CanopyJettHandleUnlock { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <canopyinternaljettisonhandle::CanopyJettHandleUnlockValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CanopyPos { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CanopySw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <internalcanopyswitch::CanopySwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CbFcsChan1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftessentialcircuitbreakers::CbFcsChan1Value as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbFcsChan2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftessentialcircuitbreakers::CbFcsChan2Value as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbFcsChan3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rightessentialcircuitbreakers::CbFcsChan3Value as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbFcsChan4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rightessentialcircuitbreakers::CbFcsChan4Value as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbHoook { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rightessentialcircuitbreakers::CbHoookValue as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbLaunchBar { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftessentialcircuitbreakers::CbLaunchBarValue as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbLg { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightessentialcircuitbreakers::CbLgValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CbSpdBrk { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftessentialcircuitbreakers::CbSpdBrkValue as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ChartDimmer { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClipApuAccLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClipBattSwLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClipCkSeatLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClipFcesLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClipFcsHotLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClipFuelLoLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClipGenTieLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClipLGenLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClipRGenLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClipSpareCtn1Lt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClipSpareCtn2Lt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClipSpareCtn3Lt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClockElapsedMinutes { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockElapsedSeconds { control, callback } => {
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
            Self::CmsdDispenseBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ecmdispenserbutton::CmsdDispenseBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CmsdDispenseSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <dispenseremcpanel::CmsdDispenseSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CmsdJetSelBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <dispenseremcpanel::CmsdJetSelBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CockkpitLightModeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <interiorlightspanel::CockkpitLightModeSwValue as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ComAux { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ComCommGXmtSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <communicationpanel::ComCommGXmtSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::ComCommRelaySw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <communicationpanel::ComCommRelaySwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::ComCryptoSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <communicationpanel::ComCryptoSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::ComIcs { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ComIffMasterSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <communicationpanel::ComIffMasterSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::ComIffMode4Sw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <communicationpanel::ComIffMode4SwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::ComIlsChannelSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ComIlsUfcManSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <communicationpanel::ComIlsUfcManSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::ComMidsA { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ComMidsB { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ComRwr { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ComTacan { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ComVox { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ComWpn { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Comm1AntSelectSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <antennaselectpanel::Comm1AntSelectSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::Comm1ChannelNumeric { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Comm1Freq { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Comm2ChannelNumeric { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Comm2Freq { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ConsolesDimmer { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DefogHandle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EcmModeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <dispenseremcpanel::EcmModeSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EcsModeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <environmentcontrolsystempanel::EcsModeSwValue as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EjectionHandleSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EjectionSeatArmed { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ejectionseat::EjectionSeatArmedValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::EjectionSeatMnlOvrd { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ejectionseat::EjectionSeatMnlOvrdValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::EmerJettBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <emergencyjettisonbutton::EmerJettBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::EmergencyGearRotate { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <landinggearhandleandwarningtonesilence::EmergencyGearRotateValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::EmergencyParkingBrakePull { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <emergencyandparkingbrakehandle::EmergencyParkingBrakePullValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::EmergencyParkingBrakeRotate { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EngAntiiceSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <environmentcontrolsystempanel::EngAntiiceSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::EngineCrankSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <auxiliarypowerunitpanel::EngineCrankSwValue as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ExtCntTankSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fuelpanel::ExtCntTankSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
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
            Self::ExtPwrSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <groundpowerpanel::ExtPwrSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ExtSpeedBrake { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtStrobeLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtWngTankSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fuelpanel::ExtWngTankSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FcsBitSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rightessentialcircuitbreakers::FcsBitSwValue as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FcsResetBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <flightcontrolsystempanel::FcsResetBtnValue as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FireApuLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FireExtBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <firesystems::FireExtBtnValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FireLeftLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FireRightLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FireTestSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <firetestpanel::FireTestSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FlapSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <selectjettisonbutton::FlapSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::FlirSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <sensorpanel::FlirSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::FloodDimmer { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FlpLgFlapsLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FlpLgFullFlapsLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FlpLgHalfFlapsLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FlpLgLeftGearLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FlpLgNoseGearLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FlpLgRightGearLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FormationDimmer { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelDumpSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GainSwitch { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flightcontrolsystempanel::GainSwitchValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::GainSwitchCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <flightcontrolsystempanel::GainSwitchCoverValue as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::GearDownlockOverrideBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <landinggearhandleandwarningtonesilence::GearDownlockOverrideBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::GearLever { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <landinggearhandleandwarningtonesilence::GearLeverValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::GearSilenceBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <landinggearhandleandwarningtonesilence::GearSilenceBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::GenTieCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <generatortiecontrolswitch::GenTieCoverValue as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::GenTieSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <generatortiecontrolswitch::GenTieSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::GndPwr1Sw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <groundpowerpanel::GndPwr1SwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::GndPwr2Sw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <groundpowerpanel::GndPwr2SwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::GndPwr3Sw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <groundpowerpanel::GndPwr3SwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::GndPwr4Sw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <groundpowerpanel::GndPwr4SwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::HideStickToggle { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ejectionseat::HideStickToggleValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::HmdOffBrt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HookBypassSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HookLever { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <arrestinghookhandleandlight::HookLeverValue as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HudAltSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <hudcontrolpanel::HudAltSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HudAoaIndexer { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HudAttSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <hudcontrolpanel::HudAttSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HudBalance { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HudBlackLvl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HudSymBrt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HudSymBrtSelect { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <hudcontrolpanel::HudSymBrtSelectValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::HudSymRejSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <hudcontrolpanel::HudSymRejSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::HudVideoBit { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <hudvideobitpanel::HudVideoBitValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::HudVideoControlSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <hudcontrolpanel::HudVideoControlSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::HydIndBrake { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HydIndLeft { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HydIndRight { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HydIsolateOverrideSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <missioncomputerandhydraulicisolatepanel::HydIsolateOverrideSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::Ifei { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiBingo { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiBingoTexture { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiClockH { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiClockM { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiClockS { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiCodes { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiDd1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiDd2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiDd3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiDd4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiDwnBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <integratedfuelengineindicatorifei::IfeiDwnBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::IfeiEtBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <integratedfuelengineindicatorifei::IfeiEtBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::IfeiFfL { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiFfR { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiFfTexture { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiFuelDown { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiFuelUp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiL0Texture { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiL100Texture { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiL50Texture { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiLpointerTexture { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiLscaleTexture { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiModeBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <integratedfuelengineindicatorifei::IfeiModeBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::IfeiOilPressL { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiOilPressR { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiOilTexture { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiQtyBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <integratedfuelengineindicatorifei::IfeiQtyBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::IfeiR100Texture { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiR50Texture { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiRpmL { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiRpmR { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiRpmTexture { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiRpointerTexture { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiRscaleTexture { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiSp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiTempL { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiTempR { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiTempTexture { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiTimeSetMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiTimerH { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiTimerM { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiTimerS { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiUpBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <integratedfuelengineindicatorifei::IfeiUpBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::IfeiZTexture { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IfeiZoneBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <integratedfuelengineindicatorifei::IfeiZoneBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::IffAntSelectSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <antennaselectpanel::IffAntSelectSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::InsSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <sensorpanel::InsSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::InstPnlDimmer { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IntWngTankSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <exteriorlightspanel::IntWngTankSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::IrCoolSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <mapgainspinrecoverypanel::IrCoolSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::Ky58FillSelect { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ky58control::Ky58FillSelectValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Ky58ModeSelect { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ky58control::Ky58ModeSelectValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Ky58PowerSelect { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ky58control::Ky58PowerSelectValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Ky58Volume { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LGenSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <electricalpowerpanel::LGenSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LandingGearHandleLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LaunchBarSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LdgTaxiSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <selectjettisonbutton::LdgTaxiSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::LeftDdiBrtCtl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LeftDdiBrtSelect { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <leftddi::LeftDdiBrtSelectValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiContCtl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LeftDdiCrsSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <headingandcoursesetswitches::LeftDdiCrsSwValue as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiHdgSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <headingandcoursesetswitches::LeftDdiHdgSwValue as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiPb01 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftddi::LeftDdiPb01Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiPb02 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftddi::LeftDdiPb02Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiPb03 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftddi::LeftDdiPb03Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiPb04 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftddi::LeftDdiPb04Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiPb05 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftddi::LeftDdiPb05Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiPb06 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftddi::LeftDdiPb06Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiPb07 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftddi::LeftDdiPb07Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiPb08 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftddi::LeftDdiPb08Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiPb09 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftddi::LeftDdiPb09Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiPb10 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftddi::LeftDdiPb10Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiPb11 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftddi::LeftDdiPb11Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiPb12 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftddi::LeftDdiPb12Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiPb13 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftddi::LeftDdiPb13Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiPb14 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftddi::LeftDdiPb14Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiPb15 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftddi::LeftDdiPb15Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiPb16 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftddi::LeftDdiPb16Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiPb17 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftddi::LeftDdiPb17Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiPb18 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftddi::LeftDdiPb18Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiPb19 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftddi::LeftDdiPb19Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftDdiPb20 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <leftddi::LeftDdiPb20Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftFireBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <leftenginefirewarningextinguisherlight::LeftFireBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::LeftFireBtnCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <leftenginefirewarningextinguisherlight::LeftFireBtnCoverValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::LeftLouver { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LeftVideoBit { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <todo::LeftVideoBitValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LhAdvAspjOh { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LhAdvGo { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LhAdvLBarGreen { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LhAdvLBarRed { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LhAdvLBleed { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LhAdvNoGo { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LhAdvRBleed { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LhAdvRec { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LhAdvSpdBrk { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LhAdvStby { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LhAdvXmit { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightsTestSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <interiorlightspanel::LightsTestSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::LowAltWarnLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LsLock { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LsShoot { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LsShootStrobe { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LstNflrSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <sensorpanel::LstNflrSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LtdRArm { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <sensorpanel::LtdRArmValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LtdRSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <sensorpanel::LtdRSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MasterArmSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <masterarmpanel::MasterArmSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MasterCautionLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::MasterCautionResetSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mastercautionlight::MasterCautionResetSwValue as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MasterModeAa { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <masterarmpanel::MasterModeAaValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MasterModeAaLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::MasterModeAg { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <masterarmpanel::MasterModeAgValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MasterModeAgLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::McDisch { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::McReady { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::McSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ModeSelectorSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <hudvideorecordpanel::ModeSelectorSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::ObogsSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <loxindicator::ObogsSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::OxyFlow { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PitotHeatSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PositionDimmer { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PressureAlt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ProbeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <fuelpanel::ProbeSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RGenSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <electricalpowerpanel::RGenSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RadaltAltPtr { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadaltGreenLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RadaltHeight { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadaltMinHeightPtr { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadaltOffFlag { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadaltTestSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <radaraltimeter::RadaltTestSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RadarSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <sensorpanel::RadarSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RadarSwPull { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <sensorpanel::RadarSwPullValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RhAdvAaa { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RhAdvAi { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RhAdvCw { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RhAdvDisp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RhAdvRcdrOn { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RhAdvSam { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RhAdvSpareRh1 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RhAdvSpareRh2 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RhAdvSpareRh3 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RhAdvSpareRh4 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RhAdvSpareRh5 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RightDdiBrtCtl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RightDdiBrtSelect { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rightddi::RightDdiBrtSelectValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RightDdiContCtl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RightDdiPb01 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightddi::RightDdiPb01Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightDdiPb02 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightddi::RightDdiPb02Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightDdiPb03 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightddi::RightDdiPb03Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightDdiPb04 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightddi::RightDdiPb04Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightDdiPb05 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightddi::RightDdiPb05Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightDdiPb06 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightddi::RightDdiPb06Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightDdiPb07 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightddi::RightDdiPb07Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightDdiPb08 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightddi::RightDdiPb08Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightDdiPb09 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightddi::RightDdiPb09Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightDdiPb10 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightddi::RightDdiPb10Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightDdiPb11 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightddi::RightDdiPb11Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightDdiPb12 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightddi::RightDdiPb12Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightDdiPb13 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightddi::RightDdiPb13Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightDdiPb14 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightddi::RightDdiPb14Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightDdiPb15 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightddi::RightDdiPb15Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightDdiPb16 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightddi::RightDdiPb16Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightDdiPb17 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightddi::RightDdiPb17Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightDdiPb18 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightddi::RightDdiPb18Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightDdiPb19 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightddi::RightDdiPb19Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightDdiPb20 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightddi::RightDdiPb20Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightFireBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightenginefirewarningextinguisherlight::RightFireBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::RightFireBtnCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rightenginefirewarningextinguisherlight::RightFireBtnCoverValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::RightLouver { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RightVideoBit { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <todo::RightVideoBitValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RudTrim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RudderPedalAdjust { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rudderpedaladjustlevel::RudderPedalAdjustValue as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrBitBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rwrcontrolindicator::RwrBitBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::RwrBitLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RwrDisTypeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rwrcontrolindicator::RwrDisTypeSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::RwrDisplayBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rwrcontrolindicator::RwrDisplayBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::RwrDisplayLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RwrDmrCtrl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RwrEnableLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RwrFailLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RwrLimitLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RwrLowerLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RwrOffsetBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rwrcontrolindicator::RwrOffsetBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::RwrOffsetLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RwrPowerBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rwrcontrolindicator::RwrPowerBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::RwrRwrIntesity { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RwrSpecialBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rwrcontrolindicator::RwrSpecialBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::RwrSpecialEnLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RwrSpecialLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::SaiAttWarningFlag { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SaiBank { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SaiCage { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <standbyattitudereferenceindicator::SaiCageValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::SaiManPitchAdj { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SaiPitch { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SaiRateOfTurn { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SaiSet { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SaiSlipBall { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SaiTestBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <standbyattitudereferenceindicator::SaiTestBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::SeatHeightSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ejectionseat::SeatHeightSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::SelJettBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <selectjettisonbutton::SelJettBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::SelJettKnob { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <selectjettisonbutton::SelJettKnobValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::SelectHmdLddiRddi { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <hudvideorecordpanel::SelectHmdLddiRddiValue as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::SelectHudLddiRddi { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <hudvideorecordpanel::SelectHudLddiRddiValue as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ShldrHarnessSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ejectionseat::ShldrHarnessSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::SjCtr { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <stationjettisonselect::SjCtrValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::SjCtrLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::SjLi { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <stationjettisonselect::SjLiValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::SjLiLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::SjLo { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <stationjettisonselect::SjLoValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::SjLoLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::SjRi { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <stationjettisonselect::SjRiValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::SjRiLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::SjRo { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <stationjettisonselect::SjRoValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::SjRoLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::SpinLt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::SpinRecoveryCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <mapgainspinrecoverypanel::SpinRecoveryCoverValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::SpinRecoverySw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mapgainspinrecoverypanel::SpinRecoverySwValue as core::convert::TryFrom<
                            u16,
                        >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::StbyAlt10000FtCnt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StbyAlt1000FtCnt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StbyAlt100FtPtr { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StbyAsiAirspeed { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StbyPressAlt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StbyPressSet0 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StbyPressSet1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StbyPressSet2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StrobeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <exteriorlightspanel::StrobeSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::SuitTemp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ThrottleFriction { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ToTrimBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flightcontrolsystempanel::ToTrimBtnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::Ufc0 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <upfrontcontrollerufc::Ufc0Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Ufc1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <upfrontcontrollerufc::Ufc1Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Ufc2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <upfrontcontrollerufc::Ufc2Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Ufc3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <upfrontcontrollerufc::Ufc3Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Ufc4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <upfrontcontrollerufc::Ufc4Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Ufc5 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <upfrontcontrollerufc::Ufc5Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Ufc6 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <upfrontcontrollerufc::Ufc6Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Ufc7 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <upfrontcontrollerufc::Ufc7Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Ufc8 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <upfrontcontrollerufc::Ufc8Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Ufc9 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <upfrontcontrollerufc::Ufc9Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UfcAdf { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <upfrontcontrollerufc::UfcAdfValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UfcAp { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <upfrontcontrollerufc::UfcApValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UfcBcn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <upfrontcontrollerufc::UfcBcnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UfcBrt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcClr { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <upfrontcontrollerufc::UfcClrValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UfcComm1Display { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcComm1Pull { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <upfrontcontrollerufc::UfcComm1PullValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::UfcComm1Vol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcComm2Display { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcComm2Pull { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <upfrontcontrollerufc::UfcComm2PullValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::UfcComm2Vol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcDl { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <upfrontcontrollerufc::UfcDlValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UfcEmcon { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <upfrontcontrollerufc::UfcEmconValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::UfcEnt { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <upfrontcontrollerufc::UfcEntValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UfcIff { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <upfrontcontrollerufc::UfcIffValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UfcIls { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <upfrontcontrollerufc::UfcIlsValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UfcIp { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <upfrontcontrollerufc::UfcIpValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UfcOnoff { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <upfrontcontrollerufc::UfcOnoffValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::UfcOptionCueing1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcOptionCueing2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcOptionCueing3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcOptionCueing4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcOptionCueing5 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcOptionDisplay1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcOptionDisplay2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcOptionDisplay3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcOptionDisplay4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcOptionDisplay5 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcOs1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <upfrontcontrollerufc::UfcOs1Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UfcOs2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <upfrontcontrollerufc::UfcOs2Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UfcOs3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <upfrontcontrollerufc::UfcOs3Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UfcOs4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <upfrontcontrollerufc::UfcOs4Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UfcOs5 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <upfrontcontrollerufc::UfcOs5Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UfcScratchpadNumberDisplay { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcScratchpadString1Display { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcScratchpadString2Display { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UfcTcn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <upfrontcontrollerufc::UfcTcnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::VoltE { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VoltU { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Vsi { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::WarnCautionDimmer { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::WingFoldPull { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <wingfoldswitch::WingFoldPullValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::WingFoldRotate { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <wingfoldswitch::WingFoldRotateValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::WshieldAntiIceSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <defogpanel::WshieldAntiIceSwValue as core::convert::TryFrom<u16>>::try_from(
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

pub struct Fa18cHornetBuilder<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    _packet_capacity: PhantomData<ProtocolParser<MAX_PACKET_DATA>>,
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    Fa18cHornetBuilder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
{
    pub fn new() -> Self {
        Self {
            controls: Vec::new(),
            _packet_capacity: PhantomData,
        }
    }

    pub fn register_ampcd_brt_ctl<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdBrtCtl {
                control: ampcd::AmpcdBrtCtl::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_cont_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdContSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdContSw {
                control: ampcd::AmpcdContSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_gain_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdGainSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdGainSw {
                control: ampcd::AmpcdGainSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_night_day<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdNightDayValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdNightDay {
                control: ampcd::AmpcdNightDay::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_pb01<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdPb01Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdPb01 {
                control: ampcd::AmpcdPb01::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_pb02<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdPb02Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdPb02 {
                control: ampcd::AmpcdPb02::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_pb03<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdPb03Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdPb03 {
                control: ampcd::AmpcdPb03::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_pb04<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdPb04Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdPb04 {
                control: ampcd::AmpcdPb04::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_pb05<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdPb05Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdPb05 {
                control: ampcd::AmpcdPb05::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_pb06<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdPb06Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdPb06 {
                control: ampcd::AmpcdPb06::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_pb07<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdPb07Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdPb07 {
                control: ampcd::AmpcdPb07::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_pb08<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdPb08Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdPb08 {
                control: ampcd::AmpcdPb08::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_pb09<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdPb09Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdPb09 {
                control: ampcd::AmpcdPb09::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_pb10<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdPb10Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdPb10 {
                control: ampcd::AmpcdPb10::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_pb11<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdPb11Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdPb11 {
                control: ampcd::AmpcdPb11::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_pb12<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdPb12Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdPb12 {
                control: ampcd::AmpcdPb12::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_pb13<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdPb13Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdPb13 {
                control: ampcd::AmpcdPb13::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_pb14<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdPb14Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdPb14 {
                control: ampcd::AmpcdPb14::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_pb15<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdPb15Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdPb15 {
                control: ampcd::AmpcdPb15::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_pb16<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdPb16Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdPb16 {
                control: ampcd::AmpcdPb16::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_pb17<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdPb17Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdPb17 {
                control: ampcd::AmpcdPb17::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_pb18<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdPb18Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdPb18 {
                control: ampcd::AmpcdPb18::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_pb19<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdPb19Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdPb19 {
                control: ampcd::AmpcdPb19::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_pb20<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdPb20Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdPb20 {
                control: ampcd::AmpcdPb20::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ampcd_sym_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ampcd::AmpcdSymSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmpcdSymSw {
                control: ampcd::AmpcdSymSw::new(),
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
        F: FnMut(selectjettisonbutton::AntiSkidSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AntiSkidSw {
                control: selectjettisonbutton::AntiSkidSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aoa_indexer_high<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AoaIndexerHigh {
                control: angleofattackindexerlights::AoaIndexerHigh::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aoa_indexer_low<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AoaIndexerLow {
                control: angleofattackindexerlights::AoaIndexerLow::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aoa_indexer_normal<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AoaIndexerNormal {
                control: angleofattackindexerlights::AoaIndexerNormal::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_apu_control_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApuControlSw {
                control: auxiliarypowerunitpanel::ApuControlSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_apu_fire_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(apufirewarningextinguisherlight::ApuFireBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApuFireBtn {
                control: apufirewarningextinguisherlight::ApuFireBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_apu_ready_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApuReadyLt {
                control: auxiliarypowerunitpanel::ApuReadyLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_arresting_hook_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ArrestingHookLt {
                control: arrestinghookhandleandlight::ArrestingHookLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aux_rel_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(dispenseremcpanel::AuxRelSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AuxRelSw {
                control: dispenseremcpanel::AuxRelSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_av_cool_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightcomputercoolswitch::AvCoolSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AvCoolSw {
                control: flightcomputercoolswitch::AvCoolSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_battery_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricalpowerpanel::BatterySwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BatterySw {
                control: electricalpowerpanel::BatterySw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bleed_air_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(environmentcontrolsystempanel::BleedAirKnobValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BleedAirKnob {
                control: environmentcontrolsystempanel::BleedAirKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bleed_air_pull<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(environmentcontrolsystempanel::BleedAirPullValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BleedAirPull {
                control: environmentcontrolsystempanel::BleedAirPull::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cabin_press_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(environmentcontrolsystempanel::CabinPressSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CabinPressSw {
                control: environmentcontrolsystempanel::CabinPressSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cabin_temp<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CabinTemp {
                control: environmentcontrolsystempanel::CabinTemp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_jett_handle_pull<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(canopyinternaljettisonhandle::CanopyJettHandlePullValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyJettHandlePull {
                control: canopyinternaljettisonhandle::CanopyJettHandlePull::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_jett_handle_unlock<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(canopyinternaljettisonhandle::CanopyJettHandleUnlockValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyJettHandleUnlock {
                control: canopyinternaljettisonhandle::CanopyJettHandleUnlock::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_pos<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyPos {
                control: internalcanopyswitch::CanopyPos::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(internalcanopyswitch::CanopySwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopySw {
                control: internalcanopyswitch::CanopySw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fcs_chan1<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftessentialcircuitbreakers::CbFcsChan1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFcsChan1 {
                control: leftessentialcircuitbreakers::CbFcsChan1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fcs_chan2<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftessentialcircuitbreakers::CbFcsChan2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFcsChan2 {
                control: leftessentialcircuitbreakers::CbFcsChan2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fcs_chan3<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightessentialcircuitbreakers::CbFcsChan3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFcsChan3 {
                control: rightessentialcircuitbreakers::CbFcsChan3::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fcs_chan4<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightessentialcircuitbreakers::CbFcsChan4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFcsChan4 {
                control: rightessentialcircuitbreakers::CbFcsChan4::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_hoook<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightessentialcircuitbreakers::CbHoookValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbHoook {
                control: rightessentialcircuitbreakers::CbHoook::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_launch_bar<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftessentialcircuitbreakers::CbLaunchBarValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbLaunchBar {
                control: leftessentialcircuitbreakers::CbLaunchBar::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_lg<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightessentialcircuitbreakers::CbLgValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbLg {
                control: rightessentialcircuitbreakers::CbLg::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_spd_brk<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftessentialcircuitbreakers::CbSpdBrkValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbSpdBrk {
                control: leftessentialcircuitbreakers::CbSpdBrk::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_chart_dimmer<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ChartDimmer {
                control: interiorlightspanel::ChartDimmer::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clip_apu_acc_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClipApuAccLt {
                control: cautionlightpanel::ClipApuAccLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clip_batt_sw_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClipBattSwLt {
                control: cautionlightpanel::ClipBattSwLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clip_ck_seat_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClipCkSeatLt {
                control: cautionlightpanel::ClipCkSeatLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clip_fces_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClipFcesLt {
                control: cautionlightpanel::ClipFcesLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clip_fcs_hot_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClipFcsHotLt {
                control: cautionlightpanel::ClipFcsHotLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clip_fuel_lo_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClipFuelLoLt {
                control: cautionlightpanel::ClipFuelLoLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clip_gen_tie_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClipGenTieLt {
                control: cautionlightpanel::ClipGenTieLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clip_l_gen_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClipLGenLt {
                control: cautionlightpanel::ClipLGenLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clip_r_gen_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClipRGenLt {
                control: cautionlightpanel::ClipRGenLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clip_spare_ctn1_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClipSpareCtn1Lt {
                control: cautionlightpanel::ClipSpareCtn1Lt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clip_spare_ctn2_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClipSpareCtn2Lt {
                control: cautionlightpanel::ClipSpareCtn2Lt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clip_spare_ctn3_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClipSpareCtn3Lt {
                control: cautionlightpanel::ClipSpareCtn3Lt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_elapsed_minutes<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockElapsedMinutes {
                control: clock::ClockElapsedMinutes::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_elapsed_seconds<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockElapsedSeconds {
                control: clock::ClockElapsedSeconds::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_hours<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockHours {
                control: clock::ClockHours::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_minutes<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockMinutes {
                control: clock::ClockMinutes::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmsd_dispense_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ecmdispenserbutton::CmsdDispenseBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmsdDispenseBtn {
                control: ecmdispenserbutton::CmsdDispenseBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmsd_dispense_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(dispenseremcpanel::CmsdDispenseSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmsdDispenseSw {
                control: dispenseremcpanel::CmsdDispenseSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmsd_jet_sel_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(dispenseremcpanel::CmsdJetSelBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmsdJetSelBtn {
                control: dispenseremcpanel::CmsdJetSelBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cockkpit_light_mode_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(interiorlightspanel::CockkpitLightModeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CockkpitLightModeSw {
                control: interiorlightspanel::CockkpitLightModeSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_com_aux<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ComAux {
                control: communicationpanel::ComAux::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_com_comm_g_xmt_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(communicationpanel::ComCommGXmtSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ComCommGXmtSw {
                control: communicationpanel::ComCommGXmtSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_com_comm_relay_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(communicationpanel::ComCommRelaySwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ComCommRelaySw {
                control: communicationpanel::ComCommRelaySw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_com_crypto_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(communicationpanel::ComCryptoSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ComCryptoSw {
                control: communicationpanel::ComCryptoSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_com_ics<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ComIcs {
                control: communicationpanel::ComIcs::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_com_iff_master_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(communicationpanel::ComIffMasterSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ComIffMasterSw {
                control: communicationpanel::ComIffMasterSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_com_iff_mode4_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(communicationpanel::ComIffMode4SwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ComIffMode4Sw {
                control: communicationpanel::ComIffMode4Sw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_com_ils_channel_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ComIlsChannelSw {
                control: communicationpanel::ComIlsChannelSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_com_ils_ufc_man_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(communicationpanel::ComIlsUfcManSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ComIlsUfcManSw {
                control: communicationpanel::ComIlsUfcManSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_com_mids_a<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ComMidsA {
                control: communicationpanel::ComMidsA::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_com_mids_b<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ComMidsB {
                control: communicationpanel::ComMidsB::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_com_rwr<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ComRwr {
                control: communicationpanel::ComRwr::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_com_tacan<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ComTacan {
                control: communicationpanel::ComTacan::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_com_vox<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ComVox {
                control: communicationpanel::ComVox::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_com_wpn<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ComWpn {
                control: communicationpanel::ComWpn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_comm1_ant_select_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(antennaselectpanel::Comm1AntSelectSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Comm1AntSelectSw {
                control: antennaselectpanel::Comm1AntSelectSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_comm1_channel_numeric<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Comm1ChannelNumeric {
                control: commsfrequency::Comm1ChannelNumeric::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_comm1_freq<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Comm1Freq {
                control: commsfrequency::Comm1Freq::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_comm2_channel_numeric<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Comm2ChannelNumeric {
                control: commsfrequency::Comm2ChannelNumeric::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_comm2_freq<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Comm2Freq {
                control: commsfrequency::Comm2Freq::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_consoles_dimmer<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ConsolesDimmer {
                control: interiorlightspanel::ConsolesDimmer::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_defog_handle<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DefogHandle {
                control: defogpanel::DefogHandle::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm_mode_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(dispenseremcpanel::EcmModeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcmModeSw {
                control: dispenseremcpanel::EcmModeSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecs_mode_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(environmentcontrolsystempanel::EcsModeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcsModeSw {
                control: environmentcontrolsystempanel::EcsModeSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ejection_handle_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EjectionHandleSw {
                control: ejectionseat::EjectionHandleSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ejection_seat_armed<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ejectionseat::EjectionSeatArmedValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EjectionSeatArmed {
                control: ejectionseat::EjectionSeatArmed::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ejection_seat_mnl_ovrd<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ejectionseat::EjectionSeatMnlOvrdValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EjectionSeatMnlOvrd {
                control: ejectionseat::EjectionSeatMnlOvrd::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emer_jett_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(emergencyjettisonbutton::EmerJettBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmerJettBtn {
                control: emergencyjettisonbutton::EmerJettBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emergency_gear_rotate<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(landinggearhandleandwarningtonesilence::EmergencyGearRotateValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmergencyGearRotate {
                control: landinggearhandleandwarningtonesilence::EmergencyGearRotate::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emergency_parking_brake_pull<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(emergencyandparkingbrakehandle::EmergencyParkingBrakePullValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmergencyParkingBrakePull {
                control: emergencyandparkingbrakehandle::EmergencyParkingBrakePull::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emergency_parking_brake_rotate<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmergencyParkingBrakeRotate {
                control: emergencyandparkingbrakehandle::EmergencyParkingBrakeRotate::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_antiice_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(environmentcontrolsystempanel::EngAntiiceSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngAntiiceSw {
                control: environmentcontrolsystempanel::EngAntiiceSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_crank_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(auxiliarypowerunitpanel::EngineCrankSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineCrankSw {
                control: auxiliarypowerunitpanel::EngineCrankSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_cnt_tank_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuelpanel::ExtCntTankSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtCntTankSw {
                control: fuelpanel::ExtCntTankSw::new(),
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

    pub fn register_ext_position_light_left<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtPositionLightLeft {
                control: externalaircraftmodel::ExtPositionLightLeft::new(),
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
                control: externalaircraftmodel::ExtPositionLightRight::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_pwr_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(groundpowerpanel::ExtPwrSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtPwrSw {
                control: groundpowerpanel::ExtPwrSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_speed_brake<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtSpeedBrake {
                control: externalaircraftmodel::ExtSpeedBrake::new(),
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

    pub fn register_ext_wng_tank_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuelpanel::ExtWngTankSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtWngTankSw {
                control: fuelpanel::ExtWngTankSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fcs_bit_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightessentialcircuitbreakers::FcsBitSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FcsBitSw {
                control: rightessentialcircuitbreakers::FcsBitSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fcs_reset_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightcontrolsystempanel::FcsResetBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FcsResetBtn {
                control: flightcontrolsystempanel::FcsResetBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fire_apu_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FireApuLt {
                control: apufirewarningextinguisherlight::FireApuLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fire_ext_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(firesystems::FireExtBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FireExtBtn {
                control: firesystems::FireExtBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fire_left_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FireLeftLt {
                control: leftenginefirewarningextinguisherlight::FireLeftLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fire_right_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FireRightLt {
                control: rightenginefirewarningextinguisherlight::FireRightLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fire_test_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(firetestpanel::FireTestSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FireTestSw {
                control: firetestpanel::FireTestSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flap_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(selectjettisonbutton::FlapSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlapSw {
                control: selectjettisonbutton::FlapSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flir_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(sensorpanel::FlirSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlirSw {
                control: sensorpanel::FlirSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flood_dimmer<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FloodDimmer {
                control: interiorlightspanel::FloodDimmer::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flp_lg_flaps_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlpLgFlapsLt {
                control: flapslandinggearstoresindicatorpanel::FlpLgFlapsLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flp_lg_full_flaps_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlpLgFullFlapsLt {
                control: flapslandinggearstoresindicatorpanel::FlpLgFullFlapsLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flp_lg_half_flaps_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlpLgHalfFlapsLt {
                control: flapslandinggearstoresindicatorpanel::FlpLgHalfFlapsLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flp_lg_left_gear_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlpLgLeftGearLt {
                control: flapslandinggearstoresindicatorpanel::FlpLgLeftGearLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flp_lg_nose_gear_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlpLgNoseGearLt {
                control: flapslandinggearstoresindicatorpanel::FlpLgNoseGearLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flp_lg_right_gear_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlpLgRightGearLt {
                control: flapslandinggearstoresindicatorpanel::FlpLgRightGearLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_formation_dimmer<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FormationDimmer {
                control: exteriorlightspanel::FormationDimmer::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_dump_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelDumpSw {
                control: fuelpanel::FuelDumpSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gain_switch<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightcontrolsystempanel::GainSwitchValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GainSwitch {
                control: flightcontrolsystempanel::GainSwitch::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gain_switch_cover<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightcontrolsystempanel::GainSwitchCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GainSwitchCover {
                control: flightcontrolsystempanel::GainSwitchCover::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gear_downlock_override_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(landinggearhandleandwarningtonesilence::GearDownlockOverrideBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GearDownlockOverrideBtn {
                control: landinggearhandleandwarningtonesilence::GearDownlockOverrideBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gear_lever<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(landinggearhandleandwarningtonesilence::GearLeverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GearLever {
                control: landinggearhandleandwarningtonesilence::GearLever::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gear_silence_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(landinggearhandleandwarningtonesilence::GearSilenceBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GearSilenceBtn {
                control: landinggearhandleandwarningtonesilence::GearSilenceBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gen_tie_cover<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(generatortiecontrolswitch::GenTieCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GenTieCover {
                control: generatortiecontrolswitch::GenTieCover::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gen_tie_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(generatortiecontrolswitch::GenTieSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GenTieSw {
                control: generatortiecontrolswitch::GenTieSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gnd_pwr1_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(groundpowerpanel::GndPwr1SwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GndPwr1Sw {
                control: groundpowerpanel::GndPwr1Sw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gnd_pwr2_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(groundpowerpanel::GndPwr2SwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GndPwr2Sw {
                control: groundpowerpanel::GndPwr2Sw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gnd_pwr3_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(groundpowerpanel::GndPwr3SwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GndPwr3Sw {
                control: groundpowerpanel::GndPwr3Sw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gnd_pwr4_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(groundpowerpanel::GndPwr4SwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GndPwr4Sw {
                control: groundpowerpanel::GndPwr4Sw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hide_stick_toggle<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ejectionseat::HideStickToggleValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HideStickToggle {
                control: ejectionseat::HideStickToggle::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hmd_off_brt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HmdOffBrt {
                control: mapgainspinrecoverypanel::HmdOffBrt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hook_bypass_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HookBypassSw {
                control: selectjettisonbutton::HookBypassSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hook_lever<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(arrestinghookhandleandlight::HookLeverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HookLever {
                control: arrestinghookhandleandlight::HookLever::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_alt_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(hudcontrolpanel::HudAltSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudAltSw {
                control: hudcontrolpanel::HudAltSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_aoa_indexer<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudAoaIndexer {
                control: hudcontrolpanel::HudAoaIndexer::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_att_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(hudcontrolpanel::HudAttSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudAttSw {
                control: hudcontrolpanel::HudAttSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_balance<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudBalance {
                control: hudcontrolpanel::HudBalance::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_black_lvl<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudBlackLvl {
                control: hudcontrolpanel::HudBlackLvl::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_sym_brt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudSymBrt {
                control: hudcontrolpanel::HudSymBrt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_sym_brt_select<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(hudcontrolpanel::HudSymBrtSelectValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudSymBrtSelect {
                control: hudcontrolpanel::HudSymBrtSelect::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_sym_rej_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(hudcontrolpanel::HudSymRejSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudSymRejSw {
                control: hudcontrolpanel::HudSymRejSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_video_bit<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(hudvideobitpanel::HudVideoBitValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudVideoBit {
                control: hudvideobitpanel::HudVideoBit::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_video_control_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(hudcontrolpanel::HudVideoControlSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudVideoControlSw {
                control: hudcontrolpanel::HudVideoControlSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hyd_ind_brake<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydIndBrake {
                control: selectjettisonbutton::HydIndBrake::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hyd_ind_left<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydIndLeft {
                control: hyd1andhydpressureindicator::HydIndLeft::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hyd_ind_right<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydIndRight {
                control: hyd1andhydpressureindicator::HydIndRight::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hyd_isolate_override_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(missioncomputerandhydraulicisolatepanel::HydIsolateOverrideSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydIsolateOverrideSw {
                control: missioncomputerandhydraulicisolatepanel::HydIsolateOverrideSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ifei {
                control: hudvideorecordpanel::Ifei::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_bingo<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiBingo {
                control: integratedfuelengineindicatorifei::IfeiBingo::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_bingo_texture<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiBingoTexture {
                control: integratedfuelengineindicatorifei::IfeiBingoTexture::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_clock_h<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiClockH {
                control: integratedfuelengineindicatorifei::IfeiClockH::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_clock_m<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiClockM {
                control: integratedfuelengineindicatorifei::IfeiClockM::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_clock_s<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiClockS {
                control: integratedfuelengineindicatorifei::IfeiClockS::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_codes<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiCodes {
                control: integratedfuelengineindicatorifei::IfeiCodes::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_dd1<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiDd1 {
                control: integratedfuelengineindicatorifei::IfeiDd1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_dd2<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiDd2 {
                control: integratedfuelengineindicatorifei::IfeiDd2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_dd3<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiDd3 {
                control: integratedfuelengineindicatorifei::IfeiDd3::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_dd4<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiDd4 {
                control: integratedfuelengineindicatorifei::IfeiDd4::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_dwn_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(integratedfuelengineindicatorifei::IfeiDwnBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiDwnBtn {
                control: integratedfuelengineindicatorifei::IfeiDwnBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_et_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(integratedfuelengineindicatorifei::IfeiEtBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiEtBtn {
                control: integratedfuelengineindicatorifei::IfeiEtBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_ff_l<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiFfL {
                control: integratedfuelengineindicatorifei::IfeiFfL::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_ff_r<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiFfR {
                control: integratedfuelengineindicatorifei::IfeiFfR::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_ff_texture<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiFfTexture {
                control: integratedfuelengineindicatorifei::IfeiFfTexture::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_fuel_down<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiFuelDown {
                control: integratedfuelengineindicatorifei::IfeiFuelDown::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_fuel_up<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiFuelUp {
                control: integratedfuelengineindicatorifei::IfeiFuelUp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_l0_texture<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiL0Texture {
                control: integratedfuelengineindicatorifei::IfeiL0Texture::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_l100_texture<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiL100Texture {
                control: integratedfuelengineindicatorifei::IfeiL100Texture::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_l50_texture<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiL50Texture {
                control: integratedfuelengineindicatorifei::IfeiL50Texture::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_lpointer_texture<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiLpointerTexture {
                control: integratedfuelengineindicatorifei::IfeiLpointerTexture::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_lscale_texture<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiLscaleTexture {
                control: integratedfuelengineindicatorifei::IfeiLscaleTexture::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_mode_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(integratedfuelengineindicatorifei::IfeiModeBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiModeBtn {
                control: integratedfuelengineindicatorifei::IfeiModeBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_oil_press_l<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiOilPressL {
                control: integratedfuelengineindicatorifei::IfeiOilPressL::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_oil_press_r<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiOilPressR {
                control: integratedfuelengineindicatorifei::IfeiOilPressR::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_oil_texture<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiOilTexture {
                control: integratedfuelengineindicatorifei::IfeiOilTexture::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_qty_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(integratedfuelengineindicatorifei::IfeiQtyBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiQtyBtn {
                control: integratedfuelengineindicatorifei::IfeiQtyBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_r100_texture<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiR100Texture {
                control: integratedfuelengineindicatorifei::IfeiR100Texture::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_r50_texture<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiR50Texture {
                control: integratedfuelengineindicatorifei::IfeiR50Texture::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_rpm_l<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiRpmL {
                control: integratedfuelengineindicatorifei::IfeiRpmL::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_rpm_r<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiRpmR {
                control: integratedfuelengineindicatorifei::IfeiRpmR::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_rpm_texture<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiRpmTexture {
                control: integratedfuelengineindicatorifei::IfeiRpmTexture::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_rpointer_texture<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiRpointerTexture {
                control: integratedfuelengineindicatorifei::IfeiRpointerTexture::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_rscale_texture<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiRscaleTexture {
                control: integratedfuelengineindicatorifei::IfeiRscaleTexture::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_sp<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiSp {
                control: integratedfuelengineindicatorifei::IfeiSp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_temp_l<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiTempL {
                control: integratedfuelengineindicatorifei::IfeiTempL::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_temp_r<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiTempR {
                control: integratedfuelengineindicatorifei::IfeiTempR::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_temp_texture<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiTempTexture {
                control: integratedfuelengineindicatorifei::IfeiTempTexture::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_time_set_mode<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiTimeSetMode {
                control: integratedfuelengineindicatorifei::IfeiTimeSetMode::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_timer_h<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiTimerH {
                control: integratedfuelengineindicatorifei::IfeiTimerH::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_timer_m<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiTimerM {
                control: integratedfuelengineindicatorifei::IfeiTimerM::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_timer_s<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiTimerS {
                control: integratedfuelengineindicatorifei::IfeiTimerS::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_up_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(integratedfuelengineindicatorifei::IfeiUpBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiUpBtn {
                control: integratedfuelengineindicatorifei::IfeiUpBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_z_texture<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiZTexture {
                control: integratedfuelengineindicatorifei::IfeiZTexture::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ifei_zone_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(integratedfuelengineindicatorifei::IfeiZoneBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IfeiZoneBtn {
                control: integratedfuelengineindicatorifei::IfeiZoneBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_ant_select_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(antennaselectpanel::IffAntSelectSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffAntSelectSw {
                control: antennaselectpanel::IffAntSelectSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(sensorpanel::InsSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsSw {
                control: sensorpanel::InsSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_inst_pnl_dimmer<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::InstPnlDimmer {
                control: interiorlightspanel::InstPnlDimmer::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_int_wng_tank_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(exteriorlightspanel::IntWngTankSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IntWngTankSw {
                control: exteriorlightspanel::IntWngTankSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ir_cool_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mapgainspinrecoverypanel::IrCoolSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IrCoolSw {
                control: mapgainspinrecoverypanel::IrCoolSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ky58_fill_select<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ky58control::Ky58FillSelectValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ky58FillSelect {
                control: ky58control::Ky58FillSelect::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ky58_mode_select<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ky58control::Ky58ModeSelectValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ky58ModeSelect {
                control: ky58control::Ky58ModeSelect::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ky58_power_select<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ky58control::Ky58PowerSelectValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ky58PowerSelect {
                control: ky58control::Ky58PowerSelect::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ky58_volume<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ky58Volume {
                control: ky58control::Ky58Volume::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_l_gen_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricalpowerpanel::LGenSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LGenSw {
                control: electricalpowerpanel::LGenSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_landing_gear_handle_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LandingGearHandleLt {
                control: landinggearhandleandwarningtonesilence::LandingGearHandleLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_launch_bar_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LaunchBarSw {
                control: selectjettisonbutton::LaunchBarSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ldg_taxi_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(selectjettisonbutton::LdgTaxiSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LdgTaxiSw {
                control: selectjettisonbutton::LdgTaxiSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_brt_ctl<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiBrtCtl {
                control: leftddi::LeftDdiBrtCtl::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_brt_select<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftddi::LeftDdiBrtSelectValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiBrtSelect {
                control: leftddi::LeftDdiBrtSelect::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_cont_ctl<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiContCtl {
                control: leftddi::LeftDdiContCtl::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_crs_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(headingandcoursesetswitches::LeftDdiCrsSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiCrsSw {
                control: headingandcoursesetswitches::LeftDdiCrsSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_hdg_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(headingandcoursesetswitches::LeftDdiHdgSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiHdgSw {
                control: headingandcoursesetswitches::LeftDdiHdgSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_pb01<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftddi::LeftDdiPb01Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiPb01 {
                control: leftddi::LeftDdiPb01::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_pb02<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftddi::LeftDdiPb02Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiPb02 {
                control: leftddi::LeftDdiPb02::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_pb03<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftddi::LeftDdiPb03Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiPb03 {
                control: leftddi::LeftDdiPb03::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_pb04<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftddi::LeftDdiPb04Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiPb04 {
                control: leftddi::LeftDdiPb04::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_pb05<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftddi::LeftDdiPb05Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiPb05 {
                control: leftddi::LeftDdiPb05::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_pb06<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftddi::LeftDdiPb06Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiPb06 {
                control: leftddi::LeftDdiPb06::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_pb07<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftddi::LeftDdiPb07Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiPb07 {
                control: leftddi::LeftDdiPb07::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_pb08<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftddi::LeftDdiPb08Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiPb08 {
                control: leftddi::LeftDdiPb08::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_pb09<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftddi::LeftDdiPb09Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiPb09 {
                control: leftddi::LeftDdiPb09::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_pb10<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftddi::LeftDdiPb10Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiPb10 {
                control: leftddi::LeftDdiPb10::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_pb11<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftddi::LeftDdiPb11Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiPb11 {
                control: leftddi::LeftDdiPb11::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_pb12<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftddi::LeftDdiPb12Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiPb12 {
                control: leftddi::LeftDdiPb12::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_pb13<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftddi::LeftDdiPb13Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiPb13 {
                control: leftddi::LeftDdiPb13::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_pb14<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftddi::LeftDdiPb14Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiPb14 {
                control: leftddi::LeftDdiPb14::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_pb15<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftddi::LeftDdiPb15Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiPb15 {
                control: leftddi::LeftDdiPb15::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_pb16<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftddi::LeftDdiPb16Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiPb16 {
                control: leftddi::LeftDdiPb16::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_pb17<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftddi::LeftDdiPb17Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiPb17 {
                control: leftddi::LeftDdiPb17::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_pb18<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftddi::LeftDdiPb18Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiPb18 {
                control: leftddi::LeftDdiPb18::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_pb19<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftddi::LeftDdiPb19Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiPb19 {
                control: leftddi::LeftDdiPb19::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_ddi_pb20<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftddi::LeftDdiPb20Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftDdiPb20 {
                control: leftddi::LeftDdiPb20::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_fire_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftenginefirewarningextinguisherlight::LeftFireBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftFireBtn {
                control: leftenginefirewarningextinguisherlight::LeftFireBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_fire_btn_cover<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(leftenginefirewarningextinguisherlight::LeftFireBtnCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftFireBtnCover {
                control: leftenginefirewarningextinguisherlight::LeftFireBtnCover::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_louver<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftLouver {
                control: environmentcontrollouver::LeftLouver::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_video_bit<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(todo::LeftVideoBitValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftVideoBit {
                control: todo::LeftVideoBit::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lh_adv_aspj_oh<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LhAdvAspjOh {
                control: lhadvisorypanel::LhAdvAspjOh::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lh_adv_go<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LhAdvGo {
                control: lhadvisorypanel::LhAdvGo::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lh_adv_l_bar_green<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LhAdvLBarGreen {
                control: lhadvisorypanel::LhAdvLBarGreen::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lh_adv_l_bar_red<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LhAdvLBarRed {
                control: lhadvisorypanel::LhAdvLBarRed::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lh_adv_l_bleed<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LhAdvLBleed {
                control: lhadvisorypanel::LhAdvLBleed::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lh_adv_no_go<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LhAdvNoGo {
                control: lhadvisorypanel::LhAdvNoGo::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lh_adv_r_bleed<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LhAdvRBleed {
                control: lhadvisorypanel::LhAdvRBleed::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lh_adv_rec<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LhAdvRec {
                control: lhadvisorypanel::LhAdvRec::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lh_adv_spd_brk<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LhAdvSpdBrk {
                control: lhadvisorypanel::LhAdvSpdBrk::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lh_adv_stby<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LhAdvStby {
                control: lhadvisorypanel::LhAdvStby::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lh_adv_xmit<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LhAdvXmit {
                control: lhadvisorypanel::LhAdvXmit::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lights_test_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(interiorlightspanel::LightsTestSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightsTestSw {
                control: interiorlightspanel::LightsTestSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_low_alt_warn_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LowAltWarnLt {
                control: radaraltimeter::LowAltWarnLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ls_lock<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LsLock {
                control: lockshootlights::LsLock::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ls_shoot<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LsShoot {
                control: lockshootlights::LsShoot::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ls_shoot_strobe<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LsShootStrobe {
                control: lockshootlights::LsShootStrobe::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lst_nflr_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(sensorpanel::LstNflrSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LstNflrSw {
                control: sensorpanel::LstNflrSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ltd_r_arm<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(sensorpanel::LtdRArmValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LtdRArm {
                control: sensorpanel::LtdRArm::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ltd_r_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(sensorpanel::LtdRSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LtdRSw {
                control: sensorpanel::LtdRSw::new(),
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
        F: FnMut(masterarmpanel::MasterArmSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterArmSw {
                control: masterarmpanel::MasterArmSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_caution_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterCautionLt {
                control: mastercautionlight::MasterCautionLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_caution_reset_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mastercautionlight::MasterCautionResetSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterCautionResetSw {
                control: mastercautionlight::MasterCautionResetSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_mode_aa<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(masterarmpanel::MasterModeAaValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterModeAa {
                control: masterarmpanel::MasterModeAa::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_mode_aa_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterModeAaLt {
                control: masterarmpanel::MasterModeAaLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_mode_ag<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(masterarmpanel::MasterModeAgValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterModeAg {
                control: masterarmpanel::MasterModeAg::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_mode_ag_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterModeAgLt {
                control: masterarmpanel::MasterModeAgLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mc_disch<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::McDisch {
                control: masterarmpanel::McDisch::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mc_ready<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::McReady {
                control: masterarmpanel::McReady::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mc_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::McSw {
                control: missioncomputerandhydraulicisolatepanel::McSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mode_selector_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(hudvideorecordpanel::ModeSelectorSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ModeSelectorSw {
                control: hudvideorecordpanel::ModeSelectorSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_obogs_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(loxindicator::ObogsSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ObogsSw {
                control: loxindicator::ObogsSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxy_flow<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxyFlow {
                control: loxindicator::OxyFlow::new(),
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
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PitotHeatSw {
                control: environmentcontrolsystempanel::PitotHeatSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_position_dimmer<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PositionDimmer {
                control: exteriorlightspanel::PositionDimmer::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pressure_alt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PressureAlt {
                control: cockpitaltimeter::PressureAlt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_probe_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuelpanel::ProbeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ProbeSw {
                control: fuelpanel::ProbeSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_r_gen_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricalpowerpanel::RGenSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RGenSw {
                control: electricalpowerpanel::RGenSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radalt_alt_ptr<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadaltAltPtr {
                control: radaraltimeter::RadaltAltPtr::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radalt_green_lamp<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadaltGreenLamp {
                control: radaraltimeter::RadaltGreenLamp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radalt_height<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadaltHeight {
                control: radaraltimeter::RadaltHeight::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radalt_min_height_ptr<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadaltMinHeightPtr {
                control: radaraltimeter::RadaltMinHeightPtr::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radalt_off_flag<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadaltOffFlag {
                control: radaraltimeter::RadaltOffFlag::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radalt_test_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(radaraltimeter::RadaltTestSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadaltTestSw {
                control: radaraltimeter::RadaltTestSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(sensorpanel::RadarSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarSw {
                control: sensorpanel::RadarSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_sw_pull<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(sensorpanel::RadarSwPullValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarSwPull {
                control: sensorpanel::RadarSwPull::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rh_adv_aaa<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RhAdvAaa {
                control: rhadvisorypanel::RhAdvAaa::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rh_adv_ai<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RhAdvAi {
                control: rhadvisorypanel::RhAdvAi::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rh_adv_cw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RhAdvCw {
                control: rhadvisorypanel::RhAdvCw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rh_adv_disp<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RhAdvDisp {
                control: rhadvisorypanel::RhAdvDisp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rh_adv_rcdr_on<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RhAdvRcdrOn {
                control: rhadvisorypanel::RhAdvRcdrOn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rh_adv_sam<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RhAdvSam {
                control: rhadvisorypanel::RhAdvSam::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rh_adv_spare_rh1<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RhAdvSpareRh1 {
                control: rhadvisorypanel::RhAdvSpareRh1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rh_adv_spare_rh2<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RhAdvSpareRh2 {
                control: rhadvisorypanel::RhAdvSpareRh2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rh_adv_spare_rh3<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RhAdvSpareRh3 {
                control: rhadvisorypanel::RhAdvSpareRh3::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rh_adv_spare_rh4<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RhAdvSpareRh4 {
                control: rhadvisorypanel::RhAdvSpareRh4::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rh_adv_spare_rh5<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RhAdvSpareRh5 {
                control: rhadvisorypanel::RhAdvSpareRh5::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_brt_ctl<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiBrtCtl {
                control: rightddi::RightDdiBrtCtl::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_brt_select<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightddi::RightDdiBrtSelectValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiBrtSelect {
                control: rightddi::RightDdiBrtSelect::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_cont_ctl<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiContCtl {
                control: rightddi::RightDdiContCtl::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_pb01<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightddi::RightDdiPb01Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiPb01 {
                control: rightddi::RightDdiPb01::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_pb02<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightddi::RightDdiPb02Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiPb02 {
                control: rightddi::RightDdiPb02::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_pb03<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightddi::RightDdiPb03Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiPb03 {
                control: rightddi::RightDdiPb03::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_pb04<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightddi::RightDdiPb04Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiPb04 {
                control: rightddi::RightDdiPb04::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_pb05<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightddi::RightDdiPb05Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiPb05 {
                control: rightddi::RightDdiPb05::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_pb06<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightddi::RightDdiPb06Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiPb06 {
                control: rightddi::RightDdiPb06::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_pb07<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightddi::RightDdiPb07Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiPb07 {
                control: rightddi::RightDdiPb07::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_pb08<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightddi::RightDdiPb08Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiPb08 {
                control: rightddi::RightDdiPb08::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_pb09<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightddi::RightDdiPb09Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiPb09 {
                control: rightddi::RightDdiPb09::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_pb10<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightddi::RightDdiPb10Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiPb10 {
                control: rightddi::RightDdiPb10::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_pb11<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightddi::RightDdiPb11Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiPb11 {
                control: rightddi::RightDdiPb11::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_pb12<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightddi::RightDdiPb12Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiPb12 {
                control: rightddi::RightDdiPb12::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_pb13<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightddi::RightDdiPb13Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiPb13 {
                control: rightddi::RightDdiPb13::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_pb14<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightddi::RightDdiPb14Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiPb14 {
                control: rightddi::RightDdiPb14::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_pb15<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightddi::RightDdiPb15Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiPb15 {
                control: rightddi::RightDdiPb15::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_pb16<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightddi::RightDdiPb16Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiPb16 {
                control: rightddi::RightDdiPb16::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_pb17<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightddi::RightDdiPb17Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiPb17 {
                control: rightddi::RightDdiPb17::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_pb18<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightddi::RightDdiPb18Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiPb18 {
                control: rightddi::RightDdiPb18::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_pb19<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightddi::RightDdiPb19Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiPb19 {
                control: rightddi::RightDdiPb19::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_ddi_pb20<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightddi::RightDdiPb20Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightDdiPb20 {
                control: rightddi::RightDdiPb20::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_fire_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightenginefirewarningextinguisherlight::RightFireBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightFireBtn {
                control: rightenginefirewarningextinguisherlight::RightFireBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_fire_btn_cover<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rightenginefirewarningextinguisherlight::RightFireBtnCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightFireBtnCover {
                control: rightenginefirewarningextinguisherlight::RightFireBtnCover::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_louver<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightLouver {
                control: environmentcontrollouver::RightLouver::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_video_bit<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(todo::RightVideoBitValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightVideoBit {
                control: todo::RightVideoBit::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rud_trim<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RudTrim {
                control: flightcontrolsystempanel::RudTrim::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rudder_pedal_adjust<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rudderpedaladjustlevel::RudderPedalAdjustValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RudderPedalAdjust {
                control: rudderpedaladjustlevel::RudderPedalAdjust::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_bit_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwrcontrolindicator::RwrBitBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrBitBtn {
                control: rwrcontrolindicator::RwrBitBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_bit_lt<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrBitLt {
                control: rwrcontrolindicator::RwrBitLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_dis_type_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwrcontrolindicator::RwrDisTypeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrDisTypeSw {
                control: rwrcontrolindicator::RwrDisTypeSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_display_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwrcontrolindicator::RwrDisplayBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrDisplayBtn {
                control: rwrcontrolindicator::RwrDisplayBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_display_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrDisplayLt {
                control: rwrcontrolindicator::RwrDisplayLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_dmr_ctrl<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrDmrCtrl {
                control: rwrcontrolindicator::RwrDmrCtrl::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_enable_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrEnableLt {
                control: rwrcontrolindicator::RwrEnableLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_fail_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrFailLt {
                control: rwrcontrolindicator::RwrFailLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_limit_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrLimitLt {
                control: rwrcontrolindicator::RwrLimitLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_lower_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrLowerLt {
                control: rwrcontrolindicator::RwrLowerLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_offset_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwrcontrolindicator::RwrOffsetBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrOffsetBtn {
                control: rwrcontrolindicator::RwrOffsetBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_offset_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrOffsetLt {
                control: rwrcontrolindicator::RwrOffsetLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_power_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwrcontrolindicator::RwrPowerBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrPowerBtn {
                control: rwrcontrolindicator::RwrPowerBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_rwr_intesity<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrRwrIntesity {
                control: rwrcontrolindicator::RwrRwrIntesity::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_special_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwrcontrolindicator::RwrSpecialBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrSpecialBtn {
                control: rwrcontrolindicator::RwrSpecialBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_special_en_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrSpecialEnLt {
                control: rwrcontrolindicator::RwrSpecialEnLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_special_lt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrSpecialLt {
                control: rwrcontrolindicator::RwrSpecialLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_att_warning_flag<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiAttWarningFlag {
                control: standbyattitudereferenceindicator::SaiAttWarningFlag::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_bank<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiBank {
                control: standbyattitudereferenceindicator::SaiBank::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_cage<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(standbyattitudereferenceindicator::SaiCageValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiCage {
                control: standbyattitudereferenceindicator::SaiCage::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_man_pitch_adj<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiManPitchAdj {
                control: standbyattitudereferenceindicator::SaiManPitchAdj::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_pitch<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiPitch {
                control: standbyattitudereferenceindicator::SaiPitch::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_rate_of_turn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiRateOfTurn {
                control: standbyattitudereferenceindicator::SaiRateOfTurn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_set<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiSet {
                control: standbyattitudereferenceindicator::SaiSet::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_slip_ball<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiSlipBall {
                control: standbyattitudereferenceindicator::SaiSlipBall::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_test_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(standbyattitudereferenceindicator::SaiTestBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiTestBtn {
                control: standbyattitudereferenceindicator::SaiTestBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_seat_height_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ejectionseat::SeatHeightSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SeatHeightSw {
                control: ejectionseat::SeatHeightSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sel_jett_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(selectjettisonbutton::SelJettBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SelJettBtn {
                control: selectjettisonbutton::SelJettBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sel_jett_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(selectjettisonbutton::SelJettKnobValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SelJettKnob {
                control: selectjettisonbutton::SelJettKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_select_hmd_lddi_rddi<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(hudvideorecordpanel::SelectHmdLddiRddiValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SelectHmdLddiRddi {
                control: hudvideorecordpanel::SelectHmdLddiRddi::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_select_hud_lddi_rddi<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(hudvideorecordpanel::SelectHudLddiRddiValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SelectHudLddiRddi {
                control: hudvideorecordpanel::SelectHudLddiRddi::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_shldr_harness_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ejectionseat::ShldrHarnessSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ShldrHarnessSw {
                control: ejectionseat::ShldrHarnessSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sj_ctr<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(stationjettisonselect::SjCtrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SjCtr {
                control: stationjettisonselect::SjCtr::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sj_ctr_lt<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::SjCtrLt {
                control: stationjettisonselect::SjCtrLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sj_li<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(stationjettisonselect::SjLiValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SjLi {
                control: stationjettisonselect::SjLi::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sj_li_lt<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::SjLiLt {
                control: stationjettisonselect::SjLiLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sj_lo<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(stationjettisonselect::SjLoValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SjLo {
                control: stationjettisonselect::SjLo::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sj_lo_lt<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::SjLoLt {
                control: stationjettisonselect::SjLoLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sj_ri<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(stationjettisonselect::SjRiValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SjRi {
                control: stationjettisonselect::SjRi::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sj_ri_lt<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::SjRiLt {
                control: stationjettisonselect::SjRiLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sj_ro<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(stationjettisonselect::SjRoValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SjRo {
                control: stationjettisonselect::SjRo::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sj_ro_lt<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::SjRoLt {
                control: stationjettisonselect::SjRoLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_spin_lt<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::SpinLt {
                control: mapgainspinrecoverypanel::SpinLt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_spin_recovery_cover<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mapgainspinrecoverypanel::SpinRecoveryCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SpinRecoveryCover {
                control: mapgainspinrecoverypanel::SpinRecoveryCover::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_spin_recovery_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mapgainspinrecoverypanel::SpinRecoverySwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SpinRecoverySw {
                control: mapgainspinrecoverypanel::SpinRecoverySw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stby_alt10000_ft_cnt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StbyAlt10000FtCnt {
                control: standbyaltimeter::StbyAlt10000FtCnt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stby_alt1000_ft_cnt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StbyAlt1000FtCnt {
                control: standbyaltimeter::StbyAlt1000FtCnt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stby_alt100_ft_ptr<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StbyAlt100FtPtr {
                control: standbyaltimeter::StbyAlt100FtPtr::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stby_asi_airspeed<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StbyAsiAirspeed {
                control: standbyairspeedindicator::StbyAsiAirspeed::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stby_press_alt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StbyPressAlt {
                control: standbyaltimeter::StbyPressAlt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stby_press_set0<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StbyPressSet0 {
                control: standbyaltimeter::StbyPressSet0::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stby_press_set1<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StbyPressSet1 {
                control: standbyaltimeter::StbyPressSet1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stby_press_set2<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StbyPressSet2 {
                control: standbyaltimeter::StbyPressSet2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_strobe_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(exteriorlightspanel::StrobeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::StrobeSw {
                control: exteriorlightspanel::StrobeSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_suit_temp<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SuitTemp {
                control: environmentcontrolsystempanel::SuitTemp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_throttle_friction<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ThrottleFriction {
                control: throttlequadrant::ThrottleFriction::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_to_trim_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightcontrolsystempanel::ToTrimBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ToTrimBtn {
                control: flightcontrolsystempanel::ToTrimBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc0<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::Ufc0Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ufc0 {
                control: upfrontcontrollerufc::Ufc0::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc1<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::Ufc1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ufc1 {
                control: upfrontcontrollerufc::Ufc1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc2<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::Ufc2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ufc2 {
                control: upfrontcontrollerufc::Ufc2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc3<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::Ufc3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ufc3 {
                control: upfrontcontrollerufc::Ufc3::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc4<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::Ufc4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ufc4 {
                control: upfrontcontrollerufc::Ufc4::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc5<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::Ufc5Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ufc5 {
                control: upfrontcontrollerufc::Ufc5::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc6<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::Ufc6Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ufc6 {
                control: upfrontcontrollerufc::Ufc6::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc7<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::Ufc7Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ufc7 {
                control: upfrontcontrollerufc::Ufc7::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc8<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::Ufc8Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ufc8 {
                control: upfrontcontrollerufc::Ufc8::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc9<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::Ufc9Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ufc9 {
                control: upfrontcontrollerufc::Ufc9::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_adf<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::UfcAdfValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcAdf {
                control: upfrontcontrollerufc::UfcAdf::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_ap<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::UfcApValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcAp {
                control: upfrontcontrollerufc::UfcAp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_bcn<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::UfcBcnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcBcn {
                control: upfrontcontrollerufc::UfcBcn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_brt<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcBrt {
                control: upfrontcontrollerufc::UfcBrt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_clr<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::UfcClrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcClr {
                control: upfrontcontrollerufc::UfcClr::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_comm1_display<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcComm1Display {
                control: upfrontcontrollerufc::UfcComm1Display::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_comm1_pull<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::UfcComm1PullValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcComm1Pull {
                control: upfrontcontrollerufc::UfcComm1Pull::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_comm1_vol<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcComm1Vol {
                control: upfrontcontrollerufc::UfcComm1Vol::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_comm2_display<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcComm2Display {
                control: upfrontcontrollerufc::UfcComm2Display::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_comm2_pull<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::UfcComm2PullValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcComm2Pull {
                control: upfrontcontrollerufc::UfcComm2Pull::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_comm2_vol<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcComm2Vol {
                control: upfrontcontrollerufc::UfcComm2Vol::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_dl<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::UfcDlValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcDl {
                control: upfrontcontrollerufc::UfcDl::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_emcon<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::UfcEmconValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcEmcon {
                control: upfrontcontrollerufc::UfcEmcon::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_ent<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::UfcEntValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcEnt {
                control: upfrontcontrollerufc::UfcEnt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_iff<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::UfcIffValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcIff {
                control: upfrontcontrollerufc::UfcIff::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_ils<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::UfcIlsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcIls {
                control: upfrontcontrollerufc::UfcIls::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_ip<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::UfcIpValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcIp {
                control: upfrontcontrollerufc::UfcIp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_onoff<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::UfcOnoffValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcOnoff {
                control: upfrontcontrollerufc::UfcOnoff::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_option_cueing1<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcOptionCueing1 {
                control: upfrontcontrollerufc::UfcOptionCueing1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_option_cueing2<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcOptionCueing2 {
                control: upfrontcontrollerufc::UfcOptionCueing2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_option_cueing3<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcOptionCueing3 {
                control: upfrontcontrollerufc::UfcOptionCueing3::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_option_cueing4<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcOptionCueing4 {
                control: upfrontcontrollerufc::UfcOptionCueing4::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_option_cueing5<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcOptionCueing5 {
                control: upfrontcontrollerufc::UfcOptionCueing5::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_option_display1<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcOptionDisplay1 {
                control: upfrontcontrollerufc::UfcOptionDisplay1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_option_display2<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcOptionDisplay2 {
                control: upfrontcontrollerufc::UfcOptionDisplay2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_option_display3<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcOptionDisplay3 {
                control: upfrontcontrollerufc::UfcOptionDisplay3::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_option_display4<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcOptionDisplay4 {
                control: upfrontcontrollerufc::UfcOptionDisplay4::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_option_display5<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcOptionDisplay5 {
                control: upfrontcontrollerufc::UfcOptionDisplay5::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_os1<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::UfcOs1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcOs1 {
                control: upfrontcontrollerufc::UfcOs1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_os2<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::UfcOs2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcOs2 {
                control: upfrontcontrollerufc::UfcOs2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_os3<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::UfcOs3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcOs3 {
                control: upfrontcontrollerufc::UfcOs3::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_os4<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::UfcOs4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcOs4 {
                control: upfrontcontrollerufc::UfcOs4::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_os5<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::UfcOs5Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcOs5 {
                control: upfrontcontrollerufc::UfcOs5::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_scratchpad_number_display<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcScratchpadNumberDisplay {
                control: upfrontcontrollerufc::UfcScratchpadNumberDisplay::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_scratchpad_string1_display<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcScratchpadString1Display {
                control: upfrontcontrollerufc::UfcScratchpadString1Display::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_scratchpad_string2_display<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcScratchpadString2Display {
                control: upfrontcontrollerufc::UfcScratchpadString2Display::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_tcn<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(upfrontcontrollerufc::UfcTcnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcTcn {
                control: upfrontcontrollerufc::UfcTcn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_volt_e<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VoltE {
                control: electricalpowerpanel::VoltE::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_volt_u<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VoltU {
                control: electricalpowerpanel::VoltU::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vsi<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vsi {
                control: standbyrateofclimbindicator::Vsi::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_warn_caution_dimmer<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WarnCautionDimmer {
                control: interiorlightspanel::WarnCautionDimmer::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wing_fold_pull<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(wingfoldswitch::WingFoldPullValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::WingFoldPull {
                control: wingfoldswitch::WingFoldPull::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wing_fold_rotate<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(wingfoldswitch::WingFoldRotateValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::WingFoldRotate {
                control: wingfoldswitch::WingFoldRotate::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wshield_anti_ice_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(defogpanel::WshieldAntiIceSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::WshieldAntiIceSw {
                control: defogpanel::WshieldAntiIceSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn build(self) -> Fa18cHornet<'a, MAX_CONTROLS, MAX_PACKET_DATA> {
        Fa18cHornet {
            parser: ProtocolParser::new(),
            controls: self.controls,
            saw_frame: false,
        }
    }
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> Default
    for Fa18cHornetBuilder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
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

pub struct Fa18cHornet<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    parser: ProtocolParser<MAX_PACKET_DATA>,
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    saw_frame: bool,
}

impl<const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    Fa18cHornet<'_, MAX_CONTROLS, MAX_PACKET_DATA>
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
