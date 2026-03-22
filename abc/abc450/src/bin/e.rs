use proconio::{input, marker::Chars};

const ALPHABET_SIZE: usize = 26;

fn ctoi(c: char) -> usize {
    c as usize - 'a' as usize
}

fn f(x: &[char]) -> [usize; ALPHABET_SIZE] {
    let mut counts = [0; ALPHABET_SIZE];
    for &c in x {
        counts[ctoi(c)] += 1;
    }
    counts
}

fn f_cum(xy: &[char]) -> Vec<[usize; ALPHABET_SIZE]> {
    let mut counts = vec![[0; ALPHABET_SIZE]; xy.len() + 1];
    for (i, &c) in xy.iter().enumerate() {
        counts[i + 1][ctoi(c)] = 1;
    }
    for i in 0..xy.len() {
        for j in 0..ALPHABET_SIZE {
            counts[i + 1][j] += counts[i][j];
        }
    }
    counts
}

// rest 個取る
fn g(
    v: &[(usize, [usize; ALPHABET_SIZE])],
    c: char,
    rest: usize,
    cum: &[[usize; ALPHABET_SIZE]],
) -> usize {
    let i = ctoi(c);
    if rest > 0 {
        let j = v.partition_point(|&(len, _)| len <= rest);
        if j >= 3 {
            return v[j - 1].1[i] + g(v, c, rest - v[j - 1].0, cum);
        }
    }
    cum[rest][i]
}

fn main() {
    input! {
        x: Chars,
        y: Chars,
        q: usize,
        lrc: [(usize, usize, char); q],
    }

    // x, y それぞれに含まれている文字数を調べる
    let x0 = f(&x);
    let y0 = f(&y);

    let xy: Vec<_> = y.iter().chain(x.iter()).copied().collect();
    let cum = f_cum(&xy);

    // フィボナッチ数列のようなものを組み立てる
    let mut v = vec![(x.len(), x0.clone()), (y.len(), y0.clone())];
    while v[v.len() - 1].0 <= 10usize.pow(18) + 1 {
        let n = v.len();
        v.push((v[n - 1].0 + v[n - 2].0, v[n - 1].1.clone()));
        for i in 0..ALPHABET_SIZE {
            (v[n].1)[i] += (v[n - 2].1)[i];
        }
    }

    // eprintln!("{:?}", &cum);
    // eprintln!("{:?}", &v);

    for &(l, r, c) in &lrc {
        let result = g(&v, c, r, &cum) - g(&v, c, l - 1, &cum);
        println!("{result}");
    }
}
