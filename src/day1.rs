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

    let elf_iter = elf_values.clone();
    let max = elf_iter.iter().max().unwrap();

    println!("Part 1 Answer: {}", max);

    let mut top_3_sum = 0;
    let index = get_max_index(elf_values.clone());
    top_3_sum += elf_values.get(index).unwrap();
    elf_values.remove(index);

    let index = get_max_index(elf_values.clone());
    top_3_sum += elf_values.get(index).unwrap();
    elf_values.remove(index);

    let index = get_max_index(elf_values.clone());
    top_3_sum += elf_values.get(index).unwrap();
    elf_values.remove(index);

    println!("Part 2 Answer: {}", top_3_sum);
}

fn get_max_index(vec: Vec<i32>) -> usize {
    let mut max_value= vec.get(0).unwrap().clone();
    let mut max_index = 0;

    let mut index = 0;
    for i in vec.clone() {
        if i > max_value {
            max_index = index.clone();
            max_value = i;
        }
        index += 1;
    };
    max_index
}