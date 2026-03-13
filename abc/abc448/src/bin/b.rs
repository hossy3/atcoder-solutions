use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [usize; m],
        ab: [(Usize1, usize); n],
    }

    let mut c0 = vec![0usize; m];
    for (a, b) in ab {
        c0[a] += b;
    }

    let result: usize = (0..m).map(|i| c[i].min(c0[i])).sum();
    println!("{result}");
}
