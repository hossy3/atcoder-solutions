use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: [usize; n],
    }
    let y = a + b;
    let result = c.iter().position(|&x| x == y).unwrap() + 1;
    println!("{}", result);
}
