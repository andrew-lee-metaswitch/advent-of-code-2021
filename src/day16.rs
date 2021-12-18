use crate::util;
use std::fmt;
use std::{collections::HashMap, fmt::Debug};

lazy_static! {
    static ref HEXBYTES: HashMap<char, Vec<char>> = hashmap! {
        '0' => vec!['0','0','0','0'],
        '1' => vec!['0','0','0','1'],
        '2' => vec!['0','0','1','0'],
        '3' => vec!['0','0','1','1'],
        '4' => vec!['0','1','0','0'],
        '5' => vec!['0','1','0','1'],
        '6' => vec!['0','1','1','0'],
        '7' => vec!['0','1','1','1'],
        '8' => vec!['1','0','0','0'],
        '9' => vec!['1','0','0','1'],
        'A' => vec!['1','0','1','0'],
        'B' => vec!['1','0','1','1'],
        'C' => vec!['1','1','0','0'],
        'D' => vec!['1','1','0','1'],
        'E' => vec!['1','1','1','0'],
        'F' => vec!['1','1','1','1'],
    };
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct BitsSequence {
    binary_string: Vec<char>,
    version: u8,
    packet_id: u8,
    fully_processed: bool,
    binary_number: Option<u64>,
    number_of_sub_packets: Option<u32>,
    length_of_sub_packets: Option<u32>,
    unprocessed_bytes: Option<Vec<char>>,
    substrates: Option<Vec<BitsSequence>>,
    kids_are_processed: bool,
}

impl fmt::Display for BitsSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        if self.binary_number != None {
            write!(
                f,
                "Version {}, Packet_ID: {}, Length: {}, BN: {}",
                self.version,
                self.packet_id,
                self.binary_string.len(),
                self.binary_number.unwrap_or(0),
            )
        } else if self.number_of_sub_packets != None {
            write!(
                f,
                "Version {}, Packet_ID: {}, Length: {}, #subparts: {} unproceed bytes: {} target#subparts {}",
                self.version,
                self.packet_id,
                self.binary_string.len(),
                &self.substrates.as_ref().unwrap_or(&vec![]).len(),

               &self.unprocessed_bytes.as_ref().unwrap_or(&vec![]).len(),
               self.number_of_sub_packets.unwrap(),

            )
        } else {
            write!(
                    f,
                    "Version {}, Packet_ID: {}, Length: {}, #subparts: {} unproceed bytes: {} target sumsubparts {}",
                    self.version,
                    self.packet_id,
                    self.binary_string.len(),
                    &self.substrates.as_ref().unwrap_or(&vec![]).len(),
                    &self.unprocessed_bytes.as_ref().unwrap_or(&vec![]).len(),
                    self.length_of_sub_packets.unwrap(),

                )
        }
    }
}

