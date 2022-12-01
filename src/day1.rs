use std::fs::read;

use cli_toolbox::*;
use crate::reading;

pub fn day1() {
    let s = reading::get_content_from_path("./input/day1.txt");
    let lines = s.lines();

    let mut elf_values: Vec<i32> = Vec::new();
    elf_values.push(0);

    for li in lines {

        if li.len() == 0 {
            elf_values.push(0);
        } else {
            let read_val: i32 = li.parse().unwrap();
            let val = elf_values.pop().unwrap() + read_val;
            elf_values.push(val);
        }
    }

    elf_values.sort();

    println!("Part 1 Answer: {}", elf_values.last().unwrap());

    let mut top_3_sum = elf_values.get(elf_values.len()-1).unwrap().clone();
    top_3_sum += elf_values.get(elf_values.len()-2).unwrap().clone();
    top_3_sum += elf_values.get(elf_values.len()-3).unwrap().clone();

    println!("Part 2 Answer: {}", top_3_sum);
}