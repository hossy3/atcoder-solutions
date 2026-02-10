use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        p: [usize; n],
    }
    let mut cand = usize::MAX;
    for (i, &x) in p.iter().enumerate() {
        if l <= x && x <= r {
            if cand == usize::MAX || p[cand] < x {
                cand = i;
            }
        }
    }
    if cand == usize::MAX {
        println!("-1");
    } else {
        println!("{}", cand + 1);
    }
}
