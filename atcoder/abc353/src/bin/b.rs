use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        a: [usize; n],
    }

    let mut count = 0;
    let mut cur = 0;
    for x in a {
        if cur + x <= k {
            cur += x;
        } else {
            count += 1;
            cur = x;
        }
    }
    count += 1;
    println!("{count}");
}
