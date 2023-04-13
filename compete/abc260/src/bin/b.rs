// -*- coding:utf-8-unix -*-

use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        z: usize,
        aa: [i64; n],
        bb: [i64; n],
    }

    let mut passed = BTreeSet::<usize>::new();
    let mut v = Vec::<(i64, usize)>::new(); // -score, index
    for i in 0..n {
        v.push((-aa[i], i));
    }
    v.sort();
    for i in 0..x {
        passed.insert(v[i].1);
    }

    let mut v = Vec::<(i64, usize)>::new(); // -score, index
    for i in 0..n {
        if !passed.contains(&i) {
            v.push((-bb[i], i));
        }
    }
    v.sort();
    for i in 0..y {
        passed.insert(v[i].1);
    }

    let mut v = Vec::<(i64, usize)>::new(); // -score, index
    for i in 0..n {
        if !passed.contains(&i) {
            v.push((-(aa[i] + bb[i]), i));
        }
    }
    v.sort();
    for i in 0..z {
        passed.insert(v[i].1);
    }

    for i in 0..n {
        if passed.contains(&i) {
            println!("{}", i + 1);
        }
    }
}
