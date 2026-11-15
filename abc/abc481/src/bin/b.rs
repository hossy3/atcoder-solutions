use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut result = 0usize;
    for a in a.windows(2) {
        if (a[0] < k && a[1] > k) || (a[0] > k && a[1] < k) {
            result += 1;
        }
    }
    for &x in &a {
        if x == k {
            result += 1;
        }
    }
    println!("{result}");
}
