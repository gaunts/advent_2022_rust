fn initial_stack(_input: &String) -> Vec<Vec<char>> {
    let lines: Vec<&str> = _input.lines().collect();
    let max_stack_size = lines.iter().take_while(|line| line.contains('[')).count();
    let number_of_stacks = lines[max_stack_size].len() / 4 + 1;
    let mut stacks: Vec<Vec<char>> = vec![vec!(); number_of_stacks];

    for i in (0..max_stack_size).rev() {
        for y in 0..number_of_stacks {
            if lines[i].as_bytes()[y * 4 + 1] != b' ' {
                stacks[y].push(char::from_u32(lines[i].as_bytes()[y * 4 + 1] as u32).unwrap());
            }
        }
    }
    stacks
}

pub fn run_part(_input: &String, _part: u8) {
    let mut stacks = initial_stack(_input);
    let iter = _input.lines().skip_while(|line| !line.starts_with("move"));
    let mut moving_packs: Vec<char> = vec!();

    for line in iter {
        let split: Vec<&str> = line.split(' ').collect();
        let nb = split[1].parse::<usize>().unwrap();
        let from = split[3].parse::<usize>().unwrap();
        let to = split[5].parse::<usize>().unwrap();
        
        for _ in 0..nb {
            moving_packs.push(stacks[from-1].pop().unwrap());
        }
        if _part == 2 {
            moving_packs.reverse();
        }
        stacks[to-1].append(&mut moving_packs);
    }
    println!("{}", stacks.iter().map(|v| v.last().unwrap()).collect::<String>());
}

pub fn run(_input: &String) {
    run_part(_input, 1);
    run_part(_input, 2);
}
