use std::io;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u8,
    y: u8,
    z: u8,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(",").collect();
        if parts.len() != 3 {
            return Err(());
        }
        let x: u8 = parts[0].parse().map_err(|_| ())?;
        let y: u8 = parts[1].parse().map_err(|_| ())?;
        let z: u8 = parts[2].parse().map_err(|_| ())?;
        Ok(Point { x: x, y: y, z: z })
    }
}

struct Field {
    f: Vec<bool>,
    max_x: u8,
    max_y: u8,
    max_z: u8,
}

impl Field {
    fn index(&mut self, x: u8, y: u8, z: u8) -> &mut bool {
        &mut self.f[x as usize * (self.max_y + 1) as usize * (self.max_z + 1) as usize
            + y as usize * (self.max_z + 1) as usize
            + z as usize]
    }
}

fn main() {
    let mut points = Vec::new();
    loop {
        let mut line = String::new();

        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if bytes == 0 {
            break;
        }

        points.push(line.trim().parse::<Point>().expect("couldn't parse line"));
    }
    let max_x = points.iter().max_by(|p1, p2| p1.x.cmp(&p2.x)).unwrap().x;
    let max_y = points.iter().max_by(|p1, p2| p1.y.cmp(&p2.y)).unwrap().y;
    let max_z = points.iter().max_by(|p1, p2| p1.z.cmp(&p2.z)).unwrap().z;

    let f = vec![false; (max_x as usize + 1) * (max_y as usize + 1) * (max_z as usize + 1)];
    let mut field = Field {
        f,
        max_x,
        max_y,
        max_z,
    };
    for p in &points {
        *field.index(p.x, p.y, p.z) = true;
    }

    let mut sides = 0;
    for p in points {
        if p.x == 0 || !*field.index(p.x - 1, p.y, p.z) {
            sides += 1;
        }
        if p.x == field.max_x || !*field.index(p.x + 1, p.y, p.z) {
            sides += 1;
        }
        if p.y == 0 || !*field.index(p.x, p.y - 1, p.z) {
            sides += 1;
        }
        if p.y == field.max_y || !*field.index(p.x, p.y + 1, p.z) {
            sides += 1;
        }
        if p.z == 0 || !*field.index(p.x, p.y, p.z - 1) {
            sides += 1;
        }
        if p.z == field.max_z || !*field.index(p.x, p.y, p.z + 1) {
            sides += 1;
        }
    }
    println!("{sides}");
}
