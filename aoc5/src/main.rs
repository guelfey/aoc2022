use std::io;
use std::io::Read;

fn parse_bottom_line(s: &str) -> Option<usize> {
    dbg!(s);
    match s.trim().rsplit_once(' ') {
        Some((_, s2)) => s2.parse::<usize>().ok(),
        None => None,
    }
}

fn parse_containers(input: &str, nstacks: usize) -> Vec<u8> {
    let mut r = Vec::with_capacity(nstacks);
    let mut s = input;
    loop {
        if s.starts_with("   ") {
            r.push(' ' as u8);
            if s.len() >= 4 {
                s = &s[4..];
            } else {
                break;
            }
        } else {
            let start = s.find('[');
            if start.is_none() {
                break;
            }
            let c = s.as_bytes()[start.unwrap()+1];
            r.push(c);
            s = &s[start.unwrap()+3..];
        }
    }
    r
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    let parts : Vec<&str> = input.split("\n\n").collect();
    if parts.len() != 2 {
        panic!("invalid input format");
    }

    //dbg!(parts[0]);

    let mut lines = parts[0].lines().rev();
    let nstacks = parse_bottom_line(lines.next().expect("no bottom line in first section")).expect("failed to parse bottom line");
    let mut stacks = vec![String::from(""); nstacks];
    for l in lines {
        let containers = parse_containers(l, nstacks);
        for (i, u) in containers.iter().enumerate() {
            let c = char::from(*u);
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }
    dbg!(&stacks);

    for l in parts[1].lines() {
        let count_str = l.strip_prefix("move ")
            .expect("no move prefix")
            .split_once(" ")
            .expect("no space after move")
            .0;
        let count : usize = count_str.parse().expect("failed to parse count");
        //dbg!(count);
        let from_str = l.split_once("from ")
            .expect("no from in line")
            .1
            .split_once(" ")
            .expect("no space after from")
            .0;
        let from : usize = from_str.parse().expect("failed to parse from");
        //dbg!(from);

        let to_str = l.split_once("to ")
            .expect("no to in line")
            .1;
        let to : usize = to_str.parse().expect("failed to parse to");
        //dbg!(to);

        for _ in 0..count {
            let c = stacks[from-1].pop().expect("stack empty");
            stacks[to-1].push(c);
        }
    }
    for stack in stacks {
        print!("{}", stack.chars().rev().next().expect("stack empty"));
    }
    println!("");
}
