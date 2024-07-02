use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        txy: [(u8, usize, usize); q],
    }
    let mut v = a.clone();
    for (t, x, y) in txy {
        match t {
            1 => {
                v.swap(x - 1, y - 1);
            }
            2 => {
                let x = v.pop().unwrap();
                v.insert(0, x);
            }
            3 => {
                println!("{}", v[x - 1]);
            }
            _ => unreachable!(),
        }
    }
}
