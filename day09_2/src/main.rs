use std::fs::read_to_string;

fn main() {
    let (files, mut free_spaces) = parse_input(read_to_string("input.txt").unwrap());

    let mut checksum = 0;
    'file_loop: for (file_number, file_addr, file_size) in files.into_iter().rev() {
        for i in 0..free_spaces.len() {
            let (space_addr, space_size) = free_spaces.get_mut(i).unwrap();
            if *space_addr > file_addr {
                break;
            } else if file_size == *space_size {
                checksum += ((*space_addr + file_size) * (*space_addr + file_size - 1)
                    - *space_addr * (*space_addr - 1))
                    / 2
                    * file_number;
                free_spaces.remove(i);
                continue 'file_loop;
            } else if file_size < *space_size {
                checksum += ((*space_addr + file_size) * (*space_addr + file_size - 1)
                    - *space_addr * (*space_addr - 1))
                    / 2
                    * file_number;
                *space_addr += file_size;
                *space_size -= file_size;
                continue 'file_loop;
            }
        }
        checksum += ((file_addr + file_size) * (file_addr + file_size - 1)
            - file_addr * (file_addr - 1))
            / 2
            * file_number;
    }

    println!("{checksum}");
}

fn parse_input(input: String) -> (Vec<(i64, i64, i64)>, Vec<(i64, i64)>) {
    let mut files = Vec::new();
    let mut free_spaces = Vec::new();
    let mut file_number = 0;
    let mut mem_addr = 0;
    let mut file_next = true;
    for c in input.trim().chars() {
        let size = (u8::try_from(c).unwrap() - b'0').into();
        if file_next {
            files.push((file_number, mem_addr, size));
            file_number += 1;
            mem_addr += size;
            file_next = false;
        } else {
            free_spaces.push((mem_addr, size));
            mem_addr += size;
            file_next = true;
        }
    }

    (files, free_spaces)
}
