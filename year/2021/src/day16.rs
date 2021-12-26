use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::str::{self, FromStr};

use hex;

/**
 * Multiple of 4
 *
 * 0-3: version
 * 4-7: type --- 4: literal value (single binary number); the binary number is padded (until multiple of 4)---
 *
 * type id != 4 is a operator
 *
 * Operator packet has a length id: 0 or 1.
 * 0: next 15 bits length of all subpackets
 * 1: next 11 bits is the number of subpackets
 *
 */

// Returns the next X bits, where X == next
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

// Either finish the process (true) or increment 'position' to point at the next packet
fn padding(position: &mut usize, input: &Vec<u8>, current_packet_size: usize) -> bool {
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

fn puzzle_1(input: &Vec<u8>) {
    let mut position: usize = 0;
    let mut end = false;
    let mut version_count = 0;
    while !end {
        let version_id = next_bits(&mut position, input, 3) as u8;
        version_count += version_id;
        let type_id = next_bits(&mut position, input, 3) as u8;
        match type_id {
            4 => {
                let mut packet_size = 6;
                let mut position_ = position;
                let literal = extract_literal(&mut position, input);
                packet_size += position - position_;
                end = padding(&mut position, input, packet_size);
            }
            _ => {
                let length_type = next_bits(&mut position, input, 1) as u8;
                if length_type == 0 {
                    let subpackets_length = next_bits(&mut position, input, 15) as u8;
                } else if length_type == 1 {
                    let subpackets_number = next_bits(&mut position, input, 11) as u8;
                }
            }
        }
    }
    
    println!("Puzzle 1: {:?}", version_count);
}

fn puzzle_2() {
    println!("Puzzle 2: {:?}", 1);
}

pub fn run() {
    let input: String =
        fs::read_to_string("./year/2021/inputs/day16.input").expect("Error reading file.");
    let hex_u8 = hex::decode(input).expect("Hex decoding error.");
    puzzle_1(&hex_u8);
}
