use std::io;

fn main() {
    let mut cur : u32 = 0;
    let mut totals = Vec::new();
    loop {
        let mut line = String::new();

        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
            
        if bytes == 0 {
                break;
        }
        if line.trim().len() == 0 {
            // new elf
            totals.push(cur);
            cur = 0;
        }

        let cal : u32 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        cur += cal;
    }
    if totals.len() < 3 {
        println!("Not enough elves!");
        return;
    }
    totals.sort_by(|a, b| b.cmp(a));
    let sum = totals[0] + totals[1] + totals[2];
    println!("Sum: {sum}");
}
