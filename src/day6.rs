use std::collections::VecDeque;
use crate::reading;

pub fn day6() {
    let s = reading::get_content_from_path("./input/day6.txt");
    let s2 = s.clone();

    let mut q: VecDeque<char> = VecDeque::new();
    let mut package_count = 0;
    let mut message_count = 0;
    
    for c in s.chars() {
        if q.len() < 4 as usize {
            q.push_back(c);
            package_count += 1;
            continue;
        }

        if check_all_distinct(&q) {
            break;
        }
        q.pop_front();
        q.push_back(c);
        package_count += 1;

    }
    println!("Part 1 Answer: {package_count}");

    for c in s2.chars() {
        if q.len() < 14 as usize {
            q.push_back(c);
            message_count += 1;
            continue;
        }

        if check_all_distinct(&q) {
            break;
        }
        q.pop_front();
        q.push_back(c);
        message_count += 1;

    }

    println!("Part 2 Answer: {message_count}");
}

fn check_all_distinct(vec: &VecDeque<char>) -> bool {
    for i in 0..vec.len() {
        for j in i..vec.len() {
            if i == j {
                continue;
            }
            if vec[i] == vec[j] {
                return false;
            }
        }
    }
    true
}