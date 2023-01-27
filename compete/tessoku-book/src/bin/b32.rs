use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize],
    }
    let mut grandy = vec![0; n + 1];
    for i in 1..=n {
        let mut b = vec![false; n + 1];
        for &a in &a {
            if i >= a {
                b[grandy[i - a]] = true;
            }
        }
        if let Some((j, _)) = b.iter().find_position(|&&b| !b) {
            grandy[i] = j;
        }
    }
    let yes = grandy[n] != 0;
    println!("{}", if yes { "First" } else { "Second" });
}
