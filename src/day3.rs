use crate::reading;

pub fn day3() {
    let s = reading::get_content_from_path("./input/day3.txt");
    let lines = s.lines();

    let mut sum = 0;

    for li in lines {

        let mid = li.chars().count()/2;
        let ruck = li.clone().to_string();

        let comp1 = &ruck[..mid];
        let comp2 = &ruck[mid..];
        
        if comp1.len() != comp2.len() {
            panic!("invalid compartment sizes
            comp1: {}
            comp2: {}
            ", comp1.len(), comp2.len());
        }

        let shared = check_for_match(comp1, comp2);
        
        sum += get_priority_of_char(shared);

    }

    println!("Part 1 Answer: {}", sum);
}

fn check_for_match(str1: &str, str2: &str) -> char {
    for c1 in str1.chars() {
        for c2 in str2.chars() {
            if c1 == c2 {
                return c1;
            }
        }
    }
    '\n'
}

fn get_priority_of_char(c: char) -> i32 {
    if c.is_lowercase() {
        return (c as i32) - 96;   
    }
    if c.is_uppercase() {
        return (c as i32) - 38;
    }
    0
}