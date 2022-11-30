struct CPU {
    cycle: i32,
    x: i32,
    current_command: Option<Command>,
    waiting_cycles: usize,
    total_strength: i32,
}

enum Command {
    AddX(i32),
    Noop
}

impl CPU {
    fn new() -> CPU {
        CPU {
            current_command: None,
            cycle: 0,
            waiting_cycles: 0,
            x: 1,
            total_strength: 0
        }
    }

    fn draw_pixel(&mut self) {
        let pixel_range = self.x-1..self.x+2;
        let position = self.cycle - 1;

        if pixel_range.contains(&(position % 40)) {
            print!("##")
        } else {
            print!("  ");
        }
        if (position + 1) % 40 == 0 {
            println!();
        }
    }

    fn get_waiting_cycles(command: &Command) -> usize {
        match command {
            Command::AddX(_) => 2,
            Command::Noop => 1
        }
    }
    
    fn addx(&mut self, v: i32) {
        self.x = self.x + v;
    }

    fn noop(&mut self) {
    }

    pub fn tick(&mut self) {
        self.cycle = self.cycle + 1;
        self.waiting_cycles = self.waiting_cycles.saturating_sub(1);

        self.draw_pixel();

        if (self.cycle - 20) % 40 == 0 {
            self.total_strength = self.total_strength + (self.cycle * self.x);
        }

        if self.waiting_cycles == 0 && self.current_command.is_some() {
            self.execute_current_command();
        }
    }

    fn execute_current_command(&mut self) {
        match self.current_command.as_ref().unwrap() {
            Command::AddX(v) => self.addx(*v),
            Command::Noop => self.noop() 
        }
        self.current_command = None;
    }

    pub fn queue(&mut self, command: Command) {
        self.waiting_cycles = CPU::get_waiting_cycles(&command);
        self.current_command = Some(command);
    }
}

pub fn run_part(_input: &String, _part: u8) {
    let mut cpu = CPU::new();
    let mut lines = _input.lines();

    loop {
        if cpu.current_command.is_none() {
            if let Some(next) = lines.next() {
                let mut split = next.split(" ");
                cpu.queue(match split.next().unwrap() {
                    "addx" => Command::AddX(split.next().unwrap().parse::<i32>().unwrap()),
                    "noop" => Command::Noop,
                    _ => panic!()
                });
            } else {
                break;
            }
        }
        cpu.tick();
    }
    println!("{}", cpu.total_strength);
}

pub fn run(_input: &String) {
    run_part(_input, 1);
    // run_part(_input, 2);
}
