use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
        t: [Chars; m],
    }

    let mut results = vec![0usize; n];
    'outer: for t in t {
        let mut cand = vec![];
        for (i, s) in s.iter().enumerate() {
            if s.len() != t.len() {
                continue;
            }
            let count = (0..(s.len())).filter(|&i| s[i] != t[i]).count();
            if count == 0 {
                results[i] += 1;
                continue 'outer;
            } else if count == 1 {
                cand.push(i);
            }
        }
        if cand.len() == 1 {
            results[cand[0]] += 1;
        }
    }

    for result in results {
        println!("{result}");
    }
}
