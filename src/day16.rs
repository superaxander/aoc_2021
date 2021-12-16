use std::io;

use crate::common;

#[derive(Debug)]
enum PacketData {
    Literal(usize),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    p_ver: usize,
    p_data: PacketData,
}

macro_rules! get_bit {
    ($bits:expr, $idx:ident) => {{
        let mut bit = ($bits[$idx / 64] >> (63 - $idx % 64)) & 1;
        $idx += 1;
        bit
    }};
}

macro_rules! get_bits {
    ($bits:expr, $idx:ident, $size:expr) => {{
        let mut val = 0;
        for _ in 0..$size {
            val = (val << 1) | get_bit!($bits, $idx);
        }
        val
    }};
}

fn parse_packet(bits: &[usize], idx: usize, is_sub_packet: bool) -> (usize, Packet) {
    let mut idx = idx;
    let p_ver = get_bits!(bits, idx, 3);
    let p_type = get_bits!(bits, idx, 3);

    match p_type {
        4 => {
            let mut num = 0;
            loop {
                let n = get_bits!(bits, idx, 5);
                num = (num << 4) | (n & 0b1111);
                if n & 0b10000 == 0 {
                    break;
                }
            }
            if !is_sub_packet && idx % 4 != 0 {
                assert_eq!(get_bits!(bits, idx, 4 - (idx % 4)), 0);
            }
            (
                idx,
                Packet {
                    p_ver,
                    p_data: PacketData::Literal(num),
                },
            )
        }
        _ => {
            let mut sub_packets;
            if get_bit!(bits, idx) > 0 {
                let no_of_sub_packets = get_bits!(bits, idx, 11);
                sub_packets = Vec::with_capacity(no_of_sub_packets);
                for _ in 0..no_of_sub_packets {
                    let (i, sub_packet) = parse_packet(bits, idx, true);
                    idx = i;
                    sub_packets.push(sub_packet);
                }
            } else {
                let mut final_idx = get_bits!(bits, idx, 15);
                final_idx += idx;
                sub_packets = Vec::new();
                while idx < final_idx {
                    let (i, sub_packet) = parse_packet(bits, idx, true);
                    idx = i;
                    sub_packets.push(sub_packet);
                }
                assert_eq!(idx, final_idx)
            }
            if !is_sub_packet && idx % 4 != 0 {
                assert_eq!(get_bits!(bits, idx, 4 - (idx % 4)), 0);
            }
            (
                idx,
                Packet {
                    p_ver,
                    p_data: match p_type {
                        0 => PacketData::Sum(sub_packets),
                        1 => PacketData::Product(sub_packets),
                        2 => PacketData::Minimum(sub_packets),
                        3 => PacketData::Maximum(sub_packets),
                        5 => PacketData::GreaterThan(sub_packets),
                        6 => PacketData::LessThan(sub_packets),
                        7 => PacketData::EqualTo(sub_packets),
                        _ => panic!("Unknown packet type: {}", p_type),
                    },
                },
            )
        }
    }
}

pub fn main() -> io::Result<(usize, usize)> {
    let lines = common::read_lines("inputs/16.txt")?;

    let mut bit_string = Vec::new();

    let mut idx = 0;

    for line in lines {
        let line = line?;
        let line = line.trim();

        for c in line.chars() {
            match c {
                '0'..='9' => {
                    if idx % 64 == 0 {
                        bit_string.push(c as usize - '0' as usize);
                    } else {
                        let l = bit_string.len();
                        bit_string[l - 1] = (bit_string[l - 1] << 4) | (c as usize - '0' as usize)
                    }
                    idx += 4;
                }
                'A'..='F' => {
                    if idx % 64 == 0 {
                        bit_string.push(c as usize - 'A' as usize + 10);
                    } else {
                        let l = bit_string.len();
                        bit_string[l - 1] =
                            (bit_string[l - 1] << 4) | (c as usize - 'A' as usize + 10)
                    }
                    idx += 4;
                }

                _ => panic!(
                    "Only allow hexadecimal characters! '{}' is not hexadecimal",
                    c
                ),
            }
        }
    }
    let l = bit_string.len();
    bit_string[l - 1] <<= 64 - idx % 64;

    let (_, packet) = parse_packet(&bit_string, 0, false);

    Ok((sum_versions(&packet), calculate(&packet)))
}

fn sum_versions(packet: &Packet) -> usize {
    let mut num = packet.p_ver;
    match &packet.p_data {
        PacketData::Literal(_) => {}
        PacketData::Sum(sub_packets)
        | PacketData::Product(sub_packets)
        | PacketData::Minimum(sub_packets)
        | PacketData::Maximum(sub_packets)
        | PacketData::GreaterThan(sub_packets)
        | PacketData::LessThan(sub_packets)
        | PacketData::EqualTo(sub_packets) => {
            for p in sub_packets {
                num += sum_versions(p);
            }
        }
    }
    num
}

fn calculate(packet: &Packet) -> usize {
    match &packet.p_data {
        PacketData::Literal(n) => *n,
        PacketData::Sum(sub_packets) => sub_packets.iter().map(calculate).sum(),
        PacketData::Product(sub_packets) => sub_packets.iter().map(calculate).product(),
        PacketData::Minimum(sub_packets) => sub_packets.iter().map(calculate).min().unwrap(),
        PacketData::Maximum(sub_packets) => sub_packets.iter().map(calculate).max().unwrap(),
        PacketData::GreaterThan(sub_packets) => {
            if calculate(&sub_packets[0]) > calculate(&sub_packets[1]) {
                1
            } else {
                0
            }
        }
        PacketData::LessThan(sub_packets) => {
            if calculate(&sub_packets[0]) < calculate(&sub_packets[1]) {
                1
            } else {
                0
            }
        }
        PacketData::EqualTo(sub_packets) => {
            if calculate(&sub_packets[0]) == calculate(&sub_packets[1]) {
                1
            } else {
                0
            }
        }
    }
}
