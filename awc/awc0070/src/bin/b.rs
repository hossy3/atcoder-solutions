use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut count = 0;
    let mut sum = 0;
    for (i, &a) in a.iter().enumerate() {
        if a * i >= sum {
            count += 1;
        }
        sum += a;
    }
    println!("{count}");
}
