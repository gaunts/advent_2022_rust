pub fn number_of_bytes(bytes: &[u8], byte: &u8) -> usize {
    bytes.iter().filter(|b| **b == *byte).count()
}

pub fn all_different(bytes: &[u8]) -> bool {
    let mut i = 0;
    for b in bytes {
        if number_of_bytes(&bytes[i..], b) != 1 {
            return false;
        }
        i = i + 1;
    }
    return true;
}

pub fn run_part(_input: &String, _part: u8) {
    let bytes = _input.as_bytes();
    let msg_size = if _part == 1 { 4 } else { 14 };
    for i in 0.._input.len() {
        if all_different(&bytes[i..i+msg_size]) {
            println!("{}", i + msg_size);
            break;
        }
    }
}

pub fn run(_input: &String) {
    run_part(_input, 1);
    run_part(_input, 2);
}
