use std::cmp;
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

fn main() {
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

    let max = sensors
        .iter()
        .max_by_key(|s| s.pos.x + s.xdist)
        .expect("could not find max x");
    let min = sensors
        .iter()
        .min_by_key(|s| s.pos.x - s.xdist)
        .expect("could not find min x");
    let x_max = max.pos.x + max.xdist;
    let x_min = min.pos.x - max.xdist;

    const y: isize = 2_000_000;
    //const y: isize = 10;
    let mut total = 0;

    'outer: for x in x_min..=x_max {
        let p = Point { x, y };
        // don't count any beacons that are exactly on that line
        for s in &sensors {
            if p == s.beacon {
                continue 'outer;
            }
        }
        for s in &sensors {
            if s.pos.dist(&p) <= s.dist {
                total += 1;
                break;
            }
        }
    }
    println!("{total}");
}
