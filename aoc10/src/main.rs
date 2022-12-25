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

const CRT_WIDTH : usize = 40;

struct CPU {
    program: Vec<Op>,
    ip: usize,
    cycles: usize,
    x: isize,
}

impl CPU {

    fn instruction(&mut self) {
        let op = self.program[self.ip];
        let mut cycles = match op {
            Op::Noop => 1,
            Op::Addx(_) => 2
        };

        while cycles != 0 {
            let column = self.cycles % CRT_WIDTH;
            //dbg!(self.cycles);
            //dbg!(self.x);
            if column as isize >= self.x -1 && column as isize <= self.x + 1 {
                print!("#");
            } else {
                print!(".");
            }
            if column == CRT_WIDTH-1 {
                println!("");
            }
            self.cycles += 1;
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
    };

    while cpu.ip < cpu.program.len() {
        cpu.instruction();
    }
}
