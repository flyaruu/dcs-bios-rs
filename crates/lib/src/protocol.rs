pub const SYNC_BYTE: u8 = 0x55;

#[derive(Clone, Copy)]
pub struct Packet<'a> {
    pub address: u16,
    pub data: &'a [u8],
}

impl Packet<'_> {
    pub fn end_address(&self) -> u16 {
        self.address + self.data.len() as u16
    }
}

pub trait Sink {
    fn on_write(&mut self, packet: Packet<'_>);
    fn on_frame_sync(&mut self);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseError {
    PacketTooLarge,
}

pub struct ProtocolParser<const MAX_PACKET_DATA: usize> {
    header: [u8; 4],
    header_len: usize,
    data: [u8; MAX_PACKET_DATA],
    data_len: usize,
    expected_len: usize,
    sync_run: u8,
}

impl<const MAX_PACKET_DATA: usize> ProtocolParser<MAX_PACKET_DATA> {
    pub const fn new() -> Self {
        Self {
            header: [0; 4],
            header_len: 0,
            data: [0; MAX_PACKET_DATA],
            data_len: 0,
            expected_len: 0,
            sync_run: 0,
        }
    }

    pub fn feed<S: Sink>(&mut self, bytes: &[u8], sink: &mut S) -> Result<(), ParseError> {
        for &byte in bytes {
            if byte == SYNC_BYTE {
                self.sync_run = self.sync_run.saturating_add(1);
                if self.sync_run == 4 {
                    self.reset_packet();
                    self.sync_run = 0;
                    sink.on_frame_sync();
                }
                continue;
            }

            self.flush_sync_bytes(sink)?;
            self.consume_packet_byte(byte, sink)?;
        }
        Ok(())
    }

    fn flush_sync_bytes<S: Sink>(&mut self, sink: &mut S) -> Result<(), ParseError> {
        let count = self.sync_run;
        self.sync_run = 0;
        for _ in 0..count {
            self.consume_packet_byte(SYNC_BYTE, sink)?;
        }
        Ok(())
    }

    fn consume_packet_byte<S: Sink>(&mut self, byte: u8, sink: &mut S) -> Result<(), ParseError> {
        if self.header_len < self.header.len() {
            self.header[self.header_len] = byte;
            self.header_len += 1;
            if self.header_len == self.header.len() {
                self.expected_len =
                    usize::from(u16::from_le_bytes([self.header[2], self.header[3]]));
                if self.expected_len > MAX_PACKET_DATA {
                    self.reset_packet();
                    return Err(ParseError::PacketTooLarge);
                }
                if self.expected_len == 0 {
                    self.emit_packet(sink);
                }
            }
            return Ok(());
        }

        self.data[self.data_len] = byte;
        self.data_len += 1;
        if self.data_len == self.expected_len {
            self.emit_packet(sink);
        }
        Ok(())
    }

    fn emit_packet<S: Sink>(&mut self, sink: &mut S) {
        let address = u16::from_le_bytes([self.header[0], self.header[1]]);
        sink.on_write(Packet {
            address,
            data: &self.data[..self.data_len],
        });
        self.reset_packet();
    }

    fn reset_packet(&mut self) {
        self.header_len = 0;
        self.data_len = 0;
        self.expected_len = 0;
    }
}

impl<const MAX_PACKET_DATA: usize> Default for ProtocolParser<MAX_PACKET_DATA> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeedError {
    Parse(ParseError),
}
