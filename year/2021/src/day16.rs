use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::str::{self, FromStr};

use hex;

// Returns the next X bits from 'input', where X == 'next'
fn next_bits(position: &mut usize, input: &Vec<u8>, next: usize) -> u64 {
    let mut bits = 0u64;
    for i in 0..next {
        let index = *position + i;
        let mut byte: u8 = input[index / 8];
        byte = (byte >> (7 - index % 8)) & 1u8;
        bits = (bits << 1) | byte as u64;
    }
    *position = *position + next;
    bits
}

// Returns the complete literal from the packet
fn extract_literal(position: &mut usize, input: &Vec<u8>) -> u64 {
    let mut literal = 0u64;
    let mut last_group = 1;
    while last_group != 0 {
        last_group = next_bits(position, input, 1);
        let partial = next_bits(position, input, 4);
        literal = (literal << 4) | partial;
    }
    literal
}

// Either finish the process (return true) or just increment
// 'position' until the next packet
// Obs: paddings are applied at the end of packets, not sub-packets
fn solve_padding(position: &mut usize, input: &Vec<u8>, current_packet_size: usize) -> bool {
    let mut literal_packet_size = current_packet_size;
    while literal_packet_size % 4 != 0 {
        literal_packet_size += 1;
        *position = *position + 1;
    }
    while *position < input.len() * 8 {
        let next_4_bits = next_bits(position, input, 4);
        if next_4_bits != 0 {
            *position = *position - 4;
            break;
        }
    }
    if *position == input.len() * 8 {
        return true;
    }
    false
}

fn perform_calculation(literal_stack: &mut Vec<u64>, literal_size: usize, calculation_type: u8) {
    let mut result = 0u64;
    let mut ls_clone = literal_stack.clone();
    literal_stack.drain(literal_stack.len() - literal_size..literal_stack.len());
    let literal_slice = &ls_clone[ls_clone.len() - literal_size..ls_clone.len()];
    match calculation_type {
        0 => {
            result = literal_slice.iter().sum();
        }
        1 => {
            result = 1;
            for literal in literal_slice {
                result *= literal;
            }
        }
        2 => {
            result = *literal_slice.iter().min().unwrap();
        }
        3 => {
            result = *literal_slice.iter().max().unwrap();
        }
        5 => {
            if literal_slice[0] > literal_slice[1] {
                result = 1;
            }
        }
        6 => {
            if literal_slice[0] < literal_slice[1] {
                result = 1;
            }
        }
        7 => {
            if literal_slice[0] == literal_slice[1] {
                result = 1;
            }
        }
        _ => {
            println!("{}", "Type id field error")
        }
    }
    literal_stack.push(result);
}

#[derive(PartialEq)]
enum SubpacketLengthType {
    // Length of subpackets
    Length,
    // Number of subpackets
    Number,
}

struct Packet {
    packet_length_type: SubpacketLengthType,
    length_field: usize,
    accumulated_length: usize,
    // how many literals to include in the calculation
    literal_size: usize,
    // type id
    calculation_type: u8,
}

fn puzzle_1_and_2(input: &Vec<u8>) {
    let mut position: usize = 0;
    let mut version_count = 0u32;
    let mut packet_stack: Vec<Packet> = Vec::new();
    let mut literal_stack: Vec<u64> = Vec::new();
    let mut position_ = 0;
    let mut end = false;
    while !end {
        let version_id = next_bits(&mut position, input, 3) as u8;
        version_count += version_id as u32;
        let type_id = next_bits(&mut position, input, 3) as u8;
        let past_position = position;
        match type_id {
            // literal packet
            4 => {
                let literal = extract_literal(&mut position, input);
                literal_stack.push(literal);
                let current_packet_length = 6 + position - past_position;
                let mut new_length = 0usize;
                if let Some(packet) = packet_stack.pop() {
                    if packet.packet_length_type == SubpacketLengthType::Length {
                        new_length = packet.length_field - current_packet_length;
                    } else if packet.packet_length_type == SubpacketLengthType::Number {
                        new_length = packet.length_field - 1;
                    }
                    if new_length > 0 {
                        packet_stack.push(Packet {
                            packet_length_type: packet.packet_length_type,
                            length_field: new_length,
                            accumulated_length: packet.accumulated_length + current_packet_length,
                            literal_size: packet.literal_size + 1,
                            calculation_type: packet.calculation_type,
                        });
                    }
                    // update previous packets
                    // and perform calculations by updating 'literal_stack'
                    else if new_length == 0 {
                        perform_calculation(
                            &mut literal_stack,
                            packet.literal_size + 1,
                            packet.calculation_type,
                        );
                        let mut acc_length = packet.accumulated_length + current_packet_length;
                        while let Some(prev_packet) = packet_stack.pop() {
                            let mut updated_length = 0;
                            let updated_acc_length = prev_packet.accumulated_length + acc_length;
                            if prev_packet.packet_length_type == SubpacketLengthType::Length {
                                updated_length = prev_packet.length_field - acc_length;
                            } else if prev_packet.packet_length_type == SubpacketLengthType::Number
                            {
                                updated_length = prev_packet.length_field - 1;
                            }
                            if updated_length > 0 {
                                packet_stack.push(Packet {
                                    packet_length_type: prev_packet.packet_length_type,
                                    length_field: updated_length,
                                    accumulated_length: updated_acc_length,
                                    literal_size: prev_packet.literal_size + 1,
                                    calculation_type: prev_packet.calculation_type,
                                });
                                break;
                            } else if updated_length == 0 {
                                perform_calculation(
                                    &mut literal_stack,
                                    prev_packet.literal_size + 1,
                                    prev_packet.calculation_type,
                                );
                            }
                            acc_length = updated_acc_length;
                        }
                    }
                }
            }
            // operation packet
            _ => {
                let length_type = next_bits(&mut position, input, 1) as u8;
                let mut acc_length = 7;
                let mut subpackets_length = 0;
                let mut length_type_enum = SubpacketLengthType::Length;
                if length_type == 0 {
                    subpackets_length = next_bits(&mut position, input, 15) as usize;
                    acc_length += 15;
                } else if length_type == 1 {
                    subpackets_length = next_bits(&mut position, input, 11) as usize;
                    acc_length += 11;
                    length_type_enum = SubpacketLengthType::Number;
                }
                packet_stack.push(Packet {
                    packet_length_type: length_type_enum,
                    length_field: subpackets_length,
                    accumulated_length: acc_length,
                    literal_size: 0,
                    calculation_type: type_id,
                });
            }
        }
        if packet_stack.is_empty() {
            let packet_size = position - position_;
            end = solve_padding(&mut position, input, packet_size);
            position_ = position;
        }
    }
    println!("Puzzle 1: {:?}", version_count);
    println!("Puzzle 2: {:?}", literal_stack[0]);
}

pub fn run() {
    let input: String =
        fs::read_to_string("./year/2021/inputs/day16.input").expect("Error reading file.");
    let hex_u8 = hex::decode(input).expect("Hex decoding error.");
    puzzle_1_and_2(&hex_u8);
}
