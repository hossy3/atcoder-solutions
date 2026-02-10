use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut count = 0;
    let mut sum = 0;
    for b in b {
        let x = a[b - 1];
        if x < k {
            count += 1;
            sum += x;
        }
    }
    println!("{count} {sum}");
}
