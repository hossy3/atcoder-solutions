use std::io::{self, BufRead};

fn main() {
    // Reading the input
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    
    let first_line = iterator.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let n = first_line_iter.next().unwrap();
    let m = first_line_iter.next().unwrap();
    let k = first_line_iter.next().unwrap();
    
    let mut tests = Vec::new();
    
    for _ in 0..m {
        let line = iterator.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let c = parts.next().unwrap().parse::<usize>().unwrap();
        let mut test_keys = Vec::new();
        for _ in 0..c {
            test_keys.push(parts.next().unwrap().parse::<usize>().unwrap() - 1);
        }
        let result = parts.next().unwrap().chars().next().unwrap();
        tests.push((test_keys, result));
    }
    
    // Function to count the number of 1s in a bitmask
    let popcount = |x: u16| -> u32 { x.count_ones() };
    
    let mut valid_combinations = 0;
    
    // Iterate over all possible subsets of keys
    for mask in 0..(1 << n) {
        let mut valid = true;
        
        for (test_keys, result) in &tests {
            let real_count = test_keys.iter().filter(|&&key| (mask & (1 << key)) != 0).count();
            if (*result == 'o' && real_count < k) || (*result == 'x' && real_count >= k) {
                valid = false;
                break;
            }
        }
        
        if valid {
            valid_combinations += 1;
        }
    }
    
    // Print the result
    println!("{}", valid_combinations);
}