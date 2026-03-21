use proconio::{input, marker::Usize1};

/// 巡回セールスマン問題 (Traveling Salesman Problem) を bit DP で解く
///
/// state: state[bits][i] bits 到達済みの、現在位置 i でのコスト最小値。事前に usize::MAX で埋めておくこと
/// dist: i から j に移動するときのコスト
fn solve_traveling_salesman<F>(state: &mut Vec<Vec<usize>>, dist: F)
where
    F: Fn(usize, usize) -> usize,
{
    let m = state[0].len();
    assert_eq!(1 << m, state.len());

    let bit_test = |bits: usize, i: usize| bits & (1 << i) != 0;

    for visited in 0..(1 << m) {
        for i in 0..m {
            if state[visited][i] == usize::MAX {
                continue;
            }

            for j in 0..m {
                if bit_test(visited, j) {
                    continue;
                }
                let x = state[visited][i] + dist(i, j);
                let visited = visited | (1 << j);
                state[visited][j] = state[visited][j].min(x);
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Usize1,
        t: Usize1,
        p: [Usize1; m],
    }

    let dist = |p0: usize, p1: usize| {
        let (x0, y0) = (p0 / n, p0 % n);
        let (x1, y1) = (p1 / n, p1 % n);
        x1.abs_diff(x0) + y1.abs_diff(y0)
    };

    if m == 0 {
        let result = dist(s, t);
        println!("{result}");
        return;
    }

    let mut state = vec![vec![usize::MAX; m]; 1 << m];
    for i in 0..m {
        let visited = 1 << i;
        state[visited][i] = dist(s, p[i]);
    }
    solve_traveling_salesman(&mut state, |i, j| dist(p[i], p[j]));

    let all_visited = (1 << m) - 1;
    let result = (0..m)
        .map(|i| state[all_visited][i] + dist(p[i], t))
        .min()
        .unwrap();
    println!("{result}");
}
