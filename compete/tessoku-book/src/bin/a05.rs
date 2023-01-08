use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut count = 0;
    for i in 1..=n {
        for j in 1..=n {
            if i + j + 1 <= k && k <= i + j + n {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
