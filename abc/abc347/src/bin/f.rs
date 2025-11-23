use proconio::input;
use rustc_hash::FxHashMap;

fn area(i: usize, j: usize, m: usize, cum: &[Vec<i64>]) -> i64 {
    cum[i + m][j + m] + cum[i][j] - cum[i + m][j] - cum[i][j + m]
}

fn main() {
    input! {
        (n, m): (usize, usize),
        a: [[i64; n]; n],
    }
    let mut cum = vec![vec![0i64; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            cum[i + 1][j + 1] = cum[i + 1][j] + cum[i][j + 1] - cum[i][j] + a[i][j];
        }
    }

    let mut map = FxHashMap::<(usize, usize, usize, usize), i64>::default();

    for i in 0..=(n - m) {
        let mut val = 0;
        for j in 0..(n - m) {
            val = val.max(area(i, j, m, &cum));
        }
        map.insert((i, 0, i + m, n), val);
    }
    for i in 0..=(n - m - 1) {
        let mut val = *map.get(&(i, 0, i + m, n)).unwrap();
        for j in (i + 1)..=(n - m) {
            val = val.max(*map.get(&(j, 0, j + m, n)).unwrap());
            map.insert((i, 0, j + m, n), val);
        }
    }

    for j in 0..=(n - m) {
        let mut val = 0;
        for i in 0..=(n - m) {
            val = val.max(area(i, j, m, &cum));
        }
        map.insert((0, j, n, j + m), val);
    }
    for i in 0..=(n - m - 1) {
        let mut val = *map.get(&(0, i, n, i + m)).unwrap();
        for j in (i + 1)..=(n - m) {
            val = val.max(*map.get(&(0, j, n, j + m)).unwrap());
            map.insert((0, i, n, j + m), val);
        }
    }

    for i in 0..(n - m) {
        for j in 0..(n - m) {
            let val = area(i, j, m, &cum)
                .max(*map.get(&(0, 0, i + m - 1, j + m)).unwrap_or(&0))
                .max(*map.get(&(0, 0, i + m, j + m - 1)).unwrap_or(&0));
            map.insert((0, 0, i + m, j + m), val);

            let val = area(n - m - i, j, m, &cum)
                .max(*map.get(&(n - m - i, 0, n - 1, j + m)).unwrap_or(&0))
                .max(*map.get(&(n - m - i, 0, n, j + m - 1)).unwrap_or(&0));
            map.insert((n - m - i, 0, n, j + m), val);

            let val = area(i, n - m - j, m, &cum)
                .max(*map.get(&(0, n - m - j, i + m - 1, n)).unwrap_or(&0))
                .max(*map.get(&(0, n - m - j, i + m, n - 1)).unwrap_or(&0));
            map.insert((0, n - m - j, i + m, n), val);

            let val = area(n - m - i, n - m - j, m, &cum)
                .max(*map.get(&(n - m - i, n - m - j, n - 1, n)).unwrap_or(&0))
                .max(*map.get(&(n - m - i, n - m - j, n, n - 1)).unwrap_or(&0));
            map.insert((n - m - i, n - m - j, n, n), val);
        }
    }

    let mut result = 0i64;

    // 幅いっぱい×3
    for i in m..=(n - m * 2) {
        for j in (i + m)..=(n - m) {
            result = result.max(
                *map.get(&(0, 0, i, n)).unwrap()
                    + *map.get(&(i, 0, j, n)).unwrap()
                    + *map.get(&(j, 0, n, n)).unwrap(),
            );
        }
    }

    // 高さいっぱい×3
    for i in m..=(n - m * 2) {
        for j in (i + m)..=(n - m) {
            result = result.max(
                *map.get(&(0, 0, n, i)).unwrap()
                    + *map.get(&(0, i, n, j)).unwrap()
                    + *map.get(&(0, j, n, n)).unwrap(),
            );
        }
    }

    // その他
    for i in m..=(n - m) {
        for j in m..=(n - m) {
            result = result.max(
                *map.get(&(0, 0, i, j)).unwrap()
                    + *map.get(&(0, j, i, n)).unwrap()
                    + *map.get(&(i, 0, n, n)).unwrap(),
            );

            result = result.max(
                *map.get(&(0, 0, i, j)).unwrap()
                    + *map.get(&(i, 0, n, j)).unwrap()
                    + *map.get(&(0, j, n, n)).unwrap(),
            );

            result = result.max(
                *map.get(&(0, 0, n, j)).unwrap()
                    + *map.get(&(0, j, i, n)).unwrap()
                    + *map.get(&(i, j, n, n)).unwrap(),
            );

            result = result.max(
                *map.get(&(0, 0, i, n)).unwrap()
                    + *map.get(&(i, 0, n, j)).unwrap()
                    + *map.get(&(i, j, n, n)).unwrap(),
            );
        }
    }

    println!("{result}");
}
