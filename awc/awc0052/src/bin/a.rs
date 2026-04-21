use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut max = 1usize;
    let mut cur = 1usize;
    for i in 1..n {
        if a[i] > a[i - 1] {
            cur += 1;
            max = max.max(cur);
        } else {
            cur = 1;
        }
    }
    println!("{max}");
}
