mod days;
use days::*;
use std::fs;
use std::env;

use std::time::Instant;
use chrono::{Utc, Datelike};

fn get_day_run(day: u32) -> fn(&String) -> () {
    type DayRun = fn(&String) -> ();
    static DAYRUNS:[DayRun; 25] = [
        day_01::run,
        day_02::run,
        day_03::run,
        day_04::run,
        day_05::run,
        day_06::run,
        day_07::run,
        day_08::run,
        day_09::run,
        day_10::run,
        day_11::run,
        day_12::run,
        day_13::run,
        day_14::run,
        day_15::run,
        day_16::run,
        day_17::run,
        day_18::run,
        day_19::run,
        day_20::run,
        day_21::run,
        day_22::run,
        day_23::run,
        day_24::run,
        day_25::run,
    ];
    DAYRUNS[day as usize -1]
}

fn main() {
    let mut day = Utc::now().day();
    if env::args().len() == 2 {
        let args: Vec<String> = env::args().collect();
        day = args[1].parse().unwrap();
    }

    let input_file_name = format!("inputs/input_{:02}.txt", day);
    let input= fs::read_to_string(&input_file_name)
        .expect(&format!("Could not open the file {}", &input_file_name));

    let now = Instant::now();
    get_day_run(day)(&input);
    let duration = now.elapsed();
    dbg!(duration);
}
