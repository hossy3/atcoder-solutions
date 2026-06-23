use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        l: [usize; n],
    }

    let mut cur = 0;
    let mut result = 1;

    for l in l {
        if cur > 0 {
            cur += 1;
        }
        cur += l;
        if cur > w {
            result += 1;
            cur = l;
        }
    }

    println!("{result}");
}
