use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
    }

    let mut dsu = Dsu::new(n);
    for &(a, b) in &ab {
        dsu.merge(a, b);
    }

    let mut v = vec![0usize; n];
    for i in 0..n {
        let leader = dsu.leader(i);
        v[leader] += 1;
    }
    eprintln!("{:?}", &v);

    let result = v.iter().map(|&x| x * (x.max(1) - 1) / 2).sum::<usize>() - m;
    println!("{result}");
}
