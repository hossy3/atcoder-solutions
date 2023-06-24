use proconio::{input, marker::Chars};

fn f(n: usize, s: &[Vec<char>]) -> bool {
    for i in 0..n {
        'outer: for j in 0..n {
            if i == j {
                continue;
            }
            let l0 = s[i].len();
            let l1 = s[j].len();
            let l = l0 + l1;
            for i0 in 0..(l / 2) {
                let i1 = l - i0 - 1;
                let c0 = if i0 < l0 { s[i][i0] } else { s[j][i0 - l0] };
                let c1 = if i1 < l0 { s[i][i1] } else { s[j][i1 - l0] };
                if c0 != c1 {
                    continue 'outer;
                }
            }
            return true;
        }
    }
    false
}

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let yes = f(n, &s);
    println!("{}", if yes { "Yes" } else { "No" });
}
