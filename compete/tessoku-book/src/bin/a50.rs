use itertools::Itertools;
use proconio::input;

// score: 9895664886

const N: usize = 100;

fn f(x: usize, y: usize, a: &[Vec<usize>]) -> usize {
    let mut h = a[y][x].min(N);

    for i in 1..N {
        if h <= i {
            break;
        }
        for j in 0..i {
            for k in 0..4 {
                let (x0, y0) = match k {
                    0 => ((x + i).wrapping_sub(j), y.wrapping_sub(j)),
                    1 => (x.wrapping_sub(j), (y + j).wrapping_sub(i)),
                    2 => ((x + j).wrapping_sub(i), y + j),
                    _ => (x + j, (y + i).wrapping_sub(j)),
                };
                if x0 >= N || y0 >= N {
                    continue;
                }

                let h0 = a[y0][x0] + i;
                if h > h0 {
                    h = h0;
                    if h == i {
                        return h;
                    }
                }
            }
        }
    }

    h
}

fn abs_diff(x: usize, y: usize) -> usize {
    if x > y {
        x - y
    } else {
        y - x
    }
}

fn main() {
    input! {
        mut a: [[usize; N]; N],
    }

    let mut xyh = Vec::new();

    for _ in 0..1000 {
        let i = (0..(N * N))
            .map(|i| (a[i / N][i % N], abs_diff(i / N + i % N, N - 1)))
            .position_max()
            .unwrap();
        let x = i % N;
        let y = i / N;

        let h = f(x, y, &a);
        if h == 0 {
            break;
        }
        xyh.push((x, y, h));

        let y0 = if y > h { y - h } else { 0 };
        let y1 = (y + h).min(N - 1);
        for y0 in y0..=y1 {
            let w = h - abs_diff(y, y0);
            let x0 = if x > w { x - w } else { 0 };
            let x1 = (x + w).min(N - 1);
            for x0 in x0..=x1 {
                a[y0][x0] -= h - abs_diff(y, y0) - abs_diff(x, x0);
            }
        }
    }

    println!("{}", xyh.len());
    for &(x, y, h) in &xyh {
        println!("{} {} {}", x, y, h);
    }
}
