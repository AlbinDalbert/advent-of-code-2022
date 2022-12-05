use std::str::Lines;
use crate::reading;

pub fn day5() {
    let s = reading::get_content_from_path("./input/day5.txt");
    let mut lines = s.lines();
    let mut lines2 = lines.clone();

    // setup vector of stacks and setup vector of moves for part 1
    let mut stacks = init_stacks(&mut lines);
    let moves: Vec<[i32; 3]> = get_moves(&mut lines);
    let mut res = "".to_string();
    
    //execute the moves for part 1
    execute_moves_9000(&mut stacks, moves);
    for mut s in stacks.clone() {
        res.push(s.pop().unwrap());
    }

    println!("Part 1 Answer: {}", res);

    // setup vector of stacks and setup vector of moves for part 2
    let mut stacks = init_stacks(&mut lines2);
    let moves: Vec<[i32; 3]> = get_moves(&mut lines2);
    let mut res = "".to_string();
    
    //execute the moves for part 2
    execute_moves_9001(&mut stacks, moves);
    for mut s in stacks.clone() {
        res.push(s.pop().unwrap());
    }

    println!("Part 2 Answer: {}", res);

}

fn execute_moves_9000(stacks: &mut Vec<Vec<char>>, moves: Vec<[i32; 3]>){

    for mv in moves {
        for _ in 0..mv[0] {

            let crt = stacks.get_mut((mv[1] as usize)-1).unwrap().pop().unwrap();
            stacks.get_mut((mv[2] as usize)-1).unwrap().push(crt);
            
        }
    }
}

fn execute_moves_9001(stacks: &mut Vec<Vec<char>>, moves: Vec<[i32; 3]>){

    for mv in moves {

        let mut temp_stack: Vec<char> = Vec::new();

        for _ in 0..mv[0] {
            let crt = stacks.get_mut((mv[1] as usize)-1).unwrap().pop().unwrap();
            temp_stack.push(crt);
        }

        for _ in 0..mv[0] {
            let crt = temp_stack.pop().unwrap();
            stacks.get_mut((mv[2] as usize)-1).unwrap().push(crt);
        }
    }
}

fn get_moves(lines: &mut Lines) -> Vec<[i32; 3]> {

    let mut moves: Vec<[i32; 3]> = Vec::new();

    for li in lines {
        
        let mut moves_array: [i32; 3] = [0; 3];
        let parts: Vec<&str> = li.split(' ').collect();

        moves_array[0] = parts.get(1).unwrap().parse::<i32>().unwrap();
        moves_array[1] = parts.get(3).unwrap().parse::<i32>().unwrap();
        moves_array[2] = parts.get(5).unwrap().parse::<i32>().unwrap();

        moves.push(moves_array);
    }

    moves
}

fn init_stacks(lines: &mut Lines) -> Vec<Vec<char>> {

    let mut trimmed: Vec<String> = Vec::new();
    let mut stacks: Vec<Vec<char>> = Vec::new();

    let lin = lines.clone().nth(0).unwrap().len();
    let num_of_stacks = (lin+1) / 4;

    println!("num of stacks {num_of_stacks}");

    for _ in 0..num_of_stacks {
        stacks.push(Vec::new());
    }

    println!("stacks length {}", stacks.len());

    for li in lines {
        if li.is_empty() {
            break;
        }
        trimmed.push(li.to_string());
    }

    for t in trimmed {

        let mut index = 1;
        let mut stack_id = 0;

        while index < t.len() && stack_id < num_of_stacks {
        
            let c = t.chars().nth(index).unwrap();
            if c.is_alphabetic() {
                stacks.get_mut(stack_id).unwrap().push(c);
            }
            
            index += 4;
            stack_id += 1;
        }

    }

    for stack in &mut stacks {
        stack.reverse();
    }

    stacks
}