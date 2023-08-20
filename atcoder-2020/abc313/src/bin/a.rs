use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let result = if n == 1 {
        0
    } else {
        p[0].max(p[1..].iter().max().unwrap() + 1usize) - p[0]
    };
    println!("{}", result);
}
