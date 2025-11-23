use proconio::input;

fn main() {
    input! {
        n: usize,
        w: i64,
        stp: [(usize, usize, i64); n],
    }
    let t_max = stp.iter().map(|&(_, t, _)| t).max().unwrap();
    let mut acc = vec![0; t_max + 1];
    for &(s, t, p) in &stp {
        acc[s] += p;
        acc[t] -= p;
    }
    for i in 0..t_max {
        acc[i + 1] += acc[i];
    }
    let yes = acc.iter().all(|&x| x <= w);
    println!("{}", if yes { "Yes" } else { "No" });
}
