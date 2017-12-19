use std::fs::File;
use std::io::prelude::*;

fn main() {
    let input_str = read_file("input.tsv");
    let parsed_input = parse_string(input_str);

    for num in parsed_input {
        println!("{}", num);
    }
}

fn read_file(filename: &'static str) -> String {
    let mut file = File::open(filename).expect("file not found");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("something went wrong reading the file");

    return content;    
}

fn parse_string(input: String) -> Vec<u32> {
    let split = input.split("\t");

    let mut nums = Vec::new();

    for s in split {
        nums.push(s.parse::<u32>().unwrap());
    }

    return nums;
}


// COMPUTING

let mut prev_vecs: Vec<Vec<u32>>;

fn compute(nums: Vec<u32>) -> u32

fn balance(nums: Vec<u32>) -> Vec<u32> {
    let max_index = maximum_index(nums);

    for (i, num) in nums.enumerate() {
        let index = max_index + i;
        nums[]
    }
}

fn maximum_index(vec: Vec<u32>) -> u32 {
    let mut max_val = 0;
    let mut max_index = 0;
    for (i, val) in vec.enumerate() {
        if (val > max_val) {
            max_val = val;
            max_index = i;
        }
    }

    return max_index;
}