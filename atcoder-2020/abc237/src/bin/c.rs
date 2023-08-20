use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let f = || {
        let l = s.iter().position(|&x| x != 'a');
        if l == None {
            return true;
        }
        let l = l.unwrap();
        let r = s.iter().rposition(|&x| x != 'a').unwrap();
        if l > s.len() - r - 1 {
            return false;
        }
        let mut l = l;
        let mut r = r;
        while l < r {
            if s[l] != s[r] {
                return false;
            }
            l += 1;
            r -= 1
        }
        true
    };
    let yes = f();
    println!("{}", if yes { "Yes" } else { "No" });
}
