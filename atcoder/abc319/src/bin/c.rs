use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        c: [usize; 9],
    }
    let mut count = 0usize;
    let v0 = [
        (0, 1, 2),
        (3, 4, 5),
        (6, 7, 8),
        (0, 3, 6),
        (1, 4, 7),
        (2, 5, 8),
        (0, 4, 8),
        (2, 4, 6),
    ];
    'outer: for v in (0..9).permutations(9) {
        let mut vinv = [0; 9]; // v: その位置が何番目に選ばれたかの配列, vinv: 位置の配列
        for (i, &x) in v.iter().enumerate() {
            vinv[x] = i;
        }
        for &(i0, i1, i2) in &v0 {
            let (x0, x1, x2) = (v[i0], v[i1], v[i2]);
            if (c[i0] == c[i1] && x0 < x2 && x1 < x2)
                || (c[i1] == c[i2] && x1 < x0 && x2 < x0)
                || (c[i2] == c[i0] && x2 < x1 && x0 < x1)
            {
                continue 'outer;
            }
        }
        count += 1;
    }
    let result = count as f64 / (9 * 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1) as f64;
    println!("{result}");
}
