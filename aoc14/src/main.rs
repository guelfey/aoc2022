use std::collections::VecDeque;
use std::io;
use std::str::FromStr;

#[derive (Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .trim()
            .split_once(",").ok_or(ParsePointError)?;

        let x_parsed = x.parse::<usize>().map_err(|_| ParsePointError)?;
        let y_parsed = y.parse::<usize>().map_err(|_| ParsePointError)?;
        Ok(Point{x: x_parsed, y: y_parsed})
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

struct Lines {
    v: Vec<Line>,
}

#[derive(Debug)]
enum ParseError {
    Point,
    Line,
}

impl FromStr for Lines {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = Vec::new();
        let res = s.trim()
            .split(" -> ")
            .map(Point::from_str);
        for p in res {
            points.push(p.map_err(|_| ParseError::Point)?);
        }
        if points.len() < 2 {
            return Err(ParseError::Line);
        }

        let mut lines = Vec::new();
        for i in 0..points.len()-1 {
            lines.push(Line{start: points[i], end: points[i+1]});
        }
        Ok(Lines{v: lines})
    }
}

struct Field {
    filled: VecDeque<Vec<bool>>,
    start: Point,
    ysize: usize,
}

impl Field {
    fn new(v: &[Line]) -> Field {
        let mut min_x = 500;
        let mut max_x = 500;
        let mut max_y = 0;
        for l in v {
            for p in [l.start, l.end] {
                if p.x > max_x {
                    max_x = p.x;
                }
                if p.x < min_x {
                    min_x = p.x;
                }
                if p.y > max_y {
                    max_y = p.y;
                }
            }
        }
        let xsize = max_x - min_x + 1;
        let ysize = max_y + 2;
        let mut filled = VecDeque::new();
        for _ in 0..xsize {
            filled.push_back(vec![false; ysize]);
        }
        let start = Point{
            x: 500 - min_x,
            y: 0,
        };

        for l in v {
            if l.start.x == l.end.x {
                let (start, end) = if l.start.y < l.end.y {
                    (l.start.y, l.end.y)
                } else {
                    (l.end.y, l.start.y)
                };
                for y in start..=end {
                    filled[l.start.x - min_x][y] = true;
                }
            } else if l.start.y == l.end.y {
                let (start, end) = if l.start.x < l.end.x {
                    (l.start.x, l.end.x)
                } else {
                    (l.end.x, l.start.x)
                };
                for x in start..=end {
                    filled[x - min_x][l.start.y] = true;
                }
            } else {
                panic!("invalid line {:?}", l);
            }
        }
        Field{ filled, start, ysize }
    }

    fn insert(&mut self) -> bool {
        let mut p = self.start;
        loop {
            // straight down?
            if p.y == self.ysize-1 {
                // infinite floor, always settle
                break;
            }
            if !self.filled[p.x][p.y+1] {
                p.y += 1;
                continue;
            }

            // down left?
            if p.x == 0 {
                // extend field on the left
                self.filled.push_front(vec![false; self.ysize]);
                self.start.x+=1;
                p.x += 1;
            }
            if !self.filled[p.x-1][p.y+1] {
                p.y += 1;
                p.x -= 1;
                continue;
            }

            // down right?
            if p.x == self.filled.len()-1 {
                self.filled.push_back(vec![false; self.ysize]);
            }
            if !self.filled[p.x+1][p.y+1] {
                p.y += 1;
                p.x += 1;
                continue;
            }
            break;
        }
        // settled
        self.filled[p.x][p.y] = true;
        return p != self.start;
    }

    fn print(&self) {
        for y in 0..self.ysize {
            for x in 0..self.filled.len() {
                let c = if self.filled[x][y] {
                    '#'
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!("");
        }
        println!("");
    }
}

fn main() {
    let mut all_lines = Lines{
        v: Vec::new(),
    };
    loop {
        let mut line = String::new();

        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
            
        if bytes == 0 {
            break;
        }

        let mut lines : Lines = line.parse().expect("could not parse line description");
        all_lines.v.append(&mut lines.v);
    }

    let mut field = Field::new(&all_lines.v);
    let mut count = 0;
    field.print();
    while field.insert() {
        //field.print();
        count += 1;
    }
    // last piece was inserted, but we didn't count it yet
    count += 1;
    field.print();
    println!("{count}");
}
