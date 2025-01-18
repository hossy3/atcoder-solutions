use proconio::input;

// x: 上限
fn f(x: usize) -> usize {
    if x == 9 {
        return 0;
    }
    
    let mut result = 0usize;

    // 最後の桁 以外
    let mut k = 10usize;
    while k * 10 < x {
        let k0 = k.ilog10();
        for i in 1usize..=9 {
            result += i.pow(k0);
        }
        k *= 10;
    }

    // 最後の桁かつ最大値 以外
    let k0 = k.ilog10();
    for i in 1usize..(x / k) {
        result += i.pow(k0);
    }

    // 最後の桁かつ最大値
    let i = x / k;
    let mut v = vec![];
    let mut x = x;

    while x > 0 {
        v.push(x % 10);
        x /= 10;
    }
    v.pop();

    for j in (0..v.len()).rev() {
        if v[j] >= i {
            for j in 0..=j {
                v[j] = i - 1;
            }
            break;
        }
    }

    for j in 0..v.len() {
        result += v[j] * i.pow(j as u32);
    }
    result += 1;

    result
}

fn main() {
    input! {
        l: usize,
        r: usize,
    }
    let result = f(r) - f(l - 1);
    println!("{result}");
}
