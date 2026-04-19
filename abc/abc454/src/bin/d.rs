use proconio::{input, marker::Chars};

fn f(a: &[char]) -> Vec<char> {
    let mut v = vec![];
    for &a in a {
        let l = v.len();
        if l > 2 && a == ')' && v[l - 3] == '(' && v[l - 2] == 'x' && v[l - 1] == 'x' {
            v.pop();
            v.pop();
            v.pop();
            v.push('x');
            v.push('x');
        } else {
            v.push(a);
        }
    }
    v
}

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            a: Chars,
            b: Chars,
        }
        let a = f(&a);
        let b = f(&b);
        let yes = a == b;
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
