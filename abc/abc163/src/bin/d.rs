use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut count = 0usize;
    let mut min = 0usize;
    let mut max = 0usize;
    for i in 0..=n {
        min += i;
        max += n - i;
        if i + 1 >= k {
            count += max - min + 1; // min..=max の全ての数を作れるはず
            count %= MOD;
        }
    }
    println!("{count}");
}
