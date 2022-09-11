use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        w: usize,
        lr: [(Usize1, Usize1)],
    }

    let mut v = Vec::with_capacity(w + 1);
    v.push((0usize, 0usize));
    v.push((w, 0));

    for &(l, r) in &lr {
        let l0 = v.lower_bound_by(|x| x.0.cmp(&l));
        let r0 = v.lower_bound_by(|x| x.0.cmp(&(r + 1)));

        let l1 = if v[l0].0 == l { l0 } else { l0 - 1 };
        let r1 = if r0 < v.len() && v[r0].0 == r + 1 {
            r0
        } else {
            r0 - 1
        };

        let height = v[l1..r0].iter().fold(0, |acc, x| acc.max(x.1)) + 1;
        let h_r1 = v[r1].1;

        let mut ins = Vec::with_capacity(2);
        if l0 == 0 || v[l0 - 1].1 != height {
            ins.push((l, height));
        }
        if h_r1 != height {
            ins.push((r + 1, h_r1));
        }
        v.splice(l0..=r1, ins);

        println!("{}", height);
    }
}
