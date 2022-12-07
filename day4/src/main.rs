use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let reader = BufReader::new(File::open("input").expect("Cannot open file.txt"));
    let (mut total, mut total_b) = (0,0);

    for line in reader.lines() {
        let test = line.unwrap();
        let assignments: Vec<i32> = test.split(&[',','-'])
                                        .map(|a| a.parse().unwrap())
                                        .collect();
        if ( assignments[0] <= assignments[2] &&  assignments[1] >= assignments[3] )
           || (assignments[0] >= assignments[2] &&  assignments[1] <= assignments[3] ) {
            total += 1;
        }
        if assignments[0] <= assignments[2] {
            let delta = assignments[2] - assignments[0];
            if assignments[1] - assignments[0] >= delta {
                total_b +=1;
            }
        }
        else {
            let delta = assignments[0] - assignments[2];
            if assignments[3] - assignments[2] >= delta {
                total_b +=1;
            }
        }
    }
    println!("{} part a", total);
    println!("{} part b", total_b);


}
