use std::io;

fn parse_line(line: &str) -> Vec<u8> {
    let mut r : Vec<u8> = Vec::new();
    for c in line.chars() {
        r.push(c.to_digit(10).expect("parsing char") as u8);
    }
    r
}

fn visible_trees(field: &Vec<Vec<u8>>) -> usize {
    let mut visible = Vec::new();
    let end = field.len()-1;
    for _ in 0..=end {
        visible.push(vec![false; field.len()]);
    }
    for y in 0..=end {
        let mut max = field[y][0];
        visible[y][0] = true;
        for x in 0..=end {
            if field[y][x] > max {
                max = field[y][x];
                visible[y][x] = true;
            }
        }

        max = field[y][end];
        visible[y][end] = true;
        for x in (0..=end).rev() {
            if field[y][x] > max {
                max = field[y][x];
                visible[y][x] = true;
            }
        }
    }

    for x in 0..=end {
        let mut max = field[0][x];
        visible[0][x] = true;
        for y in 0..=end {
            if field[y][x] > max {
                max = field[y][x];
                visible[y][x] = true;
            }
        }

        max = field[end][x];
        visible[end][x] = true;
        for y in (0..=end).rev() {
            if field[y][x] > max {
                max = field[y][x];
                visible[y][x] = true;
            }
        }
    }

    visible.iter().fold(0, |count, row|
        count + row.iter().fold(0, |rowcount, elem|
            if *elem {
                rowcount+1
            } else {
                rowcount
            }
        )
    )
}

fn tree_score(field: &Vec<Vec<u8>>, tx: usize, ty: usize) -> usize {
    let h = field[ty][tx];
    let end = field.len()-1;

    let mut xscore = 0;
    for x in tx+1..=end {
        xscore += 1;
        if field[ty][x] >= h {
            break;
        }
    }

    let mut x2score = 0;
    for x in (0..tx).rev() {
        x2score += 1;
        if field[ty][x] >= h {
            break;
        }
    }

    let mut yscore = 0;
    for y in ty+1..=end {
        yscore += 1;
        if field[y][tx] >= h {
            break;
        }
    }

    let mut y2score = 0;
    for y in (0..ty).rev() {
        y2score += 1;
        if field[y][tx] >= h {
            break;
        }
    }
    
    xscore * x2score * yscore * y2score
}

fn max_score(field: &Vec<Vec<u8>>) -> usize {
    let mut max = 0;
    for x in 0..field.len() {
        for y in 0..field.len() {
            let score = tree_score(field, x, y);
            if score > max {
                max = score;
            }
            //dbg!(x, y, score);
        }
    }
    max
}

fn main() {
    let mut field = Vec::new();
    
    loop {
        let mut line = String::new();

        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
            
        if bytes == 0 {
            break;
        }

        let row = parse_line(line.trim());
        field.push(row);
    }

    let score = max_score(&field);
    println!("{score}");
}
