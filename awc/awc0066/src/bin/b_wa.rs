use proconio::input;

// 途中で充電を繋ぎ変えられると誤読した場合

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        y: usize,
        mut a: [usize; n],
    }

    a.sort_unstable();

    let mut rest = y * k;
    let mut result = 0usize;
    for &a in a.iter().rev() {
        if a > y + l {
            result += 1;
            continue;
        }
        if a <= l {
            break; // 充電しても満たせない
        }
        if a + rest.min(y) > y + l {
            rest -= y + l - a + 1;
            result += 1;
        } else {
            break;
        }
    }
    println!("{result}");
}
