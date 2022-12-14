mod days;

use days::day_1 as d1;
use days::day_2 as d2;
use days::day_3 as d3;
use days::day_4 as d4;
use days::day_5 as d5;
use days::day_6 as d6;
use days::day_7 as d7;
use days::day_8 as d8;
use days::day_9 as d9;
use days::day_10 as d10;
use days::day_11 as d11;
use days::day_12 as d12;
use days::day_13 as d13;
use days::day_14 as d14;
use days::day_15 as d15;
use days::day_16 as d16;
use days::day_17 as d17;
use days::day_18 as d18;
use days::day_19 as d19;
use days::day_20 as d20;
use days::day_21 as d21;
use days::day_22 as d22;
use days::day_23 as d23;
use days::day_24 as d24;
use days::day_25 as d25;


fn main() {
    let day = 25;

    match day {
        1 => d1::run("./src/input/day_1.txt").expect("Failed to run"),
        2 => d2::run("./src/input/day_2.txt").expect("Failed to run"),
        3 => d3::run("./src/input/day_3.txt").expect("Failed to run"),
        4 => d4::run("./src/input/day_4.txt").expect("Failed to run"),
        5 => d5::run("./src/input/day_5.txt").expect("Failed to run"),
        6 => d6::run("./src/input/day_6.txt").expect("Failed to run"),
        7 => d7::run("./src/input/day_7.txt").expect("Failed to run"),
        8 => d8::run("./src/input/day_8.txt").expect("Failed to run"),
        9 => d9::run("./src/input/day_9.txt").expect("Failed to run"),
        10 => d10::run("./src/input/day_10.txt").expect("Failed to run"),
        11 => d11::run("./src/input/day_11.txt").expect("Failed to run"),
        12 => d12::run("./src/input/day_12.txt").expect("Failed to run"),
        13 => d13::run("./src/input/day_13.txt").expect("Failed to run"),
        14 => d14::run("./src/input/day_14.txt").expect("Failed to run"),
        15 => d15::run("./src/input/day_15.txt").expect("Failed to run"),
        16 => d16::run("./src/input/day_16.txt").expect("Failed to run"),
        17 => d17::run("./src/input/day_17.txt").expect("Failed to run"),
        18 => d18::run("./src/input/day_18.txt").expect("Failed to run"),
        19 => d19::run("./src/input/day_19.txt").expect("Failed to run"),
        20 => d20::run("./src/input/day_20.txt").expect("Failed to run"),
        21 => d21::run("./src/input/day_21.txt").expect("Failed to run"),
        22 => d22::run("./src/input/day_22.txt").expect("Failed to run"),
        23 => d23::run("./src/input/day_23.txt").expect("Failed to run"),
        24 => d24::run("./src/input/day_24.txt").expect("Failed to run"),
        25 => d25::run("./src/input/day_25.txt").expect("Failed to run"),
        _ => panic!(),
    }
}
