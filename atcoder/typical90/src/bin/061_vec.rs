use proconio::input;

fn main() {
    input! {
        q: usize,
        tx: [(u8, usize); q]
    }

    let mut v = vec![];
    for (t, x) in tx {
        match t {
            1 => {
                v.insert(0, x);
            }
            2 => {
                v.push(x);
            }
            3 => {
                println!("{}", v[x - 1]);
            }
            _ => unreachable!(),
        }
    }
}
