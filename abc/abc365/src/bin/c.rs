use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }

    a.sort();
    let mut cum = vec![0usize; n + 1];
    for (i, &x) in a.iter().enumerate() {
        cum[i + 1] = cum[i] + x;
    }
    if cum[n] <= m {
        println!("infinite");
        return;
    }

    for (i, &x) in a.iter().enumerate() {
        if cum[i] + (n - i) * x >= m {
            let result = (m - cum[i]) / (n - i);
            println!("{result}");
            return;
        }
    }
}
