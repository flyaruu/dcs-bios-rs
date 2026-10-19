use core::marker::PhantomData;

use heapless::Vec;

use crate::protocol::{FeedError, Packet, ProtocolParser, Sink};
use crate::{
    AnalogDial, AnalogGauge, Control, Display, FixedStepDial, Led, LimitedDial, Metadata, Selector,
};

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

macro_rules! wrapped_integer {
    ($name:ident, $inner:ident, $address:expr) => {
        pub struct $name($inner);
        impl $name {
            pub const fn new() -> Self {
                Self($inner::new($address, 0xffff, 0))
            }
            pub const fn value(&self) -> u16 {
                self.0.value()
            }
        }
        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
        impl Control for $name {
            fn accept(&mut self, packet: Packet<'_>) {
                self.0.accept(packet);
            }
            fn commit(&mut self) -> bool {
                self.0.commit()
            }
        }
    };
}

wrapped_integer!(CanopyValue, AnalogGauge, 30846);
wrapped_integer!(RwrVol, LimitedDial, 30776);
wrapped_integer!(BaroPressure, AnalogDial, 30782);
wrapped_integer!(BingoSet, FixedStepDial, 30760);

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

pub type Comm1StringFreq = Display<7>;
pub const fn comm1_string_freq() -> Comm1StringFreq {
    Display::new(30744, 7)
}

pub type SelectorCallback<'a> = &'a mut dyn FnMut(u16);
pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

pub enum RegisteredControl<'a> {
    Selector {
        control: AcnipMode,
        callback: SelectorCallback<'a>,
    },
    JettMode {
        control: JettMode,
        callback: SelectorCallback<'a>,
    },
    AnalogGauge {
        control: CanopyValue,
        callback: SelectorCallback<'a>,
    },
    Led {
        control: JettStation1Light,
        callback: LedCallback<'a>,
    },
    LimitedDial {
        control: RwrVol,
        callback: SelectorCallback<'a>,
    },
    Display {
        control: Comm1StringFreq,
        callback: DisplayCallback<'a>,
    },
    AnalogDial {
        control: BaroPressure,
        callback: SelectorCallback<'a>,
    },
    FixedStepDial {
        control: BingoSet,
        callback: SelectorCallback<'a>,
    },
    Metadata {
        control: Metadata,
        callback: SelectorCallback<'a>,
    },
}

impl RegisteredControl<'_> {
    fn accept(&mut self, packet: Packet<'_>) {
        match self {
            Self::Selector { control, .. } => control.accept(packet),
            Self::JettMode { control, .. } => control.accept(packet),
            Self::AnalogGauge { control, .. } => control.accept(packet),
            Self::Led { control, .. } => control.accept(packet),
            Self::LimitedDial { control, .. } => control.accept(packet),
            Self::Display { control, .. } => control.accept(packet),
            Self::AnalogDial { control, .. } => control.accept(packet),
            Self::FixedStepDial { control, .. } => control.accept(packet),
            Self::Metadata { control, .. } => control.accept(packet),
        }
    }

    fn commit(&mut self) {
        match self {
            Self::Selector { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::JettMode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AnalogGauge { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Led { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LimitedDial { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Display { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AnalogDial { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FixedStepDial { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Metadata { control, callback } => {
                if control.commit() {
                    callback(control.value());
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

    pub fn register_selector<F>(
        &mut self,
        control: AcnipMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Selector { control, callback })
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

    pub fn register_analog_gauge<F>(
        &mut self,
        control: CanopyValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AnalogGauge { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_led<F>(
        &mut self,
        control: JettStation1Light,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Led { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_limited_dial<F>(
        &mut self,
        control: RwrVol,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LimitedDial { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_display<F>(
        &mut self,
        control: Comm1StringFreq,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Display { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_analog_dial<F>(
        &mut self,
        control: BaroPressure,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AnalogDial { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fixed_step_dial<F>(
        &mut self,
        control: BingoSet,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FixedStepDial { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_metadata<F>(
        &mut self,
        control: Metadata,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Metadata { control, callback })
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
