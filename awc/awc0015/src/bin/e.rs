use ac_library::FenwickTree;
use proconio::{input, marker::Usize1};

// 区間の種類を数える

fn main() {
    input! {
        n: usize,
        q: usize,
        p: [Usize1; n],
        lr: [(Usize1, Usize1); q],
    }

    let mut queries = lr.iter().enumerate().collect::<Vec<_>>();
    queries.sort_unstable_by_key(|(_, (_, r))| r);

    let mut results = vec![0; q];

    let mut fenwick = FenwickTree::new(n, 0isize);
    let mut last_pos = vec![usize::MAX; n];
    let mut cur_r = 0;
    for (i, &(l, r)) in queries {
        while cur_r <= r {
            let x = last_pos[p[cur_r]];
            if x < usize::MAX {
                fenwick.add(x, -1);
            }
            last_pos[p[cur_r]] = cur_r;
            fenwick.add(cur_r, 1);
            cur_r += 1;
        }

        results[i] = fenwick.sum(l..=r);
    }

    for result in results {
        println!("{result}");
    }
}
