use cli_toolbox::*;

mod reading;
mod day1;
mod day2;

fn main() {
    let mut sys = system::System::new("advent of code".to_string(), None, None);

    sys.add_program("day 1".to_string(), day1::day1, None);
    sys.add_program("day 2".to_string(), day2::day2, None);

    loop {
        sys.menu();
    }

}
