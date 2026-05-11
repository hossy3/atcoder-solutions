use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        y: usize,
        a: [usize; n],
    }

    let mut rest = k;
    let mut result = 0usize;
    for &a in a.iter().rev() {
        if a > y + l {
            result += 1;
        } else if a > l && rest > 0 {
            result += 1;
            rest -= 1;
        }
    }
    println!("{result}");
}
