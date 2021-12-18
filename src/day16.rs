use crate::util;
use std::collections::HashMap;
use std::fmt;

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

#[derive(Clone, PartialEq, Eq)]
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

impl fmt::Debug for BitsSequence {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        self.print_me(0);
        Ok(())
    }
}

impl BitsSequence {
    fn print_me(&self, indent: usize) {
        println!("{: <0$} {1}", indent, self);
        if let Some(substrates) = &self.substrates {
            for s in substrates {
                s.print_me(indent + 4)
            }
        }
    }

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

        let mut bit_sequence = match packet_id {
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
                            binary_string: input[0..22].iter().copied().collect::<Vec<char>>(),

                            version,
                            packet_id,
                            fully_processed: false,
                            binary_number: None,
                            number_of_sub_packets: None,
                            length_of_sub_packets: Some(total_length_of_sub_packets),
                            unprocessed_bytes: Some(
                                input[22..input.len()].iter().copied().collect(),
                            ),
                            substrates: Some(vec![]),
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
                            binary_string: input[0..18].iter().copied().collect::<Vec<char>>(),
                            version,
                            packet_id,
                            fully_processed: false,
                            binary_number: None,
                            number_of_sub_packets: Some(number_of_sub_packets),
                            length_of_sub_packets: None,
                            unprocessed_bytes: Some(
                                input[18..input.len()].iter().copied().collect(),
                            ),
                            substrates: Some(vec![]),
                        }
                    }
                    _ => panic!("Whoops-si-daisy"),
                }
            }
        };
        bit_sequence.process_me();
        bit_sequence
    }

    fn new_length(&self) -> usize {
        let mut rc = self.binary_string.len();
        if let Some(s) = &self.substrates {
            for ss in s {
                rc += ss.new_length();
            }
        }
        rc
    }

    fn process_me(&mut self) {
        // This is complicated!

        // If we're fully proceed great
        if self.fully_processed {
            return;
        }

        // First decide if you are happy with current state, that is,
        //  #subparts ==  target#subparts or target#sum subparts sumsubparts

        if let Some(substructs) = &mut self.substrates {
            let mut length = substructs.iter().map(|s| s.new_length()).sum::<usize>();
            let mut total = substructs.len();
            while total != self.number_of_sub_packets.unwrap_or(20000) as usize
                && length != self.length_of_sub_packets.unwrap_or(20000) as usize
            {
                if let Some(unprocessed_bytes_vec) = &mut self.unprocessed_bytes {
                    if unprocessed_bytes_vec.len() < 6 {
                        // this is trailling whitescape
                        self.unprocessed_bytes = None;
                    } else {
                        let mut b = BitsSequence::from_vec(unprocessed_bytes_vec.to_vec());
                        b.process_me();

                        // b is now fully procssed
                        // So let's cut lots of bytes of unprocessed_bytes_vec
                        if b.new_length() == unprocessed_bytes_vec.len() {
                            // We've got no more unprocessed bytes {
                            self.unprocessed_bytes = None;
                        } else {
                            self.unprocessed_bytes = Some(
                                unprocessed_bytes_vec[b.new_length()..unprocessed_bytes_vec.len()]
                                    .to_vec(),
                            );
                        }
                        substructs.push(b);
                        length = substructs.iter().map(|s| s.new_length()).sum::<usize>();
                        total = substructs.len();
                    }
                }
            }
            self.fully_processed = true;
        }

        // We have substrates, but not enough, add another one
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
    let bit_sequence: BitsSequence = BitsSequence::from_vec(
        util::load_inputs("16".to_string())
            .get(0)
            .unwrap()
            .chars()
            .map(|v| HEXBYTES.get(&v).unwrap())
            .flatten()
            .cloned()
            .collect::<Vec<char>>(),
    );
    println!("{:?}", bit_sequence);

    part_one(&bit_sequence);
    part_two(&bit_sequence);
}
