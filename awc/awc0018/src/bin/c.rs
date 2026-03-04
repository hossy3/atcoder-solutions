use proconio::{input, marker::Chars};

fn c2i(c: char) -> usize {
    c as usize - 'a' as usize
}

fn main() {
    input! {
        n: usize,
        q: usize,
        s: [Chars; n],
        ab: [(char, char); q],
    }

    let mut table = vec!['a'; 26];
    for c in 'a'..='z' {
        table[c2i(c)] = c;
    }

    for (a, b) in ab {
        for c in 'a'..='z' {
            if table[c2i(c)] == a {
                table[c2i(c)] = b;
            }
        }
    }

    for s in s {
        for c in s {
            print!("{}", table[c2i(c)]);
        }
        println!();
    }
}
