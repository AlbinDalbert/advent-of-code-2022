use crate::reading;

#[derive(PartialEq)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

#[derive(PartialEq, Clone, Copy)]
enum Attack {
    Rock,
    Paper,
    Scissor,
}

pub fn day2() {

    let s = reading::get_content_from_path("./input/day2.txt");
    let lines = s.lines();
    let lines2 = lines.clone();

    let mut points: i32 = 0;
    

    for li in lines {
        let enemy = match li.chars().nth(0) {
            Some('A') => Attack::Rock,
            Some('B') => Attack::Paper,
            Some('C') => Attack::Scissor,
            _ => panic!("not A B or C"),
        };
        let me = match li.chars().nth(2) {
            Some('X') => {points += 1; Attack::Rock},
            Some('Y') => {points += 2; Attack::Paper},
            Some('Z') => {points += 3; Attack::Scissor},
            _ => panic!("not X Y or Z"),
        };

        points += match check_game(enemy, me) {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Lose => 0,
        }
    }

    println!("Part 1 Answer: {points}");

    let mut points2: i32 = 0;

    for li in lines2 {
        let enemy = match li.chars().nth(0) {
            Some('A') => Attack::Rock,
            Some('B') => Attack::Paper,
            Some('C') => Attack::Scissor,
            _ => panic!("not A B or C"),
        };

        let game_res = match li.chars().nth(2) {
            Some('X') => {points2 += 0; GameResult::Lose},
            Some('Y') => {points2 += 3; GameResult::Draw},
            Some('Z') => {points2 += 6; GameResult::Win},
            _ => panic!("not X Y or Z"),
        };

        let alt_attacks = vec![Attack::Rock, Attack::Paper, Attack::Scissor];
        for att in alt_attacks {
            if check_game(enemy, att) == game_res {
                match att {
                    Attack::Rock => points2 += 1,
                    Attack::Paper => points2 += 2,
                    Attack::Scissor => points2 += 3,
                }
            }
        }
    }

    println!("Part 2 Answer: {points2}");

}

fn check_game(enemy: Attack, me: Attack) -> GameResult {
    if enemy == me {
        return GameResult::Draw;
    }

    return match enemy {
        Attack::Rock => check_rock(me),
        Attack::Paper => check_paper(me),
        Attack::Scissor => check_scissor(me),
    };
}

fn check_rock(me: Attack) -> GameResult {
    return match me {
        Attack::Paper => GameResult::Win,
        Attack::Scissor => GameResult::Lose,
        Attack::Rock => GameResult::Draw,
    };
}

fn check_paper(me: Attack) -> GameResult {
    return match me {
        Attack::Scissor => GameResult::Win,
        Attack::Rock => GameResult::Lose,
        Attack::Paper => GameResult::Draw,
    };
}

fn check_scissor(me: Attack) -> GameResult {
    return match me {
        Attack::Rock => GameResult::Win,
        Attack::Paper => GameResult::Lose,
        Attack::Scissor => GameResult::Draw,
    };
}
