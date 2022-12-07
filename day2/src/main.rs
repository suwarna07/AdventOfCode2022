// A - Rock B - Paper C - Scissor
// X- Rock Y - Paper Z - Scissor
// Points , 1,2,3  w-6, D-3, l-0
//
// R > S, P > R, S > P


use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let reader = BufReader::new(File::open("input").expect("Cannot open file.txt"));
    let mut total = 0;
    let mut total_new = 0;

    for line in reader.lines() {
        let test = line.unwrap();
        let words: Vec<&str> = test.split_whitespace().collect();
        println!("1 '{}' 2 '{}'", words[0], words[1]);
        total+=check_win(words[1], words[0]);
        total_new+=decrypt(words[1], words[0]);
        println!("Total: '{}', total_new: '{}'", total, total_new);

    }
    println!("Total: '{}', total_new: '{}'", total, total_new);
}

fn decrypt(mine: &str, opp: &str) -> i32 {
    match (mine, opp) {
        ("X", "A") => return 3,
        ("X", "B") => return 1,
        ("X", "C") => return 2,
        ("Y", "A") => return 4,
        ("Y", "B") => return 5,
        ("Y", "C") => return 6,
        ("Z", "A") => return 8,
        ("Z", "B") => return 9,
        ("Z", "C") => return 7,
        _ => println!("WTF"),
    }
    0
}

fn check_win(mine: &str, opp: &str) -> i32 {
    match (mine, opp) {
        ("X", "A") => return 3+1,
        ("X", "B") => return 0+1,
        ("X", "C") => return 6+1,
        ("Y", "A") => return 6+2,
        ("Y", "B") => return 3+2,
        ("Y", "C") => return 0+2,
        ("Z", "A") => return 0+3,
        ("Z", "B") => return 6+3,
        ("Z", "C") => return 3+3,
        _ => println!("WTF"),
    }
    0
}
