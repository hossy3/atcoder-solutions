use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: [usize; n],
        ab: [(Usize1, Usize1); q],
    }

    for (a, b) in ab {
        let yes = s[a] > s[b];
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
