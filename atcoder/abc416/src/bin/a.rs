use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        _n: usize,
        l: Usize1,
        r: Usize1,
        s: Chars,
    }
    let yes = (l..=r).all(|i| s[i] == 'o');
    println!("{}", if yes { "Yes" } else { "No" });
}
