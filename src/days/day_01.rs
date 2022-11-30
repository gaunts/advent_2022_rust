pub fn run_part(_input: &String, _part: u8) {
    let mut all_values = vec!();
    let mut current: u32 = 0;

    for l in _input.lines() {
        if l.len() != 0 {
            let value: u32 = l.parse().unwrap();
            current = current + value;
        } else {
            all_values.push(current);
            current = 0;
        }
    }
    all_values.sort_by(|a, b| b.cmp(a));

    if _part == 1 {
        println!("{}", all_values[0]);
    } else {
        println!("{}", &all_values[0..3].iter().sum::<u32>());
    }
}

pub fn run(_input: &String) {
    run_part(_input, 1);
    run_part(_input, 2);
}
