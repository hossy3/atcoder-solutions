use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        w: usize,
        xy: [(Usize1, usize); n],
        q: usize,
        ta: [(usize, Usize1); q],
    }

    let mut m = vec![vec![]; w];
    for (i, &(x, y)) in xy.iter().enumerate() {
        m[x].push((y, i));
    }
    for i in 0..w {
        m[i].sort();
    }

    let min_height = m.iter().map(|a| a.len()).min().unwrap();

    // 消えるタイミングを求める
    let mut time = 0usize;
    let mut del_times = vec![usize::MAX; n];

    for i in 0..min_height {
        time += 1;
        for j in 0..w {
            let (y, _) = m[j][i];
            time = time.max(y);
        }
        for j in 0..w {
            let (_, k) = m[j][i];
            del_times[k] = time;
        }
    }

    for &(t, a) in &ta {
        let yes = del_times[a] > t;
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
