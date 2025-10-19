use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        b: [usize; q],
    }

    a.sort();
    let mut cum = vec![0; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + a[i];
    }
    for b in b {
        if a[n - 1] < b {
            println!("-1");
        } else {
            let i = a.partition_point(|&x| x < b);
            let result = cum[i] + (n - i) * (b - 1) + 1;
            println!("{result}");
        }
    }
}
