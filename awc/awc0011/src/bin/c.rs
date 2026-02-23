use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut result: usize = 0;
    let mut cur = 0;
    for &x in &a {
        if x | k == k {
            result += 1;
            cur |= x;
        }
    }
    if cur == k && result > 0 {
        println!("{result}");
    } else {
        println!("-1");
    }
}
