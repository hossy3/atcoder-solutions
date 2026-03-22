use proconio::input;

fn main() {
    input! {
        mut h: usize,
        mut w: usize,
        q: usize,
    }

    for _ in 0..q {
        input! {
            u: u8,
            x: usize,
        }

        match u {
            1 => {
                let result = x * w;
                println!("{result}");
                h -= x;
            }
            2 => {
                let result = h * x;
                println!("{result}");
                w -= x;
            }
            _ => unreachable!(),
        }
    }
}
