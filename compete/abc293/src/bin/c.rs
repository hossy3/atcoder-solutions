use im_rc::HashSet;
use proconio::input;

fn f(x: usize, y: usize, a: &Vec<Vec<usize>>, s: &mut HashSet<usize>) -> usize {
    let w = a[0].len();
    let h = a.len();
    let i = a[y][x];
    if s.contains(&i) {
        return 0;
    }
    if x + 1 == w && y + 1 == h {
        return 1;
    }
    s.insert(i);
    let mut count = 0;
    if x + 1 < w {
        count += f(x + 1, y, a, s);
    }
    if y + 1 < h {
        count += f(x, y + 1, a, s);
    }
    s.remove(&i);
    count
}

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }
    let mut s = HashSet::new();
    let count = f(0, 0, &a, &mut s);
    println!("{}", count);
}
