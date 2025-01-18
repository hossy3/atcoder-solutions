use proconio::input;

fn isqrt(x: usize) -> usize {
    (x as f64).sqrt() as usize
}

fn main() {
    input! {
        r: usize,
    }

    let mut result = 1usize;

    // 上下左右方向を足す
    let x = (isqrt((2 * r).pow(2) - 1) - 1) / 2;
    result += 4 * x;

    // 斜め
    for i in 1..r {
        let x = (isqrt((2 * r).pow(2) - (2 * i + 1).pow(2)) - 1) / 2;
        result += 4 * x;
    }

    println!("{result}");
}
