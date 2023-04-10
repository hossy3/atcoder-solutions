use proconio::input;

fn grundy(x: usize, l: usize, r: usize) -> usize {
    (x % (l + r)) / l
}

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        a: [usize; n],
    }
    let result = a.iter().fold(0, |acc, a| acc ^ grundy(*a, l, r));
    let first = result != 0;
    println!("{}", if first { "First" } else { "Second" });
}
