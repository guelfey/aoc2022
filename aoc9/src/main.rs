use std::io;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug)]
struct Move {
    size: usize,
    dir: Direction,
}

impl Move {
    fn new(s: &str) -> Move {
        let (s1, s2) = s.split_once(" ").expect("expected whitespace in line");
        let u : usize = s2.parse().expect("second part should parse as usize");
        let dir = match s1 {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("unexpected move letter"),
        };
        Move { size: u, dir: dir }
    }
}

fn dimensions(moves: &[Move]) -> (isize, isize, isize, isize) {
    let (mut x, mut y, mut xmax, mut ymax, mut xmin, mut ymin) : (isize, isize, isize, isize, isize, isize) = (0, 0, 0, 0, 0, 0);
    for m in moves {
        match m.dir {
            Direction::Right => {x += m.size as isize},
            Direction::Left => {x -= m.size as isize},
            Direction::Up => {y += m.size as isize},
            Direction::Down => {y -= m.size as isize},
        }
        if x > xmax {
            xmax = x;
        }
        if x < xmin {
            xmin = x;
        }
        if y > ymax {
            ymax = y;
        }
        if y < ymin {
            ymin = y;
        }
    }
    (xmax, xmin, ymax, ymin)
}

fn print_field(hx: usize, hy: usize, tx: usize, ty: usize, xsize: usize, ysize: usize) {
    for y in (0..ysize).rev() {
        for x in 0..xsize {
            if x == hx && y == hy {
                if x == tx && y == ty {
                    print!("B");
                } else {
                    print!("H");
                }
            } else if x == tx && y == ty {
                print!("T");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

fn update_tail(hx: usize, hy: usize, tx: usize, ty: usize) -> (usize, usize) {
    let mut tx = tx;
    let mut ty = ty;
    if hx > tx + 1 {
        if hy > ty {
            // RU
            ty += 1;
        } else if ty > hy {
            // RD
            ty -= 1;
        }
        // R
        tx += 1;
    } else if tx > hx + 1{
        if hy > ty {
            // LU
            ty += 1;
        } else if ty > hy {
            // LD
            ty -= 1;
        }
        // L
        tx -= 1;
    } else if hy > ty + 1 {
        if hx > tx {
            tx += 1;
        } else if tx > hx {
            tx -= 1;
        }
        ty += 1;
    } else if ty > hy + 1 {
        if hx > tx {
            tx += 1;
        } else if tx > hx {
            tx -= 1;
        }
        ty -= 1;
    }
    (tx, ty)
}

fn visited_fields(xsize: usize, ysize: usize, xstart: usize, ystart: usize, moves: &[Move]) -> usize {
    let mut visited = Vec::new();
    for _ in 0..xsize {
        visited.push(vec![false; ysize]);
    }
    let (mut tx, mut ty, mut hx, mut hy) = (xstart, ystart, xstart, ystart);
    for mo in moves {
        let mut m = *mo;
        dbg!(m);
        while m.size > 0 {
            match m.dir {
                Direction::Right => {hx += 1},
                Direction::Left => {hx -= 1},
                Direction::Up => {hy += 1},
                Direction::Down => {hy -= 1},
            }
            m.size -= 1;
            
            (tx, ty) = update_tail(hx, hy, tx, ty);
            //print_field(hx, hy, tx, ty, xsize, ysize);
            visited[tx][ty] = true;
        }
    }
    let mut total = 0;
    for row in visited {
        for b in row {
            if b {
                total += 1;
            }
        }
    }
    total
}

fn main() {
    let mut moves : Vec<Move> = Vec::new();

    loop {
        let mut line = String::new();

        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
            
        if bytes == 0 {
            break;
        }

        moves.push(Move::new(line.trim()));
    }

    let (xmax, xmin, ymax, ymin) = dimensions(&moves);
    println!("{xmax}, {xmin}, {ymax}, {ymin}");
    let xsize = (xmax - xmin + 1) as usize;
    let ysize = (ymax - ymin + 1) as usize;
    let xoff = (-1 * xmin) as usize;
    let yoff = (-1 * ymin) as usize;
    let v = visited_fields(xsize, ysize, xoff, yoff, &moves);
    println!("{v}");
}
