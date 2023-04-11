use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        a: [i64],
    }
    let evens = a.iter().filter(|x| **x % 2 == 0).sorted().collect_vec();
    let odds = a.iter().filter(|x| **x % 2 == 1).sorted().collect_vec();

    let result = if evens.len() < 2 {
        if odds.len() < 2 {
            -1
        } else {
            odds[odds.len() - 2] + odds[odds.len() - 1]
        }
    } else {
        if odds.len() < 2 {
            evens[evens.len() - 2] + evens[evens.len() - 1]
        } else {
            (odds[odds.len() - 2] + odds[odds.len() - 1])
                .max(evens[evens.len() - 2] + evens[evens.len() - 1])
        }
    };
    println!("{}", result);
}
