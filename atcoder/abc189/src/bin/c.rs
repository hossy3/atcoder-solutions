use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut result = 0;
    for l in 0..n {
        let mut x = a[l];
        for r in l..n {
            x = x.min(a[r]);
            result = result.max((r - l + 1) * x);
        }
    }
    println!("{result}");
}
