use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;



fn main() {
    let reader = BufReader::new(File::open("input").expect("Cannot open file.txt"));

    for line in reader.lines() {
        let mut index = 0;
        let mut count = 0;
        let mut chars = HashMap::new();
        let mut test = line.unwrap();
        for c in test.chars() {
            if(count - index == 14) {
                println!("SP: found {} count {}", index+14, count);
                break;
            }
            if chars.contains_key(&c){
                if(chars[&c]+1 > index) {
                    index = chars[&c]+1;
                }
            }
            chars.insert(c, count);
            count +=1;
        }
    }
}
