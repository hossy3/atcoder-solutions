use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::input;

fn f(i: usize, a: &[usize], v: &[bool; 16], uf: &UnionFind<usize>) -> usize {
    if i == 16 {
        let result = if (0..16)
            .filter(|&i| v[i])
            .tuple_windows()
            .all(|(x, y)| uf.equiv(x, y))
        {
            1
        } else {
            0
        };
        return result;
    }

    let mut result = 0;
    if a[i] == 0 {
        if (i < 4) || (i % 4 == 0) || !(!v[i - 5] && v[i - 4] && v[i - 1]) {
            result += f(i + 1, a, v, uf);
        }
    }

    if (i < 4)
        || (i % 4 == 0)
        || (!(v[i - 5] && !v[i - 4] && !v[i - 1])
            && !(!v[i - 5] && v[i - 4] && v[i - 1] && uf.equiv(i - 4, i - 1)))
    {
        let mut uf = uf.clone();
        if i >= 4 && v[i - 4] {
            uf.union(i, i - 4);
        }
        if i % 4 > 0 && v[i - 1] {
            uf.union(i, i - 1);
        }
        let mut v = v.clone();
        v[i] = true;
        result += f(i + 1, a, &v, &uf);
    }

    result
}

fn main() {
    input! {
        a: [usize; 16],
    }
    let v = [false; 16];
    let uf = UnionFind::new(16);
    let result = f(0, &a, &v, &uf);
    println!("{}", result);
}
