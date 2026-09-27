use proconio::{input, marker::Usize1};

fn f(ab: &[(usize, usize)], a: usize, b: usize) -> bool {
    for &(a1, b1) in ab {
        if !(a1 == a || a1 == b || b1 == a || b1 == b) {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    // 出現数で並び替える
    let mut counts = vec![];
    for i in 0..n {
        counts.push((i, 0usize));
    }
    for &(a, b) in &ab {
        counts[a].1 += 1;
        counts[b].1 += 1;
    }
    counts.sort_by_key(|&(_, count)| count);

    let mut result = 0usize;
    for &(a, count) in &counts {
        let b_min = (counts.partition_point(|&(_, count0)| count + count0 < m)).max(a + 1);
        for b in b_min..n {
            if f(&ab, a, b) {
                result += 1;
            }
        }
    }
    println!("{result}");
}
