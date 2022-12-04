use std::io;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}



fn parse_other(s: &str) -> Result<Choice, String> {
    match s {
        "A" => Ok(Choice::Rock),
        "B" => Ok(Choice::Paper),
        "C" => Ok(Choice::Scissors),
        _ => Err("not matching".to_string()),
    }
}

fn parse_own(s: &str, own: Choice) -> Result<Choice, String> {
    match s {
        "X" => Ok(match own {
            // need to lose
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }),
        "Y" => Ok(own),
        "Z" => Ok(match own {
            // winning
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }),
        _ => Err("not matching".to_string()),
    }
}

fn score(own: Choice, other: Choice) -> u32 {
    let mut score : u32 = match own {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };
    if own == other {
        score += 3;
    }
    if (own == Choice::Rock && other == Choice::Scissors)
        || (own == Choice::Paper && other == Choice::Rock)
        || (own == Choice::Scissors && other == Choice::Paper) {
        score += 6;
    }
    score
}

fn main() {
    let mut total_score : u32 = 0;
    loop {
        let mut line = String::new();

        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
            
        if bytes == 0 {
            break;
        }

        let syms : Vec<&str> = line.split_whitespace().collect();
        if syms.len() != 2 {
            continue;
        }
        let other_choice = parse_other(syms[0]).expect("failed to parse other choice");
        let own_choice = parse_own(syms[1], other_choice).expect("failed to parse own choice");
        let this_score = score(own_choice, other_choice);
        println!("{this_score}");
        total_score += this_score;
    }
    println!("{total_score}");
}
