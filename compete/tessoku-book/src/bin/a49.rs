use proconio::{input, marker::Usize1};

// heuristic - greedy

fn main() {
    input! {
        pqr: [(Usize1, Usize1, Usize1)]
    }

    let mut x = vec![0i64; 20];
    let mut v = Vec::with_capacity(pqr.len());
    for &(p, q, r) in &pqr {
        let a = (x[p] + 1).abs() + (x[q] + 1).abs() + (x[r] + 1).abs();
        let b = (x[p] - 1).abs() + (x[q] - 1).abs() + (x[r] - 1).abs();
        let c = if a <= b { 1 } else { -1 };
        v.push(c);
        x[p] += c;
        x[q] += c;
        x[r] += c;
    }

    for &v in &v {
        println!("{}", if v == 1 { "A" } else { "B" });
    }
}
