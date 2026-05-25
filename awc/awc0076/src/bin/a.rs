use proconio::input;

fn main() {
    input! {
        mut s: usize,
        p: usize,
        r: usize,
        m: usize,
        ev: [(u8, usize); m],
    }

    for &(e, v) in &ev {
        match e {
            1 => {
                s += v;
            }
            2 => {
                s -= p * v;
            }
            _ => unreachable!(),
        }
    }
    let result = s - r;
    println!("{result}");
}
