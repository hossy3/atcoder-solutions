use proconio::input;

fn main() {
    input! {
        n: usize,
        mut r: isize,
        da: [(u8, isize); n],
    }

    for (d, a) in da {
        match d {
            1 => {
                if 1600 <= r && r <= 2799 {
                    r += a;
                }
            }
            2 => {
                if 1200 <= r && r <= 2399 {
                    r += a;
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{r}");
}
