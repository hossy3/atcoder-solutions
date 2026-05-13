use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut cur = 0;
    let mut result = 0;
    for &a in &a {
        if a > cur {
            cur = a + cur / 2;
        } else if a < cur {
            cur = 0;
            result += 1;
        } else {
            cur = a;
            result += 1;
        }
    }
    if cur > 0 {
        result += 1;
    }
    println!("{result}");
}
