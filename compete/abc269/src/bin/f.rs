use proconio::input;

fn main() {
    input! {
        _: usize,
        m: i128,
        q: usize,
        abcd: [(i128, i128, i128, i128); q],
    }
    const MOD: i128 = 998_244_353;
    for &(a, b, c, d) in &abcd {
        let mut score = 0;

        // odd
        let i0 = if a % 2 == 0 { a + 1 } else { a };
        let i1 = if b % 2 == 0 { b - 1 } else { b };
        let j0 = if c % 2 == 0 { c + 1 } else { c };
        let j1 = if d % 2 == 0 { d - 1 } else { d };
        let h = (i1 - i0) / 2 + 1;
        let w = (j1 - j0) / 2 + 1;
        if w > 0 && h > 0 {
            score += (w * (w - 1)) * h + h * (h - 1) * m * w + w * h * ((i0 - 1) * m + j0);
            score %= MOD;
        }

        // even
        let i0 = if a % 2 == 0 { a } else { a + 1 };
        let i1 = if b % 2 == 0 { b } else { b - 1 };
        let j0 = if c % 2 == 0 { c } else { c + 1 };
        let j1 = if d % 2 == 0 { d } else { d - 1 };
        let h = (i1 - i0) / 2 + 1;
        let w = (j1 - j0) / 2 + 1;
        if w > 0 && h > 0 {
            score += (w * (w - 1)) * h + h * (h - 1) * m * w + w * h * ((i0 - 1) * m + j0);
            score %= MOD;
        }

        println!("{}", score);
    }
}
