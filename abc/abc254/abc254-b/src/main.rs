// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut a = [[0; 30]; 30];
    for i in 0..n {
        for j in 0..=i {
            a[i][j] = if j == 0 || j == i {
                1
            } else {
                a[i - 1][j - 1] + a[i - 1][j]
            };
            print!("{}{}", if j == 0 { "" } else { " " }, a[i][j]);
        }
        println!();
        // let txts: Vec<String> = a[i][0..i + 1].iter().map(|x| x.to_string()).collect();
        // println!("{}", txts.join(" "));
    }
}
