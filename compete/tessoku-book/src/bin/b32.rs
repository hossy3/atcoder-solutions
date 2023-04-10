use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize],
    }
    let mut grundy = vec![0; n + 1];
    for i in 1..=n {
        let mut b = vec![false; n + 1];
        for &a in &a {
            if i >= a {
                b[grundy[i - a]] = true;
            }
        }
        if let Some((j, _)) = b.iter().find_position(|&&b| !b) {
            grundy[i] = j;
        }
    }
    let yes = grundy[n] != 0;
    println!("{}", if yes { "First" } else { "Second" });
}
