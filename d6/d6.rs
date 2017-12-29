use std::fs::File;
use std::io::prelude::*;

fn main() {
    let input_str = read_file("input.tsv");
    let parsed_input = parse_string(input_str);

    let result = compute(parsed_input);

    println!("{}", result);
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


fn compute(nums: Vec<u32>) -> u32 {
    let mut prev_vecs: Vec<Vec<u32>> = Vec::new();
    prev_vecs.push(nums);
    let mut vec = nums;
    let mut iterations = 0;

    while exists(vec, &prev_vecs) {
        vec = balance(vec);
        prev_vecs.push(vec);
        iterations += 1;
    }

    return iterations;
}

fn balance(nums: Vec<u32>) -> Vec<u32> {
    let max_index = maximum_index(nums) as usize;
    let max_val = nums[max_index];
    nums[max_index] = 0;

    let dist_amount = max_val / ((nums.len() as u32) - 1);

    for i in 1..nums.len() {
        let index = (max_index + i) % nums.len();
        nums[index] += dist_amount;
    }

    nums[max_index] += max_val % ((nums.len() as u32) - 1);

    return nums;
}

fn exists(nums: Vec<u32>, prev_vecs: &Vec<Vec<u32>>) -> bool {
    let mut any_equal = false;
    for prev_vec in *prev_vecs {
        let mut vec_equal = true;
        for i in 0..nums.len() {
            vec_equal = vec_equal && (prev_vec[i] == nums[i]);
        }
        any_equal = any_equal || vec_equal;
    }

    return any_equal;
}

fn maximum_index(vec: Vec<u32>) -> usize {
    let mut max_val = 0;
    let mut max_index = 0;
    for i in 0..vec.len() {
        if vec[i] > max_val {
            max_val = vec[i];
            max_index = i;
        }
    }

    return max_index;
}