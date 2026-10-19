#![no_std]

#[path = "av8bna.rs"]
mod av8bna;
mod controls;
mod protocol;

pub use av8bna::{
    AcnipMode, Av8bna, Av8bnaBuilder, BaroPressure, BingoSet, CanopyValue, Comm1StringFreq,
    JettMode, JettStation1Light, RegisteredControl, RwrVol, comm1_string_freq,
};
pub use controls::{
    AnalogDial, AnalogGauge, Control, Display, FixedStepDial, Led, LimitedDial, Metadata, Selector,
};
pub use protocol::{FeedError, Packet, ParseError, ProtocolParser, Sink};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_callbacks_run_after_the_next_frame_sync() {
        let mut observed = 0;
        let mut on_mode = |value| observed = value;
        let mut aircraft = Av8bnaBuilder::<1, 16>::new();
        aircraft
            .register_selector(AcnipMode::new(), &mut on_mode)
            .unwrap();
        let mut aircraft = aircraft.build();

        aircraft.feed(&[0x55, 0x55, 0x55, 0x55]).unwrap();
        aircraft.feed(&[0x56, 0x78, 2, 0, 0, 1]).unwrap();
        aircraft.feed(&[0x55, 0x55, 0x55, 0x55]).unwrap();
        drop(aircraft);
        assert_eq!(observed, 1);
    }

    #[test]
    fn jett_mode_callback_receives_the_selector_value() {
        let mut observed = 0;
        let mut on_mode = |value| observed = value;
        let mut builder = Av8bnaBuilder::<1, 16>::new();
        builder
            .register_jett_mode(JettMode::new(), &mut on_mode)
            .unwrap();
        let mut aircraft = builder.build();

        aircraft.feed(&[0x55, 0x55, 0x55, 0x55]).unwrap();
        aircraft
            .feed(&[0x34, 0x78, 0x02, 0x00, 0x00, 0x10])
            .unwrap();
        aircraft.feed(&[0x55, 0x55, 0x55, 0x55]).unwrap();
        drop(aircraft);
        assert_eq!(observed, 4);
    }

    #[test]
    fn display_callback_receives_a_complete_staged_string() {
        let mut observed = [b' '; 7];
        let mut on_display = |value: &[u8]| observed.copy_from_slice(value);
        let mut builder = Av8bnaBuilder::<1, 16>::new();
        builder
            .register_display(comm1_string_freq(), &mut on_display)
            .unwrap();
        let mut aircraft = builder.build();

        aircraft.feed(&[0x55, 0x55, 0x55, 0x55]).unwrap();
        aircraft.feed(&[0x18, 0x78, 2, 0, b'1', b'2']).unwrap();
        aircraft
            .feed(&[0x1a, 0x78, 5, 0, b'3', b'4', b'5', b'6', b'7'])
            .unwrap();
        aircraft.feed(&[0x55, 0x55, 0x55, 0x55]).unwrap();
        drop(aircraft);
        assert_eq!(&observed, b"1234567");
    }
}
