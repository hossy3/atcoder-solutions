use proconio::{input, marker::Chars};

fn to_lower(c: char) -> char {
    (c as u8 + 'a' as u8 - 'A' as u8) as char
}

fn f(s: &[char], t: &[char]) -> bool {
    let c = to_lower(t[0]);
    let Some(i) = s.iter().position(|&x| x == c) else { 
        return false;
    };

    let c = to_lower(t[1]);
    let Some(j) = s[(i + 1)..].iter().position(|&x| x == c) else { 
        return false;
    };

    let c = to_lower(t[2]);
    if c != 'x' {
        let Some(_) = s[(i + j + 2)..].iter().position(|&x| x == c) else { 
            return false;
        };
    }

    true
}

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let yes = f(&s, &t);
    println!("{}", if yes { "Yes" } else { "No" });
}
