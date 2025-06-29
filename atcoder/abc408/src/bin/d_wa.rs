use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
        }

        let mut v = vec![];
        for i in 0..n {
            if s[i] == '1' {
                if i == 0 || s[i - 1] == '0' {
                    v.push(1usize);
                } else {
                    *v.last_mut().unwrap() += 1;
                }
            }
        }

        v.sort();
        let result: usize = if v.len() > 1 {
            v[..(v.len() - 1)].iter().sum()
        } else {
            0
        };
        println!("{result}");
    }
}
