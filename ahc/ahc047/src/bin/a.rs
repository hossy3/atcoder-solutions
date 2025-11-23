use std::{cmp::Reverse, collections::HashMap};

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn get_index(c: char) -> usize {
    c as usize - b'a' as usize
}

fn get_state(c: char, c_next: char) -> usize {
    (c as usize - b'a' as usize) * 2 + if is_type1(c_next) { 1 } else { 0 }
}

fn is_type1(c: char) -> bool {
    c == 'd' || c == 'e' || c == 'f'
}

fn make_initial_matrix(m: usize, factor: f64, sp: &[(Vec<char>, usize)]) -> Vec<Vec<usize>> {
    let mut mat = vec![vec![0usize; m]; m];

    // 最後の状態を初手状態としてどちらがお得かを調べる
    let mut v0 = vec![0usize; m];
    for (s, p) in sp {
        let j = get_state(s[0], s[1]);
        v0[j] += p;
    }

    // 出現数と得点から初期行列を作成する
    for (s, p) in sp {
        let len = s.len();
        let p0 = (*p as f64).powf(factor) as usize; // 高得点に多くヒットする重みづけ
        for (i, &c) in s[0..(len - 1)].iter().enumerate() {
            if i == len - 2 {
                let j = get_state(c, s[i + 1]);
                let k = get_index(s[i + 1]);
                if v0[k * 2] >= v0[k * 2 + 1] {
                    mat[j][k * 2] += p0;
                } else {
                    mat[j][k * 2 + 1] += p0;
                }
            } else {
                let j = get_state(c, s[i + 1]);
                let k = get_state(s[i + 1], s[i + 2]);
                mat[j][k] += p0;
            }
        }
    }

    // すべて合計が 100 になるように正規化する
    for i in 0..m {
        let sum: usize = mat[i].iter().sum();
        if sum > 0 {
            let mut v = vec![];
            for j in 0..m {
                let x = mat[i][j];
                let y = (x * 100 + sum - 1) / sum;
                let z = (x * 100 + sum - 1) % sum; // 捨てられる値
                v.push((Reverse(y), z, j));
                mat[i][j] = y;
            }
            v.sort();

            let sum0: usize = mat[i].iter().sum();
            assert!(100 <= sum0 && sum0 <= 100 + m);
            for &(_, _, j) in &v[0..(sum0 - 100)] {
                mat[i][j] -= 1;
            }
        } else {
            if i % 2 == 0 {
                mat[i] = vec![17, 0, 17, 0, 17, 0, 17, 0, 16, 0, 16, 0];
            } else {
                mat[i] = vec![0, 17, 0, 17, 0, 17, 0, 17, 0, 16, 0, 16];
            }
        }

        let sum: usize = mat[i].iter().sum();
        assert_eq!(sum, 100);
    }

    mat
}

fn output_matrix(mat: &[Vec<usize>]) {
    for (i, row) in mat.iter().enumerate() {
        let c = (b'a' + (i / 2) as u8) as char;
        println!("{} {}", c, row.iter().join(" "));
    }
}

fn compute_word_probability_sum(l: usize, sp: &[(Vec<char>, usize)], mat: &[Vec<usize>]) -> usize {
    let mut total_score = 0.0;
    for (word, score) in sp {
        let prob = compute_word_probability(
            word,
            l,
            &['a', 'a', 'b', 'b', 'c', 'c', 'd', 'd', 'e', 'e', 'f', 'f'],
            mat,
        );
        total_score += prob * *score as f64;
    }
    total_score.round() as usize
}

// 以下ほぼ Visualizer からいただいた関数

fn compute_word_probability(word: &Vec<char>, ll: usize, cc: &[char], aa: &[Vec<usize>]) -> f64 {
    let mm = cc.len();
    // Enumerate the automaton states (matching length, vertex)
    let mut n = 0;
    let mut states = HashMap::new();
    for j in 0..mm {
        states.insert((0, j), n);
        n += 1;
        for i in 0..word.len() - 1 {
            if word[i] == cc[j] {
                states.insert((i + 1, j), n);
                n += 1;
            }
        }
    }
    // Construct the state transition matrix
    let mut xx = vec![0.0; n * n];
    for (&(len, u), &j) in &states {
        for v in 0..mm {
            let mut next = word[..len].to_vec();
            next.push(cc[v]);
            let mut s = 0;
            while next[s..] != word[..next.len() - s] {
                s += 1;
            }
            if next.len() - s != word.len() {
                let i = states[&(next.len() - s, v)];
                xx[i * n + j] += aa[u][v] as f64 / 100.0;
            }
        }
    }
    // Compute Y=X^(L-1)
    let mut power = ll - 1;
    let mut yy = vec![0.0; n * n];
    for i in 0..n {
        yy[i * n + i] = 1.0;
    }
    while power > 0 {
        if power & 1 != 0 {
            yy = mul(&yy, &xx, n);
        }
        xx = mul(&xx, &xx, n);
        power >>= 1;
    }
    // Compute the probability
    let init = if cc[0] == word[0] {
        states[&(1, 0)]
    } else {
        states[&(0, 0)]
    };
    let mut ret = 1.0;
    for i in 0..n {
        ret -= yy[i * n + init];
    }
    ret.clamp(0.0, 1.0)
}

fn mul(a: &Vec<f64>, b: &Vec<f64>, n: usize) -> Vec<f64> {
    let mut c = vec![0.0; n * n];
    for i in 0..n {
        for k in 0..n {
            for j in 0..n {
                c[i * n + j] += a[i * n + k] * b[k * n + j];
            }
        }
    }
    c
}

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        sp: [(Chars, usize); n],
    }

    let mut max_score = 0;
    let mut max_mat = vec![vec![]; m];
    for i in 0..=110 {
        let factor = 0.7 + i as f64 * 0.02;
        let mat = make_initial_matrix(m, factor, &sp);
        let score = compute_word_probability_sum(l, &sp, &mat);
        if score > max_score {
            max_score = score;
            max_mat = mat;
        }
    }
    output_matrix(&max_mat);
    eprintln!("{max_score}");
}
