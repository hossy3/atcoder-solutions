use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut s: Chars,
        xc: [(Usize1, char); q],
    }

    let mut v = vec![false; n - 2];
    let mut result = 0;
    for i in 0..(n - 2) {
        if s[i..(i + 3)] == ['A', 'B', 'C'] {
            v[i] = true;
            result += 1;
        }
    }

    for (x, c) in xc {
        let x0 = x.saturating_sub(2);
        let x1 = x.min(n - 3);
        s[x] = c;
        for i in x0..=x1 {
            if v[i] {
                v[i] = false;
                result -= 1;
            }
            if s[i..(i + 3)] == ['A', 'B', 'C'] {
                v[i] = true;
                result += 1;
            }
        }

        println!("{result}");
    }
}
