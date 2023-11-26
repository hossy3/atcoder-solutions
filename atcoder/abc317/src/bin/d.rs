use proconio::input;

fn main() {
    input! {
        n: usize,
        xyz: [(i64, i64, i64); n],
    }

    let mut az = vec![]; // あと a 票取ると z 議席取れる
    let mut z_rest = xyz.iter().map(|(_, _, z)| z).sum::<i64>() / 2 + 1; // 勝利条件
    for &(x, y, z) in &xyz {
        if y > x {
            az.push(((y - x) / 2 + 1, z));
        } else {
            z_rest -= z;
        }
    }
    if z_rest <= 0 {
        println!("0");
        return;
    }

    let z_sum: i64 = az.iter().map(|(_, z)| z).sum();
    let mut v = vec![i64::MAX; z_sum as usize + 1]; // 鞍替えに必要な人数
    v[0] = 0;
    for &(a, z) in &az {
        let v0 = v.clone();
        for (i, &x) in v0.iter().enumerate() {
            if x == i64::MAX {
                continue;
            }
            v[i + z as usize] = v[i + z as usize].min(x + a);
        }
    }

    if let Some(result) = v[(z_rest as usize)..].iter().min() {
        println!("{result}");
    }
}
