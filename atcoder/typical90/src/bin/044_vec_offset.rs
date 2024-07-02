use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        txy: [(u8, usize, usize); q],
    }
    let mut v = a.clone();
    let mut offset = 0;
    let f = |x, offset| (x - 1 + offset) % n;
    for (t, x, y) in txy {
        match t {
            1 => {
                v.swap(f(x, offset), f(y, offset));
            }
            2 => {
                offset = f(n, offset);
            }
            3 => {
                println!("{}", v[f(x, offset)]);
            }
            _ => unreachable!(),
        }
    }
}
