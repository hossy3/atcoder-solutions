use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    const N: usize = 200_000;
    let mut v = vec![-1; N + 1]; // おいしさ x を食べる人の番号
    let mut max = N + 1;
    for (i, &x) in a.iter().enumerate() {
        if x < max {
            for x in x..max {
                v[x] = (i + 1) as i64;
            }
            max = x;
        }
    }

    for x in b {
        println!("{}", v[x]);
    }
}
