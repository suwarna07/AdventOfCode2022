use std::io::{BufRead, BufReader};
use std::fs::File;


fn get_priority(charac : char) -> u32 {
    let mut total : u32 = 0;
    if charac.is_lowercase() {
        total += charac as u32 - 96;
    }
    else {
        total += charac as u32 - 38;
    }
    total
}

fn parta() {
    let reader = BufReader::new(File::open("input").expect("Cannot open file.txt"));
    let mut total : u32 = 0;
    for line in reader.lines() {
        let test = line.unwrap();

        let (first_compart, last_compart) = test.split_at(test.len()/2);

        for charac in first_compart.chars() {
            if last_compart.contains(charac) {
                total += get_priority(charac);
                break;
            }
        }
    }
    println!("Part A Total Found {}",  total);

}

fn main() {
    let reader = BufReader::new(File::open("input_samp").expect("Cannot open file.txt"));
    let mut total : u32 = 0;
    let mut counter = 0;

    let mut group: [String; 3] = Default::default();
    let mut small_index = 0;

    for line in reader.lines() {
        let test = line.unwrap();
        let curr_index = counter%3;

        group[curr_index] = test.clone();
        if group[small_index].len() > test.len() {
            small_index = curr_index;
        }

        match counter%3 {
            2 => {
                for charac in group[0].chars() {
                    if group[1].contains(charac) && group[2].contains(charac) {
                        total += get_priority(charac);
                        println!("Char: {} found total {}", charac, total);
                        break;
                    }
                }
            },
            _ => println!("WTF"),
        }
        counter += 1;
    }
}
