use proconio::{input, marker::Chars};

fn f(c: char) -> usize {
    c as usize - 'a' as usize
}

fn main() {
    input! {
        _n: usize,
        _m: usize,
        s: Chars,
        t: Chars,
        q: usize,
        w: [Chars; q],
    }

    let mut s0 = vec![false; 26];
    for &c in &s {
        let i = f(c);
        s0[i] = true;
    }

    let mut t0 = vec![false; 26];
    for &c in &t {
        let i = f(c);
        t0[i] = true;
    }

    for w in w {
        let mut bs = true;
        let mut bt = true;
        for c in w {
            let i = f(c);
            if !s0[i] {
                bs = false;
            }
            if !t0[i] {
                bt = false;
            }
        }
        if bs && bt {
            println!("Unknown");
        } else if bs {
            println!("Takahashi");
        } else if bt {
            println!("Aoki");
        } else {
            unreachable!();
        }
    }
}
