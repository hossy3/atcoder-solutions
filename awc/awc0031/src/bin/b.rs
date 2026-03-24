use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        mut s: [[usize; n]; t],
    }

    let mut count = 0usize;
    for j in 0..t {
        s[j].sort();
        if s[j][n - 1] >= s[j][n - 2] * 2 {
            count += 1;
        }
    }
    println!("{count}");
}
