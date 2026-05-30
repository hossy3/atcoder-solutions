use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut result = 0usize;
    let mut cur = 0usize;
    for a in a {
        if cur <= a {
            result += a - cur;
        }
        cur = a;
    }
    println!("{result}");
}
