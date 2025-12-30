use proconio::input;

fn f(wp: &[(isize, isize)]) -> usize {
    let n = wp.len();
    let mut v = vec![0; n];
    for (i, &(w, p)) in wp.iter().enumerate() {
        let mut v0 = vec![0; n + 1];
        for i in 0..=i {
            v0[i] = v0[i].max(v[i] + p);
            v0[i + 1] = v[i] - w;
        }
        v = v0;
    }
    let result = (0..=n).rfind(|&i| v[i] >= 0).unwrap_or(0);
    result
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            wp: [(isize, isize); n],
        }
        let result = f(&wp);
        println!("{result}");
    }
}
