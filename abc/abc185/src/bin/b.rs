use proconio::input;

fn f(n: usize, t: usize, ab: &[(usize, usize)]) -> bool {
    let mut i = 0;
    let mut cur = n;
    for &(a, b) in ab {
        if cur <= a - i {
            return false;
        }
        cur = (cur - (a - i) + (b - a)).min(n);
        i = b;
    }
    if cur <= t - i {
        return false;
    }
    true
}

fn main() {
    input! {
        n: usize,
        m: usize,
        t: usize,
        ab: [(usize, usize); m],
    }
    let yes = f(n, t, &ab);
    println!("{}", if yes { "Yes" } else { "No" });
}
