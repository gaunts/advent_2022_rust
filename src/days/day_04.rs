use std::ops::Range as Range;

fn get_range_pair(line: &str) -> (Range<i32>, Range<i32>) {
    let mut ranges = line.split(',')
    .map(|part| {
        let mut split = part.split('-');
        (split.next().unwrap(), split.next().unwrap())
    }).map(|tuple| -> Range<i32> { tuple.0.parse().unwrap()..tuple.1.parse().unwrap()});

    (ranges.next().unwrap(), ranges.next().unwrap())
}

fn range_contains(first: &Range<i32>, second: &Range<i32>) -> bool {
    (first.start <= second.start && first.end >= second.end)
    || (second.start <= first.start && second.end >= first.end)
}

fn range_overlaps(first: &Range<i32>, second: &Range<i32>) -> bool {
    (first.start <= second.start && first.end >= second.start)
    || (second.start <= first.start && second.end >= first.start)
}

pub fn run_part(_input: &String, _part: u8) {
    let range_pairs: Vec<(Range<i32>, Range<i32>)> = _input.lines().map(|line| get_range_pair(line)).collect();

    if _part == 1 {
        println!("{}", range_pairs.iter().filter(|ranges| range_contains(&ranges.0, &ranges.1)).count());
    } else {
        println!("{}", range_pairs.iter().filter(|ranges| range_overlaps(&ranges.0, &ranges.1)).count());
    }
}

pub fn run(_input: &String) {
    run_part(_input, 1);
    run_part(_input, 2);
}
