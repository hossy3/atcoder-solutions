use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut v: Vec<(_, _)> = vec![];
    for &c in &s {
        if v.len() > 0 && v[v.len() - 1].0 == c {
            let i = v.len() - 1;
            v[i].1 += 1;
        } else {
            v.push((c, 1));
        }
    }
    let mut result = 0;
    for v in v.windows(2) {
        if v[0].0 as u8 + 1 == v[1].0 as u8 {
            result += v[0].1.min(v[1].1);
        }
    }
    println!("{result}");
}
