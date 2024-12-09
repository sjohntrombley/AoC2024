use std::collections::VecDeque;
use std::fs::read_to_string;

enum DiskArea {
    File(i64, i64),
    Empty(i64),
}
use DiskArea::*;

fn main() {
    let mut input = parse_input(read_to_string("input.txt").unwrap());

    let mut mem_addr = 0;
    let mut checksum = 0;
    while !input.is_empty() {
        let (file_number, file_size) = match input.pop_front().unwrap() {
            File(file_number, file_size) => (file_number, file_size),
            _ => panic!("Expected File but found Empty."),
        };
        let next_mem_addr = mem_addr + file_size;
        checksum += (next_mem_addr * (next_mem_addr - 1) - mem_addr * (mem_addr - 1)) / 2 * file_number;
        mem_addr = next_mem_addr;

        let mut blank_size = match input.pop_front() {
            None => break,
            Some(Empty(size)) => size,
            _ => panic!("Expected Empty but found File."),
        };
        while blank_size > 0 {
            let (file_number, file_size) = match input.back_mut() {
                Some(File(number, size)) => (number, size),
                _ => panic!("input ends with Empty"),
            };
            
            if blank_size < *file_size {
                let next_mem_addr = mem_addr + blank_size;
                checksum += (next_mem_addr * (next_mem_addr - 1) - mem_addr * (mem_addr - 1)) / 2 * *file_number;
                mem_addr = next_mem_addr;
                *file_size -= blank_size;
                blank_size = 0;
            } else {
                let next_mem_addr = mem_addr + *file_size;
                checksum += (next_mem_addr * (next_mem_addr - 1) - mem_addr * (mem_addr - 1)) / 2 * *file_number;
                mem_addr = next_mem_addr;
                blank_size -= *file_size;
                input.pop_back();
                match input.pop_back() {
                    None => break,
                    Some(File(..)) => panic!("input contains two files in a row"),
                    _ => (),
                }
            }
        }
    }

    println!("{checksum}");
}

fn parse_input(input: String) -> VecDeque<DiskArea> {
    let mut disk_areas = VecDeque::new();
    let mut file_number = 0;
    let mut file_next = true;
    for c in input.trim().chars() {
        let c = (u8::try_from(c).unwrap() - b'0').into();
        if file_next {
            disk_areas.push_back(File(file_number, c));
            file_number += 1;
            file_next = false;
        } else {
            disk_areas.push_back(Empty(c));
            file_next = true;
        }
    }

    disk_areas
}
