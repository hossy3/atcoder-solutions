use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i128; n],
    }

    a.sort_unstable();

    let mut result = 0;
    for (i, &x) in a.iter().enumerate() {
        result += x * (i as i128 - (n - i - 1) as i128);
    }
    println!("{result}");
}
