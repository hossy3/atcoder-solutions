use proconio::input;

fn solve(p: isize, xc: &[(isize, usize)]) -> usize {
    let mut cand = 0;
    for (i, &(x, _)) in xc.iter().enumerate() {
        let score = p.abs_diff(x);
        let cand_score = p.abs_diff(xc[cand].0);
        if score < cand_score || (score == cand_score && x < xc[cand].0) {
            cand = i;
        }
    }
    cand
}

fn main() {
    input! {
        n: usize,
        p: isize,
        q: isize,
        xc: [(isize, usize); n],
    }
    let p0 = solve(p, &xc);
    let q0 = solve(q, &xc);
    let result = if p0 == q0 {
        xc[p0].1
    } else {
        xc[p0].1 + xc[q0].1
    } + 2;
    println!("{result}");
}
