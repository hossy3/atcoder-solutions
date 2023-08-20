use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut v0 = vec![0usize; 27]; // a-z, @
    let mut v1 = vec![0usize; 27]; // a-z, @
    for &c in &s {
        if c == '@' {
            v0[26] += 1;
        } else {
            v0[c as usize - 'a' as usize] += 1;
        }
    }

    for &c in &t {
        if c == '@' {
            v1[26] += 1;
        } else {
            v1[c as usize - 'a' as usize] += 1;
        }
    }

    let cards = vec!['a', 't', 'c', 'o', 'd', 'e', 'r'];
    for &c in &cards {
        let i = c as usize - 'a' as usize;
        if v0[i] < v1[i] {
            let j = (v1[i] - v0[i]).min(v0[26]);
            v0[i] += j;
            v0[26] -= j;
        }
        if v0[i] > v1[i] {
            let j = (v0[i] - v1[i]).min(v1[26]);
            v1[i] += j;
            v1[26] -= j;
        }
    }
    let yes = (0..27).all(|i| v0[i] == v1[i]);
    println!("{}", if yes { "Yes" } else { "No" });
}
