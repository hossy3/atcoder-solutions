use proconio::input;

/// 巡回セールスマン問題 (Traveling Salesman Problem) を bit DP で解く
///
/// state: state[bits][i] bits 到達済みの、現在位置 i でのコスト最大値。事前に isize::MIN で埋めておくこと
/// dist: i から j に移動するときのコスト
fn solve_traveling_salesman<F>(state: &mut Vec<Vec<isize>>, dist: F)
where
    F: Fn(usize, usize, usize) -> isize,
{
    let m = state[0].len();
    assert_eq!(1 << m, state.len());

    let bit_test = |bits: usize, i: usize| bits & (1 << i) != 0;

    for visited in 0..(1 << m) {
        for i in 0..m {
            if state[visited][i] == isize::MIN {
                continue;
            }

            for j in 0..m {
                if bit_test(visited, j) {
                    continue;
                }
                let x = state[visited][i] + dist(visited, i, j);
                let visited = visited | (1 << j);
                state[visited][j] = state[visited][j].max(x);
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        w: [usize; n - 1],
    }

    let mut state = vec![vec![isize::MIN; n]; 1 << n];
    for i in 0..n {
        state[1 << i][i] = 0;
    }
    solve_traveling_salesman(&mut state, |visited, i, j| {
        (w[(visited.count_ones() - 1) as usize] * (p[i].abs_diff(p[j]))) as isize
    });
    let result = state[(1 << n) - 1].iter().max().unwrap();
    println!("{result}");
}
