use std::{collections::HashSet};
use core::ops::Range;

fn maybe_reverse_range(init: i32, end: i32) -> Range<i32> {
    if end < init {
        end..init+1
    } else {
        init..end+1
    }
}

fn parse(_input: &String, _part: u8) -> HashSet<(i32, i32)> {
    let mut used_points: HashSet<(i32, i32)> = HashSet::new();

    for line in _input.lines() {
        let mut split = line
            .split(" -> ")
            .map(|s| s.split(","))
            .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
            .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()));

        let mut previous: (i32, i32) = split.next().unwrap();
        for point in split {
            for x in maybe_reverse_range(previous.0, point.0) {
                used_points.insert((x, point.1));
            }
            for y in maybe_reverse_range(previous.1, point.1) {
                used_points.insert((point.0, y));
            }
            previous = point;
        }
    }

    used_points
}

#[allow(dead_code)]
fn print(initial_map: &HashSet<(i32, i32)>, grains: &HashSet<(i32, i32)>) {
    for y in 0..10 {
        for x in 490..510 {
            if initial_map.contains(&(x, y)) {
                print!("#");
            } else if grains.contains(&(x, y)) {
                print!("O");
            } else {
                print!(".")
            }
        }
        println!();
    }
    println!()
}

fn next_grain_position(current_position: &(i32, i32), used_points: &HashSet<(i32, i32)>) -> (i32, i32) {
    let test:[(i32, i32); 3] = [(current_position.0, current_position.1 + 1),
                                (current_position.0 - 1, current_position.1 + 1),
                                (current_position.0 + 1, current_position.1 + 1)];
    test
        .iter()
        .find(|p| !used_points.contains(p))
        .unwrap_or(&current_position)
        .clone()
}

fn new_grain_position(used_points: &HashSet<(i32, i32)>, lowest_y: i32) -> Option<(i32, i32)> {
    let mut grain_position = (500, 0);
    loop {
        let next_grain_position = next_grain_position(&grain_position, used_points);
        if next_grain_position == grain_position {
            return Some(grain_position);
        } else if next_grain_position.1 > lowest_y {
            return None;
        } else {
            grain_position = next_grain_position;
        }
    }
}

pub fn run_part(_input: &String, _part: u8) {
    let mut used_points = parse(_input, _part);
    let mut lowest_y = used_points
    .iter()
    .map(|p| p.1)
    .max()
    .unwrap();

    if _part == 2 {
        lowest_y += 2;
        used_points.extend((0..1000)
            .map(|x| (x, lowest_y)));
    }

    let _initial_map = used_points.clone();
    let mut _grains: HashSet<(i32, i32)> = HashSet::new();
    
    let mut res = 0;
    while let Some(p) = new_grain_position(&used_points, lowest_y) {
        used_points.insert(p);
        // _grains.insert(p);
        // print(&_initial_map, &grains);
        res += 1;
        if p == (500, 0) {
            break;
        }
    }
    dbg!(res);
}

pub fn run(_input: &String) {
    run_part(_input, 1);
    run_part(_input, 2);
}
