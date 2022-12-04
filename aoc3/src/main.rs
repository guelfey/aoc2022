use std::io;

fn char_idx(c: char) -> Result<usize, char> {
    match c {
        'a'..='z' => Ok(c as usize - 'a' as usize),
        'A'..='Z' => Ok(26 + c as usize - 'A' as usize),
        _ => Err(c),
    }
}

fn get_common_char(s1: &str, s2: &str, s3: &str) -> Result<char, char> {
    let mut char_present1: [bool; 2*26] = [false; 52];
    for c in s1.chars() {
        let i = char_idx(c)?;
        char_present1[i] = true;
        println!("present 1: {c}");
    }
    let mut char_present2: [bool; 2*26] = [false; 52];
    for c in s2.chars() {
        let i = char_idx(c)?;
        if char_present1[i] {
            char_present2[i] = true;
            println!("present 2: {c}");
        }
    }
    for c in s3.chars() {
        let i = char_idx(c)?;
        if char_present2[i] {
            println!("present 3: {c}");
            return Ok(c);
        }
    }
    Err(' ')
}

fn main() {
    let mut total : usize = 0;

'Outer:
    loop {
        let mut lines : Vec<String> = Vec::new();

        for _ in 1..=3 {
            let mut line = String::new();

            let bytes = io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
                
            if bytes == 0 {
                break 'Outer;
            }
            lines.push(line.trim().to_string());
        }
  

        let common_char = get_common_char(lines[0].as_str(), lines[1].as_str(), lines[2].as_str())
            .expect("invalid character");
        total += char_idx(common_char).expect("invalid character") + 1;
    }
    println!("{total}");
}
