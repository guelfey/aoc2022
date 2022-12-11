use std::io;
use std::io::Read;

const MARKER_SIZE : usize = 4;

fn is_marker(buf: &[u8]) -> bool {
    for i in 0..buf.len() {
        let b = buf[i];
        for j in 0..buf.len() {
            if i != j && buf[j] == b {
                return false;
            }
        }
    }
    true
}

fn first_marker(b: &Vec<u8>) -> Option<usize> {
    for i in 0..b.len()-MARKER_SIZE {
        if is_marker(&b[i..i+MARKER_SIZE]) {
            return Some(i+MARKER_SIZE)
        }
    }
    None
}

fn main() {
    let mut b = Vec::new();
    io::stdin().read_to_end(&mut b).expect("failed to read input");
    println!("{}", first_marker(&b).expect("input should contain marker"));
}
