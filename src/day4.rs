use std::vec;

use crate::reading;


pub fn day4() {
    let s = reading::get_content_from_path("./input/day4.txt");
    let lines = s.lines();
    let lines2 = lines.clone();

    for li in lines {
        let mut indexes: [i32; 3];

        let mut slices: Vec<&str> = Vec::new();


        let lit = li.clone();

        for c in lit.chars() {

            if slices.len() == 0 && c == '-' {

                // slices.push(&)

            }

            else if slices.len() == 1 && c == ',' {
 

            }

            else if slices.len() == 2 && c == '-' {

            }
            
            

        }

    }
}