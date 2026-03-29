use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        t: usize,
        c: usize,
        a: [usize; n],
    }

    let mut cum = vec![0usize; n + 1]; // まいた水の総和
    for (i, &x) in a.iter().enumerate() {
        let y = t.saturating_sub(x + (cum[i] - cum[i.saturating_sub(k - 1)]));
        cum[i + 1] = cum[i] + y;
        // eprintln!("{:?}", &cum);
    }

    let result = cum[n] * c;
    println!("{result}");
}
