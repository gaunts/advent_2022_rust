use rangemap::set::RangeSet;
use itertools::Itertools;
use rayon::prelude::*;

pub fn run_part(_input: &String, _part: u8) {
    let _input = _input.replace("Sensor at x=", "")
        .replace("closest beacon is at x=", "")
        .replace("y=", "");

    let parsed_input = _input
        .lines()
        .map(|line| {
            line
                .split(": ")
                .map(|s| s.split(", "))
                .flatten()
                .map(|s| s.parse::<i32>().unwrap())
                .batching(|it| Some(((it.next().unwrap(), it.next().unwrap()), (it.next().unwrap(), it.next().unwrap()))))
                .next()
                .unwrap()
        })
        .collect_vec();

    let minus = parsed_input
        .iter()
        .unique_by(|(_, beacon)| beacon)
        .filter(|(_, beacon)| beacon.1 == 10)
        .count() as i32;

    let range = if _part == 1 { 2000000..2000001 } else { 0..4000000 };
    range.into_par_iter().for_each(|searched_y| {
        let mut ranges = RangeSet::new();
        for (sensor, beacon) in &parsed_input {
            let distance_covered = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
            let distance_to_y = (sensor.1 - searched_y).abs();
            if distance_to_y > distance_covered {
                continue;
            }
            let covered_size_on_y = ((distance_covered - distance_to_y) * 2) + 1;
            ranges.insert((sensor.0 - (covered_size_on_y / 2))..(sensor.0 + (covered_size_on_y / 2) + 1));
        }
    
        if _part == 1 {
            dbg!(&ranges);
            let max = ranges
                .iter()
                .last()
                .unwrap()
                .end;
            let first = ranges.iter().next().unwrap().start;
            dbg!((max - first).abs() - minus);
        } else {
            for g in ranges.gaps(&(0..4000000)) {
                dbg!(4000000 as u64 * g.start.abs() as u64 + searched_y as u64);
                return;
            }
        }
    });

}

pub fn run(_input: &String) {
    run_part(_input, 1);
    run_part(_input, 2);
}
