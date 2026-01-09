use ac_library::Dsu;
use itertools::Itertools;
use proconio::input;

fn is_intersect((x0, x1): (isize, isize), (x2, x3): (isize, isize)) -> bool {
    x0.max(x2) < x1.min(x3)
}

fn main() {
    input! {
        n: usize,
        x: isize,
        y: isize,
        cab: [(char, isize, isize); n],
    }

    // かたまりを動かす
    let mut state = vec![((0, 0), (x, y))];
    for &(c, a, b) in &cab {
        let mut state0 = vec![];
        for ((x0, y0), (x1, y1)) in state {
            match c {
                'X' => {
                    if a <= x0 {
                        state0.push(((x0, y0 + b), (x1, y1 + b)));
                    } else if a < x1 {
                        state0.push(((x0, y0 - b), (a, y1 - b)));
                        state0.push(((a, y0 + b), (x1, y1 + b)));
                    } else {
                        state0.push(((x0, y0 - b), (x1, y1 - b)));
                    }
                }
                'Y' => {
                    if a <= y0 {
                        state0.push(((x0 + b, y0), (x1 + b, y1)));
                    } else if a < y1 {
                        state0.push(((x0 - b, y0), (x1 - b, a)));
                        state0.push(((x0 + b, a), (x1 + b, y1)));
                    } else {
                        state0.push(((x0 - b, y0), (x1 - b, y1)));
                    }
                }
                _ => {
                    unreachable!()
                }
            }
        }
        state = state0;
    }

    // かたまりをつなげる
    let mut dsu = Dsu::new(state.len());
    for (i, &((x0, y0), (x1, y1))) in state.iter().enumerate() {
        for (j, &((x2, y2), (x3, y3))) in state[(i + 1)..].iter().enumerate() {
            let j = i + 1 + j;
            if (((x0 == x3) || (x1 == x2)) && is_intersect((y0, y1), (y2, y3)))
                || (((y0 == y3) || (y1 == y2)) && is_intersect((x0, x1), (x2, x3)))
            {
                dsu.merge(i, j);
            }
        }
    }

    // それぞれ大きさを求める
    let mut results = vec![];
    for v in dsu.groups() {
        let area: isize = v
            .iter()
            .map(|&i| state[i])
            .map(|((x0, y0), (x1, y1))| (x1 - x0) * (y1 - y0))
            .sum();
        results.push(area);
    }
    results.sort();
    println!("{}", results.len());
    println!("{}", results.iter().join(" "));
}
