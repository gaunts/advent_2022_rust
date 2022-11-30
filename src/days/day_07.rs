use std::str::Lines as Lines;

#[derive(Debug)]
enum Entry {
    File { _name: String, size: u32 },
    Directory(Directory)
}

#[derive(Debug)]
struct Directory {
    pub name: String,
    pub entries: Vec<Entry>,
}

impl Directory {
    pub fn size(&self) -> u32 {
        self.entries.iter().map(|e| match e {
            Entry::Directory(dir) => dir.size(),
            Entry::File { _name: _, size } => *size
        }).sum()
    }

    pub fn get_sub_directory(&mut self, name: &str) -> &mut Directory {
        for entry in self.entries.iter_mut() {
            match entry {
                Entry::Directory(dir) => if dir.name == name { return dir; },
                _ => ()
            }
        }
        panic!("Dir {} not found", name);
    }

    pub fn add_items_from_lines(&mut self, lines: &Vec<&str>) {
        for line in lines {
            let split = line.split(" ").collect::<Vec<&str>>();
            match split[0] {
                "dir" => self.entries.push(Entry::Directory(Directory { name: split[1].to_owned(), entries: vec!() })),
                _ => self.entries.push(Entry::File { _name: split[1].to_owned(), size: split[0].parse::<u32>().unwrap() })
            }
        }
    }
}

fn compute(current: &mut Directory, lines: &mut Lines) -> () {
    while let Some(line) = lines.next() {
        let split = line.split(" ").collect::<Vec<&str>>();
        match split[1] {
            "cd" => {
                if split[2] == ".." { 
                    return;
                } else {
                    compute(current.get_sub_directory(split[2]), lines)
                }
            }
            "ls" => {
                let ls_lines: Vec<&str> = lines.clone().take_while(|l| !l.starts_with("$")).collect();
                current.add_items_from_lines(&ls_lines);
                for _ in 0..ls_lines.len() { lines.next(); }
            },
            _ => panic!("tried to compute line {}", line)
        }
    }
}

fn get_sizes(current: &Directory) -> Vec<u32> {
    let mut res = vec!();

    for entry in &current.entries {
        match entry {
            Entry::Directory(dir) => {
                res.push(dir.size());
                res.append(&mut get_sizes(dir));
            }
            _ => ()
        }
    }
    res
}

pub fn run_part(_input: &String, _part: u8) {
    let mut lines = _input.lines();
    lines.next(); // skipping "cd /"
    let mut root = Directory {name: "/".to_owned(), entries: vec!()};

    compute(&mut root, &mut lines);

    let mut sizes = get_sizes(&root);
    sizes.sort();

    if _part == 1 {
        println!("{}", sizes.iter().filter(|s| **s <= 100000_u32).sum::<u32>());
    } else {
        let free_space = 70000000_u32 - root.size();
        let required_space = 30000000_u32 - free_space;
        println!("{}", sizes.iter().find(|s| **s >= required_space).unwrap());
    }
}

pub fn run(_input: &String) {
    run_part(_input, 1);
    run_part(_input, 2);
}
