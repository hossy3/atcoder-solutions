use proconio::input;

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
        v: [usize; n],
    }

    let mut state = vec![vec![usize::MAX; n]; 1 << n];
    state[1][0] = 0;
    solve_traveling_salesman(&mut state, |i, j| (v[j].abs_diff(v[i])) * i.abs_diff(j));
    let result = state[(1 << n) - 1].iter().min().unwrap();
    println!("{result}");
}
