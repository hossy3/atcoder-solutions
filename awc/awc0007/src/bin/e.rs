use proconio::{input, marker::Usize1};

// 巡回セールスマン (TSP), bit DP

trait BitTest {
    fn bit_test(&self, i: usize) -> bool;
}

impl BitTest for usize {
    fn bit_test(&self, i: usize) -> bool {
        self & (1 << i) != 0
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

    for visited in 1..(1 << m) {
        // 現在地
        for i in 0..m {
            if !visited.bit_test(i) {
                continue;
            }

            // 次の移動先
            for j in 0..m {
                if visited.bit_test(j) {
                    continue;
                }
                let x = state[visited][i] + dist(p[i], p[j]);
                let visited = visited | (1 << j);
                state[visited][j] = state[visited][j].min(x);
            }
        }
    }

    let all_visited = (1 << m) - 1;
    let result = (0..m)
        .map(|i| state[all_visited][i] + dist(p[i], t))
        .min()
        .unwrap();
    println!("{result}");
}
