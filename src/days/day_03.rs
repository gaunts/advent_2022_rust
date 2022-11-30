pub fn get_char_priority(char: char) -> i32 {
    let char = char.encode_utf8(&mut [0; 1]).as_bytes()[0];
    if char <= b'Z' {
        (char - b'A' + 27) as i32
    } else {
        (char - b'a' + 1) as i32
    }
}

pub fn get_line_priority(line: &str) -> i32 {
    let first = &line[0..line.len() / 2];
    let second = &line[(line.len() / 2)..];

    for character in first.chars() {
        if second.find(character).is_some() {
           return get_char_priority(character);
        }
    }
    panic!("No duplicate char found");
}

pub fn run_part(_input: &String, _part: u8) {
    if _part == 1 {
        dbg!(_input.lines().map(|line| get_line_priority(line)).sum::<i32>());
        return;
    }
    
    let mut total = 0;
    let lines = _input.lines().collect::<Vec<&str>>();

    let mut i = 0;
    while i < lines.len() {
        let group = &lines[i..i+3];
        for character in group[0].chars() {
            if group[1].find(character).is_some()
            && group[2].find(character).is_some() {
                total = total + get_char_priority(character);
                break;
            }
        }
        i = i + 3;
    }
    dbg!(total);
}

pub fn run(_input: &String) {
    run_part(_input, 1);
    run_part(_input, 2);
}
