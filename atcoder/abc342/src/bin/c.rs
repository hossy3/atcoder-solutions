use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
        q: usize,
        cd: [(char, char); q],
    }

    let mut dict = vec!['a'; 26];
    for i in 0..26 {
        dict[i] = ('a' as u8 + i as u8) as char;
    }

    for (c, d) in cd {
        for i in 0..26 {
            if dict[i] == c {
                dict[i] = d;
            }
        }
    }

    let result = s.iter().map(|&c| dict[c as usize - 'a' as usize]).join("");
    println!("{result}");
}
