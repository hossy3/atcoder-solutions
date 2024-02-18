use proconio::input;

fn is_intersect(
    (x0, y0, x1, y1): (usize, usize, usize, usize),
    (x2, y2, x3, y3): (usize, usize, usize, usize),
) -> bool {
    x0.max(x2) < x1.min(x3) && y0.max(y2) < y1.min(y3)
}

fn f(h: usize, w: usize, mut a: Vec<u32>) -> bool {
    a.sort();
    a.reverse();

    let mut stack = vec![vec![(0usize, 0usize)]]; // positions
    while let Some(v) = stack.pop() {
        let i = v.len();
        if i == a.len() {
            return true;
        }
        for j in 0..i {
            let x = v[j];

            let x1 = x.0 + 2usize.pow(a[j]);
            let y1 = x.1;
            let len = 2usize.pow(a[i]);
            if y1 + len <= h && x1 + len <= w {
                if v.iter().enumerate().all(|(i, &(x0, y0))| {
                    !is_intersect(
                        (x0, y0, x0 + 2usize.pow(a[i]), y0 + 2usize.pow(a[i])),
                        (x1, y1, x1 + len, y1 + len),
                    )
                }) {
                    let mut v = v.clone();
                    v.push((x1, y1));
                    stack.push(v);
                }
            }

            let x1 = x.0;
            let y1 = x.1 + 2usize.pow(a[j]);
            let len = 2usize.pow(a[i]);
            if y1 + len <= h && x1 + len <= w {
                if v.iter().enumerate().all(|(i, &(x0, y0))| {
                    !is_intersect(
                        (x0, y0, x0 + 2usize.pow(a[i]), y0 + 2usize.pow(a[i])),
                        (x1, y1, x1 + len, y1 + len),
                    )
                }) {
                    let mut v = v.clone();
                    v.push((x1, y1));
                    stack.push(v);
                }
            }
        }
    }

    false
}

fn main() {
    input! {
        (h, w, n): (usize, usize, usize),
        a: [u32; n],
    }
    let yes = f(h, w, a);
    println!("{}", if yes { "Yes" } else { "No" });
}
