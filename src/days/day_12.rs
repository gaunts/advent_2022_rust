use std::{collections::{VecDeque, HashMap}};

fn get_map(_input: &String) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
    let mut map: Vec<Vec<u8>> = vec!();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in _input.lines().enumerate() {
        let mut line_vec = vec!();
        for (x, c) in line.bytes().enumerate() {
            if c == b'S' {
                start = (x, y);
            } else if c == b'E' {
                end = (x, y);
            }
            line_vec.push(c);
        }
        map.push(line_vec);
    }
    (map, start, end)
}

fn neighbors(map: &Vec<Vec<u8>>, current: (usize, usize)) -> Vec<(usize, usize)> {
    let mut targets = Vec::with_capacity(4);//Vec<Position>
    // let mut res = vec!();

    let current_value = map[current.1][current.0];

    if current.0 < map[0].len() - 1 {
        if map[current.1][current.0 + 1] >= current_value - 1 {
            targets.push((current.0 + 1, current.1));
        }
    }
    if current.0 > 0 {
        if map[current.1][current.0 - 1] >= current_value - 1 {
            targets.push((current.0 - 1, current.1));
        }
    }
    if current.1 > 0 {
        if map[current.1 - 1][current.0] >= current_value - 1 {
            targets.push((current.0, current.1 - 1));
        }
    }
    if current.1 < map.len() - 1{
        if map[current.1 + 1][current.0] >= current_value - 1 {
            targets.push((current.0, current.1 + 1));
        }
    }
    targets
}

pub fn run_part(_input: &String, _part: u8) {
    let (mut map, mut start, mut end) = get_map(_input);
    map[start.1][start.0] = b'a';
    map[end.1][end.0] = b'z';
    start = end;
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    let mut frontier: VecDeque<(usize, usize)> = VecDeque::new();
    frontier.push_back(start);
    let mut total_calculating_frontier = std::time::Duration::from_millis(0);

    while !frontier.is_empty() {
        let current = frontier.pop_front().unwrap();
        let now = std::time::Instant::now();
        let neighbors = neighbors(&map, current);
        total_calculating_frontier += now.elapsed();
        for next in neighbors {
            if came_from.get(&next).is_none() {
                frontier.push_back(next);
                came_from.insert(next, current);
            }
            if map[next.1][next.0] == b'a' {
                end = next;
                frontier.clear();
                break;
            }
        }
    }
    dbg!(total_calculating_frontier);

    let mut current = end;
    let mut path = vec!();
    while current != start {
        path.push(current);
        current = came_from[&current];
    }
    println!("{}", path.len());
}

pub fn run(_input: &String) {
    // run_part(_input, 1);
    run_part(_input, 2);
}
