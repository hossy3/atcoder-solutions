use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        w: usize,
        lr: [(usize, usize)],
    }

    let mut v = Vec::with_capacity(w + 1);
    v.push((1usize, 0usize));
    v.push((w + 1, 0));

    for &(l, r) in &lr {
        let l0 = v.lower_bound_by(|x| x.0.cmp(&l));
        let r0 = v.lower_bound_by(|x| x.0.cmp(&(r + 1)));

        let l1 = if v[l0].0 == l { l0 } else { l0 - 1 };
        let r1 = if r0 < v.len() && v[r0].0 == r + 1 {
            r0
        } else {
            r0 - 1
        };

        let h_new = v[l1..r0].iter().fold(0, |acc, x| acc.max(x.1)) + 1;
        let h_r1 = v[r1].1;
        v.splice(l0..=r1, vec![(l, h_new), (r + 1, h_r1)]);

        println!("{}", h_new);
    }
}
