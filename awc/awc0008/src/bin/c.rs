use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [isize; n],
    }

    let mut sum = a[0..k].iter().sum::<isize>();
    let mut count = 0;
    if sum <= 0 {
        count += 1;
    }

    for i in 0..(n - k) {
        sum += a[i + k] - a[i];
        if sum <= 0 {
            count += 1;
        }
    }
    println!("{count}");
}
