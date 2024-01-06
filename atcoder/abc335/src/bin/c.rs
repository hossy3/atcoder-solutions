use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut v = Vec::with_capacity(n + q);
    for i in (1..=n).rev() {
        v.push((i as i64, 0i64));
    }
    for _ in 0..q {
        input! {
            t: u8,
        }
        if t == 1 {
            input! {
                c: char,
            }
            let (x, y) = v[v.len() - 1];
            let (x, y) = match c {
                'R' => (x + 1, y),
                'L' => (x - 1, y),
                'U' => (x, y + 1),
                'D' => (x, y - 1),
                _ => unreachable!(),
            };
            v.push((x, y));
        } else if t == 2 {
            input! {
                p: usize,
            }
            let (x, y) = v[v.len() - p];
            println!("{x} {y}");
        }
    }
}
