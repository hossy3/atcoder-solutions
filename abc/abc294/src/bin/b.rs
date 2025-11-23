use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u8; w]; h],
    }
    for i in 0..h {
        for j in 0..w {
            let x = a[i][j];
            let c = if x == 0 {
                '.'
            } else {
                (b'A' - 1u8 + x) as char
            };
            print!("{}", c);
        }
        println!();
    }
}
