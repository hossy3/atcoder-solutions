use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut lr: [(usize, usize); m],
    }

    if n < m {
        println!("No"); // 駐車スペースが足りない
        return;
    }

    let mut set = std::collections::BTreeSet::new();
    for i in 1..=n {
        set.insert(i);
    }

    lr.sort_by_key(|&(l, r)| (r, l));
    for (l, r) in lr {
        if let Some(&x) = set.range(l..=r).next() {
            set.remove(&x);
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
