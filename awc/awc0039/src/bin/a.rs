use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        _n: usize,
        m: usize,
        q: usize,
        pc: [(Usize1, char); m],
        t: [Chars; q],
    }

    for t in t {
        let yes = pc.iter().all(|&(p, c)| t[p] == c);
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
