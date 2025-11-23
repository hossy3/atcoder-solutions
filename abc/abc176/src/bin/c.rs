use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut result = 0usize;
    let mut cur = 0usize;
    for x in a {
        if cur >= x {
            result += cur - x;
        } else {
            cur = x;
        }
    }
    println!("{result}");
}
