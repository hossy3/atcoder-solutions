use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        t: [usize; m],
        s: [[usize; m]; n],
    }

    // 基準通過者の一覧
    let mut v = vec![];
    for (i, s) in s.iter().enumerate() {
        if std::iter::zip(s, &t).all(|(&s, &t)| s >= t) {
            v.push((s.iter().sum::<usize>(), i));
        }
    }

    if v.len() > k {
        v.sort_unstable();
        v.reverse();
        let x = v[k - 1].0;
        while let Some(&(x0, _)) = v.last() {
            if x0 >= x {
                break;
            }
            v.pop();
        }
        v.sort_by_key(|&(_, i)| i);
    }

    for (_, i) in v {
        println!("{}", i + 1);
    }
}
