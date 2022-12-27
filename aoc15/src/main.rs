use std::cmp;
use std::cmp::Ordering;
use std::io;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn dist(&self, other: &Self) -> isize {
        (cmp::max(self.x, other.x) - cmp::min(self.x, other.x) + cmp::max(self.y, other.y)
            - cmp::min(self.y, other.y))
    }
    fn xdist(&self, other: &Self) -> isize {
        (cmp::max(self.x, other.x) - cmp::min(self.x, other.x))
    }
}

struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.trim().split_once(", ").ok_or(ParsePointError)?;

        let x_parsed = x
            .strip_prefix("x=")
            .ok_or(ParsePointError)?
            .parse::<isize>()
            .map_err(|_| ParsePointError)?;
        let y_parsed = y
            .strip_prefix("y=")
            .ok_or(ParsePointError)?
            .parse::<isize>()
            .map_err(|_| ParsePointError)?;
        Ok(Point {
            x: x_parsed,
            y: y_parsed,
        })
    }
}

struct Sensor {
    pos: Point,
    beacon: Point,
    dist: isize,
    xdist: isize,
}

struct EdgeIterator {
    sensor: Point,
    dist: isize,
    next: Point,
    done: bool,
}

impl Iterator for EdgeIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        if self.next.x == self.sensor.x {
            if self.next.y > self.sensor.y {
                // top
                println!("top");
                self.next.x -= 1;
                self.next.y -= 1;
            } else {
                // bottom
                println!("bottom");
                self.next.x += 1;
                self.next.y += 1;
            }
        } else if self.next.y == self.sensor.y {
            if self.next.x > self.sensor.x {
                // right
                println!("right");
                self.next.x -= 1;
                self.next.y += 1;
            } else {
                // left
                println!("left");
                self.next.x += 1;
                self.next.y -= 1;
            }
        } else if self.next.x > self.sensor.x {
            if self.next.y > self.sensor.y {
                self.next.x -= 1;
            } else if self.next.y < self.sensor.y {
                self.next.x += 1;
            }
            self.next.y += 1;
        } else if self.next.x < self.sensor.x {
            if self.next.y > self.sensor.y {
                self.next.x -= 1;
            } else if self.next.y < self.sensor.y {
                self.next.x += 1;
            }
            self.next.y -= 1;
        } else {
            panic!("unexpected coords");
        }
        let ret = self.next;
        let start = Point {
            x: self.sensor.x + self.dist + 1,
            y: self.sensor.y,
        };
        if self.next == start {
            self.done = true;
        }
        return Some(ret);
    }
}

impl Sensor {
    fn points_on_edge(&self) -> EdgeIterator {
        return EdgeIterator {
            sensor: self.pos,
            dist: self.dist,
            next: Point {
                x: self.pos.x + self.dist + 1,
                y: self.pos.y,
            },
            done: false,
        };
    }
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pos = s
            .strip_prefix("Sensor at ")
            .ok_or(())?
            .split_once(":")
            .ok_or(())?
            .0
            .parse::<Point>()
            .map_err(|_| ())?;

        let beacon = s
            .split_once("closest beacon is at ")
            .ok_or(())?
            .1
            .parse::<Point>()
            .map_err(|_| ())?;

        let dist = pos.dist(&beacon);
        let xdist = pos.xdist(&beacon);

        Ok(Sensor {
            pos,
            beacon,
            dist,
            xdist,
        })
    }
}

fn main() -> Result<(), String> {
    let mut sensors = Vec::new();
    loop {
        let mut line = String::new();

        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if bytes == 0 {
            break;
        }

        let sensor = line.trim().parse::<Sensor>().expect("couldn't parse line");
        sensors.push(sensor);
    }

    for s in &sensors {
        // Since we know there's exactly one valid position, we know it has to be
        // directly next to the edge of one of the sensor's range. So we only need
        // to iterate through these and check if any other sensor is in range.
        'p: for (i, p) in s.points_on_edge().enumerate() {
            if p.x < 0 || p.x > 4_000_000 || p.y < 0 || p.y > 4_000_000 {
                continue;
            }
            for other_s in &sensors {
                if other_s.pos.dist(&p) <= other_s.dist {
                    continue 'p;
                }
            }
            println!("{}", p.x * 4_000_000 + p.y);
            return Ok(());
        }
    }
    Err(String::from("not found"))
}
