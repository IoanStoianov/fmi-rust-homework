use std::str;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum PacketError {
    InvalidPacket,
    InvalidChecksum,
    UnknownProtocolVersion,
    CorruptedMessage,
}

impl fmt::Display for PacketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match &self {
            PacketError::InvalidPacket => "InvalidPacket",
            PacketError::InvalidChecksum => "InvalidChecksum",
            PacketError::UnknownProtocolVersion => "UnknownProtocolVersion",
            PacketError::CorruptedMessage => "CorruptedMessage",
        };

        write!(f, "{}", msg)
    }
}

impl std::error::Error for PacketError {}

#[derive(PartialEq, Debug)]
pub struct Packet {
    version: u8,
    length: u8,
    payload: Vec<u8>,
    checksum: u32,
}

impl Clone for Packet {
    fn clone(&self) -> Self {
        let x = &self.payload();
        Packet{
            version: self.version,
            length: self.length,
            payload: x.to_vec(),
            checksum: self.checksum,
        }
    }
}

impl Packet {

    pub fn from_source(source: &[u8], size: u8) -> (Self, &[u8]) {
        if size == 0 {
            panic!("SOS");
        }

        let len:usize = source.len();
        let mut end:usize = size as usize;
        
        if end > len{ end = len; }

        let pay = &source[0..end];
        let rem = &source[end..len];

        let pay = pay.to_vec();

        let mut sum: u32 = 0;

        for elem in &pay {
            sum += *elem as u32
        }

        let pac = Packet{version: 1, length: end as u8, payload: pay, checksum: sum};
        (pac,rem)
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload[..]
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut ser: Vec<u8> = vec![self.version, self.length].iter().chain(&self.payload).map(|&x|x).collect();
        let bytes = self.checksum.to_be_bytes();
        for byte in &bytes{
            ser.push(*byte)
        }

        ser
    }

    pub fn deserialize(bytes: &[u8]) -> Result<(Packet, &[u8]), PacketError> {
        let byte_len = bytes.len();
        if byte_len < 2 { 
            return Err(PacketError::InvalidPacket);
        }
        let version: u8 = bytes[0];
        let length: u8 = bytes[1];
        if version != 1 { 
            return Err(PacketError::UnknownProtocolVersion);
        }
        let pay_size:usize = (length as usize) + 2;
        if byte_len < 4+pay_size { 
            return Err(PacketError::InvalidPacket);
        }
        let payload = (&bytes[2..pay_size]).to_vec();;

        let mut sum: u32 = 0;

        for elem in &payload {
            sum += *elem as u32
        }

        let checksum = &bytes[pay_size..pay_size+4];
        let bytes_of_sum = sum.to_be_bytes();
        for (i, &item) in checksum.iter().enumerate() {
            if item != bytes_of_sum[i] { 
                return Err(PacketError::InvalidChecksum);
            }
        }

        let rem = &bytes[pay_size+4..(byte_len as usize)];

        Ok((Packet{version,length,payload, checksum: sum}, rem))
    }
}

#[derive(Debug)]
pub struct PacketSerializer {
    packets:  Vec<Packet>,
    index: usize,
}


impl Iterator for PacketSerializer{
    type Item =  Packet;
    

    fn next(& mut self) -> Option<Self::Item> {
        let result = match self.packets.get(self.index) {
            Some(x) => x,
            None => return None,
        };
        self.index += 1;
        Some(result.clone())
    }
}

pub trait Packetable: Sized {
    fn to_packets(&self, packet_size: u8) -> PacketSerializer;
    fn to_packet_data(&self, packet_size: u8) -> Vec<u8>;
    fn from_packet_data(packet_data: &[u8]) -> Result<Self, PacketError>;
}

impl Packetable for String {

    fn to_packets(&self, packet_size: u8) -> PacketSerializer {
        let  mut packets: Vec<Packet> = Vec::new();
        let bytes = self.as_bytes();
        
        let mut start:usize = 0; 
        loop {
            let (current,b) = Packet::from_source(&bytes[start..], packet_size);
            packets.push(current);
            start += packet_size as usize;
            if b.len() == 0 {break}
        }
        PacketSerializer{ packets, index: 0}
    }

    fn to_packet_data(&self, packet_size: u8) -> Vec<u8> {
        let mut ser: Vec<u8> = Vec::new();

        let iterator = Packetable::to_packets(self, packet_size);

        for item in iterator {
            let mut serialized_item = item.serialize();
            ser.append(&mut serialized_item);
        }

        ser
    }

    fn from_packet_data(packet_data: &[u8]) -> Result<Self, PacketError> {
        let mut packets: Vec<u8> = Vec::new();
        let mut bytes = packet_data;
        
        loop {
            let res = Packet::deserialize(&bytes);
            match res{
                Ok((packet,rem)) => {
                    let payload = packet.payload();
                    packets.append(&mut payload.to_vec());
                    bytes = rem;
                }
                Err(x) => return Err(x),
            }
            if bytes.len() == 0 {break}
        }

        let result = match str::from_utf8(&packets) {
            Ok(value) => Ok(value.to_string()),
            Err(_) => Err(PacketError::CorruptedMessage),
        };

        result
    }
}


#[test]
fn test_basic_packets() {
    let source = b"hello";
    let (packet, remainder) = Packet::from_source(source, 100);

    assert_eq!(packet.payload().len(), source.len());
    assert_eq!(remainder, b"");
    assert!(packet.serialize().len() > 0);

    if let Err(_) = Packet::deserialize(&packet.serialize()) {
        assert!(false, "Couldn't deserialize serialized packet");
    }
}

#[test]
fn test_basic_iteration() {
    let source = String::from("hello");
    let packets = source.to_packets(100).collect::<Vec<Packet>>();
    assert!(packets.len() > 0);

    let data = source.to_packet_data(100);
    assert!(data.len() > 0);

    if let Err(_) = String::from_packet_data(&data) {
        assert!(false, "Couldn't deserialize serialized packet data");
    }
}
