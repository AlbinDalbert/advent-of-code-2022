use cli_toolbox::*;

mod reading;
mod day1;
mod day2;
mod day3;

fn main() {
    let mut sys = system::System::new("advent of code".to_string(), None, None);

    sys.add_program("INFO".to_string(), print_info, None);
    sys.add_program("day 1".to_string(), day1::day1, None);
    sys.add_program("day 2".to_string(), day2::day2, None);
    sys.add_program("day 3".to_string(), day3::day3, None);

    loop {
        sys.menu();
    }

}


fn print_info(){
    println!("
    This is my solusions for Advent of Code 2022.
    My main goal will be to make clear and easy to read code.
    Not to write the most optimized solution.
    I might try to make more optimized versions in the future
    ");
}
