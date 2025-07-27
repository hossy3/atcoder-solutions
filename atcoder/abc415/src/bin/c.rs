use proconio::{input, marker::Chars};

fn f(k: usize, s: Vec<char>) -> bool {
    let n0 = 1usize << k;
    let mut v = vec![false; n0];
    v[0] = true;
    for (i, &c) in s.iter().enumerate() {
        if c == '1' {
            v[i + 1] = true;
        }
    }

    let mut stack = vec![];
    for i in 0..k {
        let i = 1 << i;
        if !v[i] {
            if i == n0 - 1 {
                return true;
            }
            v[i] = true;
            stack.push(i);
        }
    }

    while let Some(i) = stack.pop() {
        for j in 0..k {
            let j0 = 1usize << j;
            let i = i | j0;
            if !v[i] {
                if i == n0 - 1 {
                    return true;
                }
                v[i] = true;
                stack.push(i);
            }
        }
    }

    false
}

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
        }
        let yes = f(n, s);
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
