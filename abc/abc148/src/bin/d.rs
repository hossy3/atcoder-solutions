use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    // 貪欲に
    let mut cur = 1;
    let mut result = 0; // 壊した数
    for x in a {
        if x == cur {
            cur += 1;
        } else {
            result += 1;
        }
    }

    if result < n {
        println!("{result}");
    } else {
        println!("-1");
    }
}
