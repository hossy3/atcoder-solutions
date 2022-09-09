use proconio::input;
use proconio::marker::Usize1;

fn prev_lower_bound(vi: &[usize], i: usize) -> usize {
    let mut j = 0;
    while i > vi[j] {
        j = vi[j];
    }
    j
}

fn max_height(vh: &[usize], vi: &[usize], l: usize, r: usize) -> usize {
    let mut h_max = vh[l];
    let mut j = vi[l];
    while j <= r {
        h_max = h_max.max(vh[j]);
        j = vi[j];
    }
    h_max
}

fn main() {
    input! {
        w: usize,
        lr: [(Usize1, Usize1)],
    }

    let mut vh = vec![0usize; w + 1]; // vec<height>
    let mut vi = vec![0usize; w + 1]; // vec<next_index>
    vi[0] = w;

    for &(l, r) in &lr {
        let l0 = prev_lower_bound(&vi, l + 1);
        let r0 = prev_lower_bound(&vi, r + 1);
        let r1 = vi[r0];

        let h_new = max_height(&vh, &vi, l0, r0) + 1;
        vi[l0] = l;
        if r1 == r + 1 {
            vi[l] = r1;
        } else {
            vi[l] = r + 1;
            vi[r + 1] = r1;
            vh[r + 1] = vh[r0];
        }
        vh[l] = h_new;
        println!("{}", h_new);
    }
}
