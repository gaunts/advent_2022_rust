use std::collections::HashSet;

fn move_head(position: &(i32, i32), direction: &u8) -> (i32, i32) {
    match direction {
        b'R' => (position.0 + 1, position.1),
        b'U' => (position.0, position.1 + 1),
        b'L' => (position.0 - 1, position.1),
        b'D' => (position.0, position.1 - 1),
        _ => panic!()
    }
}

fn are_adjascent(head: &(i32, i32), tail: &(i32, i32))  -> bool {
    (head.0 - tail.0).abs() <= 1
    && (head.1 - tail.1).abs() <= 1
}

fn move_tail(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    if are_adjascent(head, tail) {
        *tail
    } else {
        let mut dif_x = head.0 - tail.0;
        if dif_x != 0  {
            dif_x = dif_x / dif_x.abs();
        }
        let mut dif_y = head.1 - tail.1;
        if dif_y != 0 {
            dif_y = dif_y / dif_y.abs();
        }
        (tail.0 + dif_x, tail.1 + dif_y)
    }
}

pub fn run_part(_input: &String, _part: u8) {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut rope = vec![(0, 0); if _part == 1 { 2 } else { 10 }];

    for mut line in _input.lines().map(|l| l.split(" ")) {
        let direction = line.next().unwrap().as_bytes()[0];
        let value = line.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..value {
            rope[0] = move_head(&rope[0], &direction);
            for knot_index in 1..rope.len() {
                rope[knot_index] = move_tail(&rope[knot_index - 1], &rope[knot_index]);
            }
            visited.insert(rope[rope.len()-1]);
        }
    }
    println!("{}", visited.len());
}

pub fn run(_input: &String) {
    run_part(_input, 1);
    run_part(_input, 2);
}
