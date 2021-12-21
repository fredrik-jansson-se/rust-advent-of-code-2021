use std::fs;

pub fn run() -> anyhow::Result<()> {
    let input = fs::read_to_string("day16.txt").unwrap();
    println!("day16-1: {}", run_1(&input)?);
    println!("day16-2: {}", run_2(&input)?);
    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let input = str_to_vec(input);
    let (_, packet) =
        parse_packet((&input, 0)).map_err(|e| anyhow::anyhow!("Failed to parse: {:?}", e))?;

    Ok(vsn_sum(packet))
}

fn vsn_sum((vsn, p): (usize, Packet)) -> usize {
    match p {
        Packet::Literal(_) => vsn,
        Packet::Operator(v) => v.into_iter().map(vsn_sum).sum::<usize>() + vsn,
    }
}

fn run_2(input: &str) -> anyhow::Result<usize> {
    let input = str_to_vec(input);
    let (_, _packet) =
        parse_packet((&input, 0)).map_err(|e| anyhow::anyhow!("Failed to parse: {:?}", e))?;

    Ok(0)
}

#[derive(Debug, PartialEq)]
enum Packet {
    Literal(u64),
    Operator(Vec<(usize, Packet)>),
}

fn parse_literal(io: (&[u8], usize)) -> nom::IResult<(&[u8], usize), Packet> {
    println!("parse_literal: {:?}", io);
    for v in io.0.iter() {
        print!("{:#b}", v);
    }
    println!();
    let high_value = nom::sequence::preceded(
        nom::bits::complete::tag(1, 1usize),
        nom::bits::complete::take(4usize),
    );

    let mut low_value = nom::sequence::preceded(
        nom::bits::complete::tag(0, 1usize),
        nom::bits::complete::take(4usize),
    );

    let (io, highs): (_, Vec<u64>) = nom::multi::many0(high_value)(io)?;
    for h in &highs {
        println!("high: {:#b}", h);
    }
    for v in io.0.iter() {
        print!("{:#b}", v);
    }
    println!();
    let (io, low): (_, u64) = low_value(io)?;
    println!("low: {:#b} - {}", low, low);
    //

    let mut val = 0;
    for h in highs {
        val = (val << 4) + h;
    }

    val = (val << 4) + low;

    Ok((io, Packet::Literal(val)))
}

fn parse_operator(io: (&[u8], usize)) -> nom::IResult<(&[u8], usize), Packet> {
    #[derive(Debug)]
    enum LT {
        Bits(usize),
        Packets(usize),
    }

    let total_length_bits = nom::combinator::map(
        nom::sequence::preceded(
            nom::bits::complete::tag(0, 1usize),
            nom::bits::complete::take(15usize),
        ),
        LT::Bits,
    );

    let total_length_packets = nom::combinator::map(
        nom::sequence::preceded(
            nom::bits::complete::tag(1, 1usize),
            nom::bits::complete::take(11usize),
        ),
        LT::Packets,
    );

    let (mut io, len) = nom::branch::alt((total_length_packets, total_length_bits))(io)?;

    dbg! {&len};

    match len {
        LT::Packets(num_packets) => {
            let (io, packtes) = nom::multi::many_m_n(num_packets, num_packets, parse_packet)(io)?;

            println!("Read {} packets", num_packets);

            // After we're done with n packets, we stop
            // let io: (&[u8], usize) = (&[], 0);
            Ok((io, Packet::Operator(packtes)))
        }

        LT::Bits(bits) => {
            let start_size = io.0.len() * 8 - io.1;

            let mut packets = Vec::new();
            loop {
                let (new_io, new_packet) = parse_packet(io).unwrap();
                io = new_io;
                packets.push(new_packet);
                let size_now = io.0.len() * 8 - io.1;
                if (start_size - size_now) > bits {
                    println!("Read {} bits ({} bits left)", bits, size_now);
                    break;
                }
            }

            Ok((io, Packet::Operator(packets)))
        }
    }
}

fn parse_packet(io: (&[u8], usize)) -> nom::IResult<(&[u8], usize), (usize, Packet)> {
    println!("==> parse_packet");
    let (io, version): (_, usize) = nom::bits::complete::take(3usize)(io)?;
    let (io, typ): (_, u8) = nom::bits::complete::take(3usize)(io)?;

    dbg! {version};
    dbg! {typ};

    match typ {
        4 => {
            let (io, packet) = parse_literal(io)?;
            println!("<== parse_packet");
            Ok((io, (version, packet)))
        }
        _ => {
            let (io, packet) = parse_operator(io)?;
            println!("<== parse_packet");
            Ok((io, (version, packet)))
        }
    }
}

fn str_to_vec(i: &str) -> Vec<u8> {
    let mut hex_bytes = i
        .as_bytes()
        .iter()
        .filter_map(|b| match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            b'A'..=b'F' => Some(b - b'A' + 10),
            _ => None,
        })
        .fuse();

    let mut bytes = Vec::new();
    while let (Some(h), Some(l)) = (hex_bytes.next(), hex_bytes.next()) {
        bytes.push(h << 4 | l)
    }
    bytes
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc16_parse() {
        // let input = super::str_to_vec("D2FE28");
        // let ((io, _), (vsn, res)) = super::parse_packet((&input, 0)).unwrap();
        // assert_eq!(io.len(), 0);
        // assert_eq!(vsn, 6);
        // assert_eq!(res, super::Packet::Literal(2021));

        // let input = super::str_to_vec("38006F45291200");
        // let _ = super::parse_packet((&input, 0)).unwrap();

        let input = super::str_to_vec("EE00D40C823060");
        let _ = super::parse_packet((&input, 0)).unwrap();

        let input = super::str_to_vec("8A004A801A8002F478");
        let _ = super::parse_packet((&input, 0)).unwrap();
    }

    #[test]
    fn aoc16_run_1() {
        // assert_eq!(super::run_1("8A004A801A8002F478").unwrap(), 16);
        assert_eq!(super::run_1("620080001611562C8802118E34").unwrap(), 12);
        // assert_eq!(super::run_1("C0015000016115A2E0802F182340").unwrap(), 23);
        // assert_eq!(super::run_1("A0016C880162017C3686B18A3D4780").unwrap(), 31);
    }
}
