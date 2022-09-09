use proconio::{input, marker::Usize1};

fn init_cv(w: usize, lr: &[(usize, usize)]) -> (Vec<usize>, usize) {
    let mut v = Vec::with_capacity(lr.len() * 2 + 2);
    v.push(0);
    v.push(w);
    for &(l, r) in lr {
        v.push(l);
        v.push(r + 1);
    }
    v.sort();
    v.dedup();

    let mut cv = vec![0; w + 1];
    for (i, &x) in v.iter().enumerate() {
        cv[x] = i;
    }
    
    (cv, v.len())
}

fn main() {
    input! {
        w: usize,
        lr: [(Usize1, Usize1)],
    }

    let (cv, len) = init_cv(w, &lr);
    let mut v = vec![0usize; len];

    for &(l, r) in &lr {
        let l0 = cv[l];
        let r0 = cv[r + 1];
        let h_max = v[l0..r0].iter().fold(0, |acc, &x| acc.max(x)) + 1;
        for x in v[l0..r0].iter_mut() {
            *x = h_max;
        }
        println!("{}", h_max);
    }
}
