// -*- coding:utf-8-unix -*-

use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    }
    let mut m = HashMap::<String, i64>::new();
    for s in ss {
        if let Some(i) = m.get_mut(&s) {
            println!("{}({})", s, i);
            *i += 1;
        } else {
            println!("{}", s);
            m.insert(s, 1);
        }
    }
}
