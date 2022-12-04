use std::error::Error;
use std::io;

#[derive(Clone, Copy)]
struct Sect {
    start: u32,
    end: u32,
}

fn parse_sect(s: &str) -> Result<Sect, Box<dyn Error>> {
    match s.split_once('-') {
        None => Err(Box::<dyn Error>::from("no dash in section")),
        Some((s1, s2)) => {
            let start : u32 = s1.parse()?;
            let end : u32 = s2.parse()?;

            Ok(Sect{ start, end})
        }
    }
}

#[derive(Clone, Copy)]
struct Pair {
    s1: Sect,
    s2: Sect,
}

fn parse_pair(s: &str) -> Result<Pair, Box<dyn Error>> {
    match s.split_once(',') {
        None => Err(Box::<dyn Error>::from("no comma in pair")),
        Some((s1, s2)) => {
            let p1 = parse_sect(s1)?;
            let p2 = parse_sect(s2)?;

            Ok(Pair{s1: p1, s2: p2})
        }
    }
}

fn overlaps(s1 : Sect, s2 : Sect) -> bool {
    (s1.start >= s2.start && s1.start <= s2.end) ||
    (s1.end >= s2.start && s1.end <= s2.end)
}

fn redundant_pair(p: Pair) -> bool {
    overlaps(p.s1, p.s2) || overlaps(p.s2, p.s1)
}

fn main() {
    let mut count : u32 = 0;
    
    loop {
        let mut line = String::new();

        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
            
        if bytes == 0 {
            break;
        }

        let pair = parse_pair(line.trim()).expect("parsing failed");

        if redundant_pair(pair) {
            count += 1;
        }
    }
    println!("{count}");
}
