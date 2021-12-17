use std::{str::FromStr, num::ParseIntError};

#[derive(Debug)]
struct Packet {
    pub version: u8,
    pub packet_id: u8,
    pub data: PacketData,
    pub length: i64,
}

#[derive(Debug)]
enum PacketData {
    Literal(i64),
    Operator(i64, i64, Vec<Packet>),
}

fn read_int(bytes: &[u8], index: &mut i64, len: i64) -> i64 {
    let mut ret: i64 = 0;
    for _ in 0..len {
        let byte_index = *index / 8;
        let bit_index = *index % 8;
        let shift_width = 8 - bit_index - 1;
        let bitmask = 1 << shift_width;

        ret <<= 1;
        ret += ((bytes[byte_index as usize] & bitmask) >> shift_width) as i64;

        *index += 1;
    }

    ret
}

impl Packet {
    fn from_bytes(bytes: &[u8], offset: i64) -> Result<Self, ParseIntError> {
        let mut length = offset;
        let version = read_int(&bytes, &mut length, 3) as u8;
        let packet_id = read_int(&bytes, &mut length, 3) as u8;

        let data = match packet_id {
            4 => {
                let mut literal: i64 = 0;
                let mut last = false;

                while !last {
                    let last_val = read_int(&bytes, &mut length, 1);
                    if last_val == 0 {
                        last = true;
                    }

                    let val = read_int(&bytes, &mut length, 4);

                    literal <<= 4;
                    literal += val;

                    if last {
                        break;
                    }
                }

                PacketData::Literal(literal)
            },
            _ => {
                let mut sub_packets: Vec<Packet> = Vec::new();
                let length_type_id = read_int(&bytes, &mut length, 1);
                if length_type_id == 0 {
                    let total_length = read_int(&bytes, &mut length, 15);
                    let mut data_length = 0;

                    while data_length < total_length {
                        let packet = Self::from_bytes(&bytes, length).unwrap();
                        data_length += packet.length;
                        length += packet.length;
                        sub_packets.push(packet);
                    }

                    PacketData::Operator(length_type_id, total_length, sub_packets)
                } else {
                    let num_subpackets = read_int(&bytes, &mut length, 11);
                    for _ in 0..num_subpackets {
                        let packet = Self::from_bytes(&bytes, length).unwrap();
                        length += packet.length;
                        sub_packets.push(packet);
                    }

                    PacketData::Operator(length_type_id, num_subpackets, sub_packets)
                }
            },
        };

        Ok(Packet {
            version,
            packet_id,
            data,
            length: length - offset,
        })
    }
}

impl FromStr for Packet {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = Vec::new();
        for i in (0..s.len()-1).step_by(2) {
            let b = u8::from_str_radix(&s[i..i+2], 16).unwrap();
            bytes.push(b);
        }

        Self::from_bytes(&bytes, 0)
    }
}

fn sum_versions(packet: &Packet) -> i64 {
    match &packet.data {
        PacketData::Literal(_) => {
            packet.version as i64
        },
        PacketData::Operator(_, _, sub_packets) => {
            let mut sum = 0;
            for p in sub_packets {
                sum += sum_versions(&p);
            }
            packet.version as i64 + sum
        },
    }
}

pub fn a() -> i64 {
    let f = std::fs::read_to_string("input/2021/day16.txt").unwrap();
    let packet = f.parse::<Packet>().unwrap();
    let sum = sum_versions(&packet);
    sum
}

fn calc(packet: &Packet) -> i64 {
    match &packet.data {
        PacketData::Literal(value) => {
            *value
        },
        PacketData::Operator(_, _, sub_packets) => {
            match packet.packet_id {
                0 => {
                    let mut sum = 0;
                    for p in sub_packets {
                        sum += calc(&p);
                    }
                    sum
                },
                1 => {
                    let mut prod = 1;
                    for p in sub_packets {
                        prod *= calc(&p);
                    }
                    prod
                },
                2 => {
                    let mut min = i64::MAX;
                    for p in sub_packets {
                        let v = calc(&p);
                        if v < min {
                            min = v;
                        }
                    }
                    min
                },
                3 => {
                    let mut max = 0;
                    for p in sub_packets {
                        let v = calc(&p);
                        if v > max {
                            max = v;
                        }
                    }
                    max
                },
                5 => {
                    let v1 = calc(&sub_packets[0]);
                    let v2 = calc(&sub_packets[1]);
                    if v1 > v2 {
                        1
                    } else {
                        0
                    }
                },
                6 => {
                    let v1 = calc(&sub_packets[0]);
                    let v2 = calc(&sub_packets[1]);
                    if v1 < v2 {
                        1
                    } else {
                        0
                    }
                },
                7 => {
                    let v1 = calc(&sub_packets[0]);
                    let v2 = calc(&sub_packets[1]);
                    if v1 == v2 {
                        1
                    } else {
                        0
                    }
                },
                _ => 0,
            }
        },
    }
}

pub fn b() -> i64 {
    let f = std::fs::read_to_string("input/2021/day16.txt").unwrap();
    let packet = f.parse::<Packet>().unwrap();
    let val = calc(&packet);
    val
}