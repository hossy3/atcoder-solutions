use proconio::{input, marker::Chars};

fn f(s: &[char]) -> bool {
    if s[0] == '1' {
        return false;
    }
    let a = [
        s[6] == '1',
        s[3] == '1',
        s[1] == '1' || s[7] == '1',
        s[4] == '1',
        s[2] == '1' || s[8] == '1',
        s[5] == '1',
        s[9] == '1',
    ];
    let b = (a[0] && !a[1] && (a[2] || a[3] || a[4] || a[5] || a[6]))
        || ((a[0] || a[1]) && !a[2] && (a[3] || a[4] || a[5] || a[6]))
        || ((a[0] || a[1] || a[2]) && !a[3] && (a[4] || a[5] || a[6]))
        || ((a[0] || a[1] || a[2] || a[3]) && !a[4] && (a[5] || a[6]))
        || ((a[0] || a[1] || a[2] || a[3] || a[4]) && !a[5] && a[6]);
    b
}

fn main() {
    input! {
        s: Chars,
    }
    let yes = f(&s);
    println!("{}", if yes { "Yes" } else { "No" });
}
