use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ps: [(Usize1, String); m],
    }

    let mut v = vec![(false, 0usize); n]; // (正解済みか, それまでのWA数)
    for (p, s) in ps {
        if v[p].0 {
            continue;
        }
        match s.as_str() {
            "AC" => {
                v[p].0 = true;
            }
            "WA" => {
                v[p].1 += 1;
            }
            _ => unreachable!(),
        }
    }

    let v: Vec<_> = v.iter().filter(|&&(b, _)| b).map(|&(_, wa)| wa).collect();
    let correct_answers = v.len();
    let penalties = v.iter().sum::<usize>();
    println!("{correct_answers} {penalties}");
}
