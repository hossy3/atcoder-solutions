use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        hp: [(usize, usize); n],
    }

    let mut a = vec![vec![]; 2];
    for (h, p) in hp {
        a[h].push(p);
    }
    a[0].sort_unstable();
    a[1].sort_unstable();
    if a[0].len() < k - m || a[1].len() < m {
        println!("-1");
        return;
    }

    let result = a[0][(a[0].len() - (k - m))..].iter().sum::<usize>()
        + a[1][(a[1].len() - m)..].iter().sum::<usize>();
    println!("{result}");
}
