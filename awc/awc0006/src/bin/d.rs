use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut lr: [(usize, usize); m],
    }

    lr.sort_unstable();
    let mut cur = 0;
    let mut cand = 0;
    let mut result = 0;
    for (l, r) in lr {
        if l > cur + 1 {
            if cand + 1 < l {
                println!("-1");
                return;
            }
            cur = cand;
            result += 1;
        }
        cand = cand.max(r);
    }
    if cand < n {
        println!("-1");
        return;
    }
    if cur < n {
        result += 1;
    }

    println!("{result}");
}
