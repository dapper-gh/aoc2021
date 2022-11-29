use aoc_runner_derive::{aoc_generator, aoc};

#[derive(Debug)]
struct Parsed<T> {
    value: T,
    consumed: usize
}
#[derive(Debug)]
pub struct Packet {
    version: u8,
    data: PacketType
}
#[derive(Debug)]
pub enum PacketType {
    Lit(u64),
    Sum(Vec<Packet>),
    Mul(Vec<Packet>),
    Min(Vec<Packet>),
    Max(Vec<Packet>),
    Gt(Box<Packet>, Box<Packet>),
    Lt(Box<Packet>, Box<Packet>),
    Eq(Box<Packet>, Box<Packet>)
}

fn parser(input: &str) -> Parsed<Packet> {
    fn parse_operator(input: &str) -> Parsed<Vec<Packet>> {
        match input.bytes().next().unwrap() {
            b'0' => {
                let length = usize::from_str_radix(input.get(1..16).unwrap(), 2).unwrap();

                let mut consumed = 0;
                let mut packets = vec![];
                while consumed < length {
                    let parsed = parser(input.get((16 + consumed)..).unwrap());
                    consumed += parsed.consumed;
                    packets.push(parsed.value);
                }

                Parsed {
                    value: packets,
                    consumed: consumed + 16
                }
            },

            b'1' => {
                let length = usize::from_str_radix(input.get(1..12).unwrap(), 2).unwrap();

                let mut packets = vec![];
                let mut consumed = 12;
                while packets.len() < length {
                    let parsed = parser(input.get(consumed..).unwrap());
                    consumed += parsed.consumed;
                    packets.push(parsed.value);
                }

                Parsed {
                    value: packets,
                    consumed
                }
            },

            _ => panic!()
        }
    }

    let version = u8::from_str_radix(input.get(0..3).unwrap(), 2).unwrap();
    let tid = u8::from_str_radix(input.get(3..6).unwrap(), 2).unwrap();

    match tid {
        4 => {
            let mut i = 6;
            let mut num = 0;
            while input.bytes().nth(i).unwrap() == b'1' {
                num *= 16;
                num += u64::from_str_radix(input.get((i + 1)..(i + 5)).unwrap(), 2).unwrap();
                i += 5;
            }
            num *= 16;
            num += u64::from_str_radix(input.get((i + 1)..(i + 5)).unwrap(), 2).unwrap();
            i += 5;

            Parsed {
                value: Packet {
                    version,
                    data: PacketType::Lit(num as u64),
                },
                consumed: i
            }
        },

        0 => {
            let parsed = parse_operator(input.get(6..).unwrap());

            Parsed {
                value: Packet {
                    version,
                    data: PacketType::Sum(parsed.value)
                },
                consumed: parsed.consumed + 6
            }
        },

        1 => {
            let parsed = parse_operator(input.get(6..).unwrap());

            Parsed {
                value: Packet {
                    version,
                    data: PacketType::Mul(parsed.value)
                },
                consumed: parsed.consumed + 6
            }
        },

        2 => {
            let parsed = parse_operator(input.get(6..).unwrap());

            Parsed {
                value: Packet {
                    version,
                    data: PacketType::Min(parsed.value)
                },
                consumed: parsed.consumed + 6
            }
        },

        3 => {
            let parsed = parse_operator(input.get(6..).unwrap());

            Parsed {
                value: Packet {
                    version,
                    data: PacketType::Max(parsed.value)
                },
                consumed: parsed.consumed + 6
            }
        },

        5 => {
            let mut parsed = parse_operator(input.get(6..).unwrap());

            let last = parsed.value.pop().unwrap();
            Parsed {
                value: Packet {
                    version,
                    data: PacketType::Gt(Box::new(parsed.value.pop().unwrap()), Box::new(last))
                },
                consumed: parsed.consumed + 6
            }
        },

        6 => {
            let mut parsed = parse_operator(input.get(6..).unwrap());

            let last = parsed.value.pop().unwrap();
            Parsed {
                value: Packet {
                    version,
                    data: PacketType::Lt(Box::new(parsed.value.pop().unwrap()), Box::new(last))
                },
                consumed: parsed.consumed + 6
            }
        },

        7 => {
            let mut parsed = parse_operator(input.get(6..).unwrap());

            let last = parsed.value.pop().unwrap();
            Parsed {
                value: Packet {
                    version,
                    data: PacketType::Eq(Box::new(parsed.value.pop().unwrap()), Box::new(last))
                },
                consumed: parsed.consumed + 6
            }
        }

        _ => panic!()
    }
}

#[aoc_generator(day16)]
pub fn generator(input: &str) -> Packet {
    let mut s = String::new();

    for b in input.bytes() {
        s.push_str(match b {
            b'0' => "0000",
            b'1' => "0001",
            b'2' => "0010",
            b'3' => "0011",
            b'4' => "0100",
            b'5' => "0101",
            b'6' => "0110",
            b'7' => "0111",
            b'8' => "1000",
            b'9' => "1001",
            b'A' => "1010",
            b'B' => "1011",
            b'C' => "1100",
            b'D' => "1101",
            b'E' => "1110",
            b'F' => "1111",

            _ => panic!()
        });
    }

    parser(&s).value
}

#[aoc(day16, part1)]
pub fn part1(input: &Packet) -> u64 {
    fn recurse(packet: &Packet) -> u64 {
        (packet.version as u64) + (match &packet.data {
            PacketType::Lit(_) => 0,
            PacketType::Sum(packets) => packets.iter().map(recurse).sum(),
            PacketType::Mul(packets) => packets.iter().map(recurse).sum(),
            PacketType::Min(packets) => packets.iter().map(recurse).sum(),
            PacketType::Max(packets) => packets.iter().map(recurse).sum(),
            PacketType::Gt(p1, p2) => recurse(&*p1) + recurse(&*p2),
            PacketType::Lt(p1, p2) => recurse(&*p1) + recurse(&*p2),
            PacketType::Eq(p1, p2) => recurse(&*p1) + recurse(&*p2)
        })
    }

    recurse(input)
}

#[aoc(day16, part2)]
pub fn part2(input: &Packet) -> u64 {
    fn recurse(packet: &Packet) -> u64 {
        match &packet.data {
            PacketType::Lit(n) => *n,
            PacketType::Sum(packets) => packets.iter().map(recurse).sum(),
            PacketType::Mul(packets) => packets.iter().map(recurse).product(),
            PacketType::Min(packets) => packets.iter().map(recurse).min().unwrap(),
            PacketType::Max(packets) => packets.iter().map(recurse).max().unwrap(),
            PacketType::Gt(p1, p2) => if recurse(&*p1) > recurse(&*p2) {
                1
            } else {
                0
            },
            PacketType::Lt(p1, p2) => if recurse(&*p1) < recurse(&*p2) {
                1
            } else {
                0
            },
            PacketType::Eq(p1, p2) => if recurse(&*p1) == recurse(&*p2) {
                1
            } else {
                0
            }
        }
    }

    recurse(input)
}