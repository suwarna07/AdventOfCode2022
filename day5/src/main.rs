#![feature(str_split_whitespace_as_str)]
use std::io::{BufRead, BufReader};
use std::fs::File;
use regex::Regex;


fn printTopLevel(packages: Vec<Vec<char>>, col: usize) {
    let mut val = String::from("VAL ");

    for colm in 0..col {
    //    println!("HI col {}", colm);
        for row in (0..packages.len()) {
            if packages[row].len() <= colm {
                continue;
            }
            if packages[row][colm] != '?' {
                val.push(packages[row][colm]);
                break;
            }
        }
    }
    println!("val {}", val);

}

fn rearrange(packages: &mut Vec<Vec<char>>, qty: u32, from: usize, to: usize) {

    for _  in 0..qty {
        let mut charToMove = ' ';
        // find the element
        println!("Qty {}, from {}, to {}", qty, from, to);
        for row in 0..packages.len() {
            if packages[row].len() <= from {
                for idx in packages[row].len()..from {
                    println!("going to add");
                    packages[row].push('?');
                }
            }
            if packages[row][from-1] != '?' {
                charToMove = packages[row][from-1];
                println!("Moving {}", charToMove);
                packages[row][from-1] = '?';
                break;
            }
        }

        for row in (0..packages.len()).rev() {
            println!(" hi {} ", packages[row].len());
            if packages[row].len() <= to {
                for idx in packages[row].len()..to {
                    println!("going to add");
                    packages[row].push('?');
                }
            }
            println!(" hi {} ", packages[row][to-1]);
            if row == 0 && packages[0][to-1] != '?' {
                println!("Stack getting bigger lets add");
                let newStack : Vec<char> = vec!['?';packages[0].len()];
                packages.insert(0, newStack);
                println!(" ADDED");
            }
            if packages[row][to-1] == '?' {
                packages[row][to-1] = charToMove;
                println!(" Vec {:?}", packages);
                break;
            }
            else if row == 0 {
                if packages[0][to-1] != '?' {
                    println!("Stack getting bigger lets add");
                    let newStack : Vec<char> = vec!['?';packages[0].len()];
                    packages.insert(0, newStack);
                    println!(" ADDED");
                }

            }
        }
    }

}


fn rearrange_pt2(packages: &mut Vec<Vec<char>>, qty: usize, from: usize, to: usize, col: usize) {

    for i  in 0..qty {
        let mut charToMove = ' ';
        // find the element
        println!("Qty {}, from {}, to {}", qty, from, to);
        for row in 0..packages.len() {
            if packages[row].len() <= from {
                for idx in packages[row].len()..from {
                    println!("going to add");
                    packages[row].push('?');
                }
            }
            if packages[row][from-1] != '?' {
                charToMove = packages[row][from-1];
                println!("Moving {}", charToMove);
                packages[row][from-1] = '?';
                break;
            }
        }

        for row in (0..packages.len()).rev() {
            println!(" hi rowLength {} row {}", packages[row].len(), row);
            if packages[row].len() <= to {
                for idx in packages[row].len()..to {
                    println!("going to add");
                    packages[row].push('?');
                }
            }
            println!(" hi val {} ", packages[row][to-1]);
            if row == 0 && packages[0][to-1] != '?' {
                println!("Stack getting bigger lets add");
                let newStack : Vec<char> = vec!['?';packages[0].len()];
                packages.insert(0, newStack);
                println!(" ADDED");
            }
            if packages[row][to-1] == '?' {
                //check if i have enough rows
                println!("{} {} {} {}", row, qty, i, packages.len());
                let row_to_add = ((qty as isize- i as isize)-row as isize);
                println!("going to add {} row", row_to_add-1);
                for idx in 0..row_to_add-1 {
                    let newStack : Vec<char> = vec!['?';col];
                    println!("New stack added in row {}", idx);
                    packages.insert(0, newStack);
                }
                if row_to_add > 0 {
                    println!("going to add in {} {}", 0, to-1);
                    packages[0][to-1] = charToMove;
                    println!(" Vec {:?}", packages);
                    break;
                }
                else {
                    let rowToPlace = row-(qty-i-1);
                    println!("going to add in {} {}", rowToPlace, to-1);
                    packages[rowToPlace][to-1] = charToMove;
                    println!(" Vec {:?}", packages);
                }
                break;
            }
            else if row == 0 {
                if packages[0][to-1] != '?' {
                    println!("Stack getting bigger lets add");
                    let newStack : Vec<char> = vec!['?';packages[0].len()];
                    packages.insert(0, newStack);
                    println!(" ADDED");
                }

            }
        }
    }

}




fn main() {
    let reader = BufReader::new(File::open("input").expect("Cannot open file.txt"));
    let mut packages: Vec<Vec<char>> = Vec::with_capacity(100);
    let mut index = 0;
    let mut col: usize = 0;

    for line in reader.lines() {
        let mut test = line.unwrap();
        if test.contains("move") {
            for row in (0..packages.len()) {
                if packages[row].len() <= col {
                    for idx in packages[row].len()..col {
                        packages[row].push('?');
                    }
                }
            }
            println!(" Vec {:?}", packages);
            println!("Move start");
            let abcd = test.split(" ").collect::<Vec<&str>>();
            rearrange_pt2(&mut packages,
                      abcd[1].parse::<usize>().unwrap(),
                      abcd[3].parse::<usize>().unwrap(),
                      abcd[5].parse::<usize>().unwrap(),
                      col);
        }
        else if test.contains("1") {
            col = test.split(' ').last().unwrap().parse::<usize>().unwrap();
            println!("End of input {} ", col);
        }
        else if test == ""{
            println!("Do nothing");
        }
        else {
            //test = test.replace(/  +/g, ' ');
            let re = Regex::new(r"\s   ").unwrap();
            test = re.replace_all(&test, "?").to_string();
            test = test.replace(&['[', ']', ' '][..], "");
            packages.insert(index,test.chars().collect());
            println!(" Vec {:?}", packages);
            index += 1;
        }



/*
        loop {
            println!(" split {}", split.as_str());
            let ws_count = test
                .chars()
                .take_while(|ch| ch.is_whitespace() && *ch != '\n')
                .count();

            if ws_count > 1 {
                println!(" empty {} ", ws_count);
                //empty val for the row
            }
            else {
                println!(" non empty {} ", ws_count);
            }
            if !split.next().is_some() {break;}
        }
        break;
        */
    }
    printTopLevel(packages, col.try_into().unwrap());
/*    rearrange(&mut packages, 1, 2, 1);
    rearrange(&mut packages, 3, 1, 3);
    rearrange(&mut packages, 2, 2, 1);
    rearrange(&mut packages, 1, 1, 2);*/
}