impl BitsSequence {
    fn from_vec(input: Vec<char>) -> Self {
        // Every packet begins with a standard header: the first three bits encode the packet version
        let version: u8 = u8::from_str_radix(
            &input[0..3].iter().map(|c| *c as char).collect::<String>(),
            2,
        )
        .unwrap();
        // the next three bits encode the packet type ID
        let packet_id: u8 = u8::from_str_radix(
            &input[3..6].iter().map(|c| *c as char).collect::<String>(),
            2,
        )
        .unwrap();

        let mut a = match packet_id {
            4 => {
                // Remaining chars in packet represent a literal value:
                // So it encodes a single binary number.

                // To do this, the binary number is padded with leading zeroes until its length is a multiple of four bits, and then it is broken into groups of four bits.
                // Each group is prefixed by a 1 bit except the last group, which is prefixed by a 0 bit.
                // These groups of five bits immediately follow the packet header
                let mut binary_number_as_vec: Vec<char> = vec![];
                let mut last_quad = false;

                for (pos, c) in input[6..input.len()].iter().enumerate() {
                    if pos % 5 != 0 {
                        binary_number_as_vec.push(*c as char);
                    } else if c == &'0' && !last_quad {
                        last_quad = true;
                    } else if last_quad {
                        break;
                    }
                }
                let bit_sequence_length = 6 + (5 * (binary_number_as_vec.len()) / 4);
                let binary_number =
                    u64::from_str_radix(&binary_number_as_vec.into_iter().collect::<String>(), 2)
                        .unwrap();
                BitsSequence {
                    binary_string: input[0..bit_sequence_length].to_vec(),
                    version,
                    packet_id,
                    fully_processed: true,
                    number_of_sub_packets: None,
                    length_of_sub_packets: None,
                    binary_number: Some(binary_number),
                    substrates: None,
                    unprocessed_bytes: None,
                    kids_are_processed: true,
                }
            }
            _ => {
                // Every other type of packet (any packet with a type ID other than 4)
                // represent an operator that performs some calculation on one or more sub-packets contained within.
                let packet_length_type_id = input[6];
                match packet_length_type_id {
                    '0' => {
                        // If the length type ID is '0', then the next 15 bits are a number that represents
                        // the total length in bits of the sub-packets contained by this packet.
                        let total_length_of_sub_packets = u32::from_str_radix(
                            &input[7..22].iter().map(|c| *c as char).collect::<String>(),
                            2,
                        )
                        .unwrap();

                        BitsSequence {
                            binary_string: input.clone(),
                            version,
                            packet_id,
                            fully_processed: false,
                            binary_number: None,
                            number_of_sub_packets: None,
                            length_of_sub_packets: Some(total_length_of_sub_packets),
                            unprocessed_bytes: Some(
                                input[22..input.len()].iter().copied().collect(),
                            ),
                            substrates: None,
                            kids_are_processed: false,
                        }
                    }
                    '1' => {
                        // If the length type ID is '1', then the next 11 bits are a number that represents
                        // the number of sub-packets immediately contained by this packet.
                        let number_of_sub_packets = u32::from_str_radix(
                            &input[7..18].iter().map(|c| *c as char).collect::<String>(),
                            2,
                        )
                        .unwrap();
                        BitsSequence {
                            binary_string: input.clone(),
                            version,
                            packet_id,
                            fully_processed: false,
                            binary_number: None,
                            number_of_sub_packets: Some(number_of_sub_packets),
                            length_of_sub_packets: None,
                            unprocessed_bytes: Some(
                                input[18..input.len()].iter().copied().collect(),
                            ),
                            substrates: None,
                            kids_are_processed: false,
                        }
                    }
                    _ => panic!("Whoops-si-daisy"),
                }
            }
        };

        while !a.fully_processed {
            // a is not fully OK, so lets process all those unprocessed bytes
            println!("a thing is not fully processed {}", a);
            if a.substrates != None {
                println!("Here are tings in a");
                for b in a.substrates.as_ref().unwrap() {
                    println!("    {}", b);
                }
                println!("Here endeth tings in a");
            }

            // Either, we should keep using execess unprocessed_bytes_vec to make more kids, but first check we don't already have too many kids,

            if let Some(substructs) = &mut a.substrates {
                let length = substructs
                    .iter()
                    .map(|s| s.binary_string.len())
                    .sum::<usize>();
                let total = substructs.len();
                println!("len is {}, totoal is {}", length, total);
                if substructs.iter().all(|s| s.fully_processed) {
                    if total == a.number_of_sub_packets.unwrap_or(0) as usize
                        || length == a.length_of_sub_packets.unwrap_or(0) as usize
                    {
                        // a is complete
                        println!(
                            "Hitting this - need to get rid of some of those unproceesed bytes"
                        );
                        a.kids_are_processed = true;
                        return a;
                    } else if total > a.number_of_sub_packets.unwrap_or(200000) as usize
                        || length > a.length_of_sub_packets.unwrap_or(100000) as usize
                    {
                        panic!("😢");
                    }
                }

                // Okay we have substrates
                for s in substructs.iter_mut() {
                    if s.kids_are_processed {
                        if let Some(s_unprocessed_bytes) = &mut s.unprocessed_bytes {
                            s.binary_string = s.binary_string
                                [0..s.binary_string.len() - s_unprocessed_bytes.len()]
                                .to_vec();
                            if s_unprocessed_bytes.len() < 6 {
                                s_unprocessed_bytes.clear();
                            }

                            a.unprocessed_bytes = match a.unprocessed_bytes {
                                Some(mut a_unprocessed_bytes) => Some({
                                    a_unprocessed_bytes.extend(s_unprocessed_bytes.clone());
                                    a_unprocessed_bytes
                                }),
                                None => Some(s_unprocessed_bytes.to_vec()),
                            };
                        }

                        s.unprocessed_bytes = None;
                        s.fully_processed = true;
                    }
                }
            }

            if let Some(unprocessed_bytes_vec) = &mut a.unprocessed_bytes {
                if unprocessed_bytes_vec.len() < 6 {
                    // this is trailling whitescape
                    a.unprocessed_bytes = None;
                } else {
                    let b = BitsSequence::from_vec(unprocessed_bytes_vec.to_vec());

                    // We've parsed b, lets removed those chars from unprocessed_bytes
                    if b.binary_string.len() == unprocessed_bytes_vec.len() {
                        // We've got no more unprocessed bytes {
                        a.unprocessed_bytes = None;
                    } else {
                        a.unprocessed_bytes = Some(
                            unprocessed_bytes_vec
                                [b.binary_string.len()..unprocessed_bytes_vec.len()]
                                .to_vec(),
                        );
                    }
                    a.substrates = match a.substrates {
                        Some(mut vec) => Some({
                            vec.push(b);
                            vec
                        }),
                        None => Some([b].to_vec()),
                    };
                }
            }

            if a.unprocessed_bytes == None {
                a.fully_processed = a.all_subpackets_processed();
            }
        }
        println!("New fully processed thing : {}", a);
        println!("Here are things inside the new thing:");
        if a.substrates != None {
            for b in a.substrates.as_ref().unwrap() {
                println!("    {}", b);
                println!("    Here endeth tings in a");
            }
        }
        a
    }

