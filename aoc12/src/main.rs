use std::io;
use std::collections::VecDeque;

struct Node {
    elevation: i8,
    distance: isize,
    visited: bool,
}

fn neighbours(c: (usize, usize), f: &Vec<Vec<Node>>) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let ysize = f.len();
    let xsize = f[c.0].len();
    let own_elevation = f[c.0][c.1].elevation;

    let mut candidates = Vec::new();
    if c.0+1 < ysize {
        candidates.push((c.0+1, c.1));
    }
    if c.0 > 0 {
        candidates.push((c.0-1, c.1));
    }
    if c.1+1 < xsize {
        candidates.push((c.0, c.1+1));
    }
    if c.1 > 0 {
        candidates.push((c.0, c.1-1));
    }
    for can in candidates {
        let elevation = f[can.0][can.1].elevation;
        if !f[can.0][can.1].visited && own_elevation+1 >= elevation {
            res.push(can);
        }
    }
    res
}

fn main() {
    let mut start = None;
    let mut end = None;
    let mut field = Vec::new();
    loop {
        let mut line = String::new();

        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
            
        if bytes == 0 {
            break;
        }

        let mut field_line = Vec::new();

        for (i, c) in line.trim().chars().enumerate() {
            let height_char = match c {
                'S' => 'a',
                'E' => 'z',
                _ => c,
            };
            let coords = (field.len(), i);
            field_line.push(Node{
                elevation: height_char as i8 - 'a' as i8,
                distance: -1,
                visited: false,
            });
            if c == 'S' {
                start = Some(coords);
            }
            if c == 'E' {
                end = Some(coords);
            }
        }
        field.push(field_line);
    }

    let real_start = start.expect("should have found start");
    let real_end = end.expect("should have found end");

    let mut queue : VecDeque<(usize, usize)> = VecDeque::new();
    field[real_start.0][real_start.1].distance = 0;
    queue.push_back(real_start);
    while !queue.is_empty() {
        let c = queue.pop_front().expect("queue should not be empty");
        if field[c.0][c.1].visited {
            continue;
        }
        dbg!(c);
        field[c.0][c.1].visited = true;
        let new_d = field[c.0][c.1].distance + 1;
        for n in neighbours(c, &field) {
            let d = field[n.0][n.1].distance;
            if d == -1 || new_d < d {
                field[n.0][n.1].distance = new_d;
            }
            queue.push_back(n);
        }
    }
    println!("{} {}", real_end.0, real_end.1);
    println!("{}", field[real_end.0][real_end.1].distance);
}
