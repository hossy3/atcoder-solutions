use proconio::input;

fn main() {
    input! {
        q: usize,
        tx: [(u8, usize); q],
    }

    let mut v = vec![];
    for (t, x) in tx {
        match t {
            1 => {
                v.push(x);
            }
            2 => {
                println!("{}", v[v.len() - x])
            }
            _ => unreachable!(),
        }
    }
}
