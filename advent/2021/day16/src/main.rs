use std::io::{Error, Cursor, ErrorKind};

use bitstream_io::{FromBitStream, BitReader, BigEndian, BitRead, BitWriter, BitWrite};
use hex::decode;

#[derive(Debug)]
enum Operator {
    Sum,
    Product,
    Min,
    Max,
    GT,
    LT,
    EQ
}

#[derive(Debug)]
enum PacketBody {
    Literal(u64),
    Operator(Operator, Vec<Packet>)
}

#[derive(Debug)]
struct Packet {
    version: u8,
    body: PacketBody
}

impl Packet {
    fn version_sum(&self) -> u64 {
        self.version as u64 + match &self.body {
            PacketBody::Literal(_) => 0,
            PacketBody::Operator(_, packets) => packets.iter().map(|p| p.version_sum() ).sum()
        }
    }

    fn eval(&self) -> u64 {
        use Operator::*;
        match &self.body {
            PacketBody::Literal(lit) => *lit,
            PacketBody::Operator(op, packets) => {
                let mut evaled = packets.iter().map(|p| p.eval());
                match op {
                    Sum => evaled.sum(),
                    Product => evaled.product(),
                    Min => evaled.min().unwrap(),
                    Max => evaled.max().unwrap(),
                    binop => {
                        let a = evaled.next().unwrap();
                        let b = evaled.next().unwrap();
                        match binop {
                            GT => a > b,
                            LT => a < b,
                            EQ => a == b,
                            _ => unreachable!()
                        }.into()
                    }
                }
            }
        }
    }
}

impl FromBitStream for Packet {
    type Error = Error;
    fn from_reader<R: bitstream_io::BitRead + ?Sized>(r: &mut R) -> Result<Self, Self::Error>
    where
        Self: Sized {
        let version = r.read(3)?;
        let type_id = r.read(3)?;
        let body = match type_id {
            4 => {
                println!("reading literal");
                let mut lit = 0u64;
                loop {
                    let cont = r.read_bit()?;
                    lit = (lit << 4) + r.read::<u64>(4)?;
                    if !cont {
                        break;
                    }
                }
                PacketBody::Literal(lit)
            },
            id => {
                println!("reading operator: {}", id);
                let mut packets = vec![];
                if r.read_bit()? { //number of sub-packets
                    let num_packets = r.read(11)?;
                    println!("reading sub packets: {}", num_packets);
                    for _ in 0..num_packets {
                        packets.push(r.parse()?);
                    }
                } else { //number of bits
                    let mut to_read = r.read(15)?;
                    println!("buffering bits: {}", to_read);

                    //buffer that many bits
                    let mut buf: Vec<u8> = vec![];
                    let mut writer = BitWriter::endian(&mut buf, BigEndian);
                    while to_read != 0 {
                        let bits = if to_read > 64 { 64 } else { to_read };
                        writer.write(bits, r.read::<u64>(bits)?)?;
                        to_read -= bits;
                    }
                    writer.byte_align()?;

                    //read out the packets
                    println!("reading buffered packets: {}", buf.len());
                    let mut cursor = Cursor::new(&buf);
                    let mut reader = BitReader::endian(&mut cursor, BigEndian);
                    loop {
                        match reader.parse::<Packet>() {
                            Ok(packet) => packets.push(packet),
                            //assuming that the only way we get this if we've read all the bits
                            Err(err) if err.kind() == ErrorKind::UnexpectedEof => break,
                            err => return err
                        }
                    }
                }

                use Operator::*;
                let op = match id {
                    0 => Sum,
                    1 => Product,
                    2 => Min,
                    3 => Max,
                    5 => GT,
                    6 => LT,
                    7 => EQ,
                    _ => unreachable!()
                };

                PacketBody::Operator(op, packets)
            }
        };

        Ok(Packet { version, body })
    }
}

fn parse_input(input: &str) -> anyhow::Result<Packet> {
    let data = decode(input.trim())?;
    let mut cursor = Cursor::new(&data);
    let mut reader = BitReader::endian(&mut cursor, BigEndian);

    Ok(reader.parse()?)
}

#[test]
fn test_eval() {
    let tests = [("C200B40A82", 3),
    ("04005AC33890", 54),
    ("880086C3E88112", 7),
    ("CE00C43D881120", 9),
    ("D8005AC2A8F0", 1),
    ("F600BC2D8F", 0),
    ("9C005AC2F8F0", 0),
    ("9C0141080250320F1802104A08", 1)];

    for (hex, ex) in tests {
        let packet = parse_input(hex).unwrap();
        assert_eq!(packet.eval(), ex);
    }
}

#[test]
fn test_parse_and_sum() {
    let packet = parse_input("8A004A801A8002F478").unwrap();
    assert_eq!(packet.version_sum(), 16);

    let packet = parse_input("620080001611562C8802118E34").unwrap();
    assert_eq!(packet.version_sum(), 12);

    let packet = parse_input("C0015000016115A2E0802F182340").unwrap();
    assert_eq!(packet.version_sum(), 23);

    let packet = parse_input("A0016C880162017C3686B18A3D4780").unwrap();
    assert_eq!(packet.version_sum(), 31);
}


fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let packet = parse_input(&input)?;
    println!("version sum: {}", packet.version_sum());
    println!("eval result: {}", packet.eval());

    Ok(())
}
