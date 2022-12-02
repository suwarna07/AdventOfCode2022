//use std::env;
//use std::fs;
//
//fn main() {
//    // --snip--
//    //println!("In file {}", input_sample);
//
//    let contents = fs::read_to_string("input_sample")
//        .expect("Should have been able to read the file");
//
//    println!("With text:\n{contents}");
//}
//


use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let reader = BufReader::new(File::open("input").expect("Cannot open file.txt"));
    let mut total = 0;
    let mut highest = 0;
    let mut vec = Vec::new();

    for line in reader.lines() {
        let test = line.unwrap();
        if test == "" {
            // There is a bug here where it doesnt add the last int on the list
            // I am too lazy to add it :)
            vec.push(total);
            highest = if total>highest {total} else {highest};
            total = 0;
        }
        else {
            let value = test.parse::<i64>().unwrap();
            total += value;
        }
    }
    vec.sort();
    let sumthree = vec[vec.len()-1] + vec[vec.len()-2] + vec[vec.len()-3];
    println!("Highest '{}'", highest);
    println!("sumThree '{}'", sumthree);
}
