use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut si = Vec::with_capacity(n);
    for (i, s) in s.iter().enumerate() {
        si.push((s, i));
    }
    si.sort();

    let mut v = vec![0; n];
    for i in 0..(n - 1) {
        let mut j = 0;
        let (s0, s1) = (&si[i].0, &si[i + 1].0);
        while j < s0.len() && j < s1.len() && s0[j] == s1[j] {
            j += 1;
        }

        let (i0, i1) = (si[i].1, si[i + 1].1);
        v[i0] = v[i0].max(j);
        v[i1] = v[i1].max(j);
    }

    for &x in &v {
        println!("{}", x);
    }
}
