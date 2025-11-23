use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        s: [Chars; h],
    }

    let mut result = 0usize;
    for i0 in 0..(h * w - 1) {
        let (h0, w0) = (i0 / w, i0 % w);
        if s[h0][w0] == '#' {
            continue;
        }
        for i1 in (i0 + 1)..(h * w) {
            let (h1, w1) = (i1 / w, i1 % w);
            if s[h1][w1] == '#' {
                continue;
            }

            let mut count = 0usize;
            for i2 in 0..(h * w) {
                let (h2, w2) = (i2 / w, i2 % w);
                if s[h2][w2] == '#' {
                    continue;
                }
                if h2.abs_diff(h0) + w2.abs_diff(w0) <= d || h2.abs_diff(h1) + w2.abs_diff(w1) <= d
                {
                    count += 1;
                }
            }
            result = result.max(count);
        }
    }

    println!("{result}");
}
