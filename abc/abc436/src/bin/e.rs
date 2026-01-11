use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    }

    let mut dsu = Dsu::new(n);
    for (i, &x) in p.iter().enumerate() {
        dsu.merge(i, x);
    }
    let mut result = 0;
    for groups in dsu.groups() {
        let len = groups.len();
        result += len * (len - 1) / 2;
    }
    println!("{result}");
}