    fn all_subpackets_processed(&self) -> bool {
        //println!("Deciding if {} is fully procseed", self);
        if self.fully_processed {
            true
        } else if let Some(substrates_vec) = &self.substrates {
            let length = substrates_vec
                .iter()
                .map(|s| s.binary_string.len())
                .sum::<usize>();
            let total = substrates_vec.len();
            substrates_vec.iter().all(|s| s.fully_processed)
                && (total == self.number_of_sub_packets.unwrap_or(0) as usize
                    || length == self.length_of_sub_packets.unwrap_or(0) as usize)
        } else {
            // No substrates and not fully_processed
            false
        }
    }

    fn total_sum(&self) -> u32 {
        let mut ans: u32 = 0;

        ans += self.version as u32;
        if let Some(substrate) = &self.substrates {
            ans += substrate.iter().map(|s| s.total_sum()).sum::<u32>()
        }
        ans
    }

    fn expresion(&self) -> u64 {
        if let Some(no) = self.binary_number {
            println!(
                "Packet {} has expression value {} and type {}",
                self, no, self.packet_id
            );
            return no;
        }
        let a = match self.packet_id {
            0 => self
                .substrates
                .as_ref()
                .unwrap()
                .iter()
                .map(|v| v.expresion())
                .sum::<u64>(),
            1 => self
                .substrates
                .as_ref()
                .unwrap()
                .iter()
                .map(|v| v.expresion())
                .product::<u64>(),
            2 => self
                .substrates
                .as_ref()
                .unwrap()
                .iter()
                .map(|v| v.expresion())
                .min()
                .unwrap(),
            3 => self
                .substrates
                .as_ref()
                .unwrap()
                .iter()
                .map(|v| v.expresion())
                .max()
                .unwrap(),
            5 => {
                let first_sp_value = self.substrates.as_ref().unwrap()[0].expresion();
                let second_sp_value = self.substrates.as_ref().unwrap()[1].expresion();
                if first_sp_value > second_sp_value {
                    1
                } else {
                    0
                }
            }
            6 => {
                let first_sp_value = self.substrates.as_ref().unwrap()[0].expresion();
                let second_sp_value = self.substrates.as_ref().unwrap()[1].expresion();
                if first_sp_value < second_sp_value {
                    1
                } else {
                    0
                }
            }
            7 => {
                let first_sp_value = self.substrates.as_ref().unwrap()[0].expresion();
                let second_sp_value = self.substrates.as_ref().unwrap()[1].expresion();
                if first_sp_value == second_sp_value {
                    1
                } else {
                    0
                }
            }
            _ => panic!("Whoops-idaisy"),
        };
        println!(
            "Packet {} has expression value {} and type {}",
            self, a, self.packet_id
        );
        a
    }
}

fn part_one(bit_sequence: &BitsSequence) {
    println!("The answer to part 1 is {}", bit_sequence.total_sum());
}

fn part_two(bit_sequence: &BitsSequence) {
    println!("The answer to part 2 is {}", bit_sequence.expresion());
}

pub(crate) fn day16() {
    // Load inputs from input directory
    let bit_sequence: BitsSequence = BitsSequence::from_vec({
        let hex_str = util::load_inputs("16".to_string())
            .get(0)
            .unwrap()
            .chars()
            .map(|v| HEXBYTES.get(&v).unwrap())
            .flatten()
            .cloned()
            .collect::<Vec<char>>();

        // The hexadecimal representation of this packet might encode a few extra 0 bits at the end;
        // while hex_str.last().unwrap() == &'0' {
        //     hex_str.pop();
        // }
        hex_str
    });

    println!("{}", bit_sequence);

    // while !bit_sequence.fully_processed {
    //     println!("Processing main packet");
    //     bit_sequence.process_me();
    // }

    part_one(&bit_sequence);
    part_two(&bit_sequence);
}
