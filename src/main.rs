use cli_toolbox::*;
use std::fs;

mod reading;
mod day1;

fn main() {
    let mut sys = cli_toolbox::system::System::new("advent of code".to_string(), None, None);

    sys.add_program("day 1".to_string(), day1::day1, None);

    loop {
        sys.menu();
    }

}



