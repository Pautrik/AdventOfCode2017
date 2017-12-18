use std::fs::File;
use std::io::prelude::*;

fn main() {
    let inputStr = readFile(*"input.tsv");
    let mut parsedInput = parseString(inputStr);

    for num in parsedInput {
        println!("{}", num);
    }
}

fn readFile(filename: str) -> str {
    let mut file = File::open(filename).expect("file not found");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("something went wrong reading the file");

    return content;    
}

fn parseString(input: str) -> Vec<u32> {
    let mut split = input.split("\t");

    let mut nums = Vec::new();

    for s in split {
        nums.push(s.parse::<u32>().unwrap);
    }

    return nums;
}