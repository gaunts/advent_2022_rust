pub fn run_part(_input: &String, _part: u8) {
    let mut score:usize = 0;

    for line in _input.lines() {
        let left = (line.as_bytes()[0] - b'A') as usize;
        let mut right = (line.as_bytes()[2] - b'X') as usize;

        if _part == 2 {
            right = match right {
                0 => (left + 2) % 3,
                1 => left,
                2 => (left + 1) % 3,
                _ => panic!()
            }
        }

        if right == (left + 1) % 3 {
            score = score + 6;
        } else if left == right {
            score = score + 3;
        }
        score = score + right + 1;
    }
    dbg!(score);
}

pub fn run(_input: &String) {
    run_part(_input, 1);
    run_part(_input, 2);
}
