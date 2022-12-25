use std::io;

#[derive (Clone, Copy)]
enum Op {
    Noop,
    Addx(isize),
}

impl Op {
    fn new(s: &str) -> Op {
        if s == "noop" {
            return Op::Noop;
        }
        let (s1, s2) = s.split_once(" ").expect("operation should have whitespace");
        if s1 == "addx" {
            let i = s2.parse().expect("addx operand should be integer");
            return Op::Addx(i);
        }
        panic!("can't parse operation");
    }
}

struct CPU {
    program: Vec<Op>,
    ip: usize,
    cycles: usize,
    x: isize,
    signal: isize,
}

impl CPU {
    fn instruction(&mut self) {
        let op = self.program[self.ip];
        let mut cycles = match op {
            Op::Noop => 1,
            Op::Addx(_) => 2
        };

        while cycles != 0 {
            self.cycles += 1;
            if self.cycles % 40 == 20 {
                dbg!(self.cycles);
                dbg!(self.x);
                let signal = self.x * (self.cycles as isize);
                dbg!(signal);
                self.signal += signal;
            }
            cycles -= 1;
        }
        
        if let Op::Addx(i) = op {
            self.x += i;
        }

        self.ip += 1;
    }
}

fn main() {
    let mut ops = Vec::new();
    loop {
        let mut line = String::new();

        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
            
        if bytes == 0 {
            break;
        }

        ops.push(Op::new(line.trim()));
    }

    let mut cpu = CPU{
        program: ops,
        ip: 0,
        cycles: 0,
        x: 1,
        signal: 0,
    };

    while cpu.ip < cpu.program.len() {
        cpu.instruction();
    }

    println!("{}", cpu.signal);
}
