use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn f(n: usize, a: usize, b: usize) -> Option<Vec<char>> {
    if n % 2 != 0 {
        return None; // 幅が奇数のときは満たさない
    }
    if (a + b) % 2 == 0 {
        return None; // 偶数奇数と交互に動くので、偶数マスが減ると満たさなくなる
    }

    let mut result = vec![];
    let mut i = 0; // 現在位置
    let mut j = 0;
    while !(i == n - 1 && j == n - 1) {
        if i == a || i + 1 == a {
            let i0 = i;
            if j == 0 {
                while !(i == i0 + 1 && j == n - 1) {
                    if i == i0 {
                        if !(a == i + 1 && b == j) {
                            result.push('D');
                            i += 1;
                        }
                    } else {
                        if !(a == i - 1 && b == j) {
                            result.push('U');
                            i -= 1;
                        }
                    }
                    if j < n - 1 {
                        result.push('R');
                        j += 1;
                    }
                }
            } else {
                while !(i == i0 + 1 && j == 0) {
                    if i == i0 {
                        if !(a == i + 1 && b == j) {
                            result.push('D');
                            i += 1;
                        }
                    } else {
                        if !(a == i - 1 && b == j) {
                            result.push('U');
                            i -= 1;
                        }
                    }
                    if j > 0 {
                        result.push('L');
                        j -= 1;
                    }
                }
            }
        } else {
            if j == 0 {
                result.append(&mut vec!['R'; n - 1]);
                j = n - 1;
            } else {
                result.append(&mut vec!['L'; n - 1]);
                j = 0;
            }
        }
        if i < n - 1 {
            result.push('D');
            i += 1;
        }
    }

    Some(result)
}

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            a: Usize1,
            b: Usize1,
        }
        if let Some(result) = f(n, a, b) {
            println!("Yes");
            println!("{}", result.iter().join(""));
        } else {
            println!("No");
        }
    }
}
