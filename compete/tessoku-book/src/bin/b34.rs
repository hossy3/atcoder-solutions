use proconio::input;

// grundy number

fn main() {
    input! {
        n: usize,
        _: usize,
        _: usize,
        a: [usize; n],
    }
    let grundy = [0, 0, 1, 1, 2];
    let result = a.iter().fold(0, |acc, a| acc ^ grundy[*a % 5]);
    let first = result != 0;
    println!("{}", if first { "First" } else { "Second" });
}
