use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut uvc: [(Usize1, Usize1, usize); m],
    }

    uvc.sort_unstable_by_key(|&(_, _, c)| c);

    let mut dsu = Dsu::new(n);
    let mut result = 0;

    for (u, v, c) in uvc {
        if !dsu.same(u, v) {
            dsu.merge(u, v);
            result += c;
        }
    }

    if dsu.groups().len() == 1 {
        println!("{result}");
    } else {
        println!("-1");
    }
}
