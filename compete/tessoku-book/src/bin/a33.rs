use proconio::input;

// nim sum

fn main() {
    input! {
        a: [usize],
    }
    let nim = a.iter().fold(0, |acc, x| acc ^ x);
    let first = nim != 0;
    println!("{}", if first { "First" } else { "Second" });
}
