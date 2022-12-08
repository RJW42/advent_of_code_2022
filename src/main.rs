mod days;

use days::day_1 as d1;
use days::day_2 as d2;
use days::day_3 as d3;
use days::day_4 as d4;
use days::day_5 as d5;
use days::day_6 as d6;
use days::day_7 as d7;
use days::day_8 as d8;


fn main() {
    let day = 8;

    match day {
        1 => d1::run("./src/input/day_1.txt").expect("Failed to run"),
        2 => d2::run("./src/input/day_2.txt").expect("Failed to run"),
        3 => d3::run("./src/input/day_3.txt").expect("Failed to run"),
        4 => d4::run("./src/input/day_4.txt").expect("Failed to run"),
        5 => d5::run("./src/input/day_5.txt").expect("Failed to run"),
        6 => d6::run("./src/input/day_6.txt").expect("Failed to run"),
        7 => d7::run("./src/input/day_7.txt").expect("Failed to run"),
        8 => d8::run("./src/input/day_8.txt").expect("Failed to run"),
        _ => panic!(),
    }
}
