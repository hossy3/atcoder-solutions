use std::io;

fn main() {
    // Read the input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let inputs: Vec<usize> = input.split_whitespace()
                                  .map(|x| x.parse().expect("Not a number"))
                                  .collect();
    let (n, l, r) = (inputs[0], inputs[1], inputs[2]);

    // Create the sequence A = [1, 2, ..., N]
    let mut sequence: Vec<usize> = (1..=n).collect();

    // Reverse the subarray from L-1 to R-1
    sequence[l-1..r].reverse();

    // Print the resulting sequence
    for num in sequence {
        print!("{} ", num);
    }
    println!();
}
