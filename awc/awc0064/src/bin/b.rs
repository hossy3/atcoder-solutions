use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
    }
    a.sort_unstable();
    let mut result = 0isize;
    for (i, &a) in a.iter().enumerate() {
        if (i + n) % 2 == 0 {
            result -= a;
        } else {
            result += a;
        }
    }
    println!("{result}");
}
