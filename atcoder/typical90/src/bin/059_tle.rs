use proconio::{input, marker::Usize1};

fn f(n: usize, xy: &[(usize, usize)], (a, b): (usize, usize)) -> bool {
    let mut v = vec![false; n];
    v[a] = true;
    for &(x, y) in xy {
        v[y] |= v[x];
    }
    v[b]
}

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        mut xy: [(Usize1, Usize1); m],
        ab: [(Usize1, Usize1); q],
    }
    xy.sort();

    for (a, b) in ab {
        let yes = f(n, &xy, (a, b));
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
