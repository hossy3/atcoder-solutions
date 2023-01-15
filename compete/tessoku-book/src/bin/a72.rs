use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c: [Chars; h],
    }

    let mut score = 0;
    for i in 0usize..(1 << h) {
        let ones = i.count_ones() as usize;
        if ones > k {
            continue;
        }

        let mut v = vec![0; w];
        for j in 0..h {
            let b = (i >> j) & 1 == 1;
            for k in 0..w {
                if b || c[j][k] == '#' {
                    v[k] += 1;
                }
            }
        }
        v.sort();

        for j in 0..(k - ones) {
            v[w - j - 1] = h;
        }
        score = score.max(v.iter().sum());
    }

    println!("{}", score);
}
