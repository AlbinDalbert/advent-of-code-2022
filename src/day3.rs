use std::vec;

use crate::reading;


pub fn day3() {
    let s = reading::get_content_from_path("./input/day3.txt");
    let lines = s.lines();
    let lines2 = lines.clone();

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

    let mut sum2 = 0;
    let mut group: Vec<&str> = Vec::new();

    for li in lines2 {
        
        group.push(li);

        if group.len() > 2 {

            sum2 += get_priority_of_char(find_group_badge(group.clone()));
            group = Vec::new();
            
        }
        
    }
    println!("Part 2 Answer: {}", sum2);

}

fn find_group_badge(mut group: Vec<&str>) -> char {
    let str1 = group.pop().unwrap();
    let str2 = group.pop().unwrap();
    let str3 = group.pop().unwrap();
    for c1 in str1.chars() {
        for c2 in str2.chars() {
            for c3 in str3.chars() {
                if c1 == c2 && c1 == c3 {
                    return c1;
                }
            }
        }
    }
    '\n'
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