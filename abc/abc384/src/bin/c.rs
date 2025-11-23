use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
    }

    let mut results = BTreeMap::new();
    results.entry(a).or_insert(BTreeSet::new()).insert("A");
    results.entry(b).or_insert(BTreeSet::new()).insert("B");
    results.entry(a + b).or_insert(BTreeSet::new()).insert("AB");
    results.entry(c).or_insert(BTreeSet::new()).insert("C");
    results.entry(a + c).or_insert(BTreeSet::new()).insert("AC");
    results.entry(b + c).or_insert(BTreeSet::new()).insert("BC");
    results
        .entry(a + b + c)
        .or_insert(BTreeSet::new())
        .insert("ABC");
    results.entry(d).or_insert(BTreeSet::new()).insert("D");
    results.entry(a + d).or_insert(BTreeSet::new()).insert("AD");
    results.entry(b + d).or_insert(BTreeSet::new()).insert("BD");
    results
        .entry(a + b + d)
        .or_insert(BTreeSet::new())
        .insert("ABD");
    results.entry(c + d).or_insert(BTreeSet::new()).insert("CD");
    results
        .entry(a + c + d)
        .or_insert(BTreeSet::new())
        .insert("ACD");
    results
        .entry(b + c + d)
        .or_insert(BTreeSet::new())
        .insert("BCD");
    results
        .entry(a + b + c + d)
        .or_insert(BTreeSet::new())
        .insert("ABCD");
    results.entry(e).or_insert(BTreeSet::new()).insert("E");
    results.entry(a + e).or_insert(BTreeSet::new()).insert("AE");
    results.entry(b + e).or_insert(BTreeSet::new()).insert("BE");
    results
        .entry(a + b + e)
        .or_insert(BTreeSet::new())
        .insert("ABE");
    results.entry(c + e).or_insert(BTreeSet::new()).insert("CE");
    results
        .entry(a + c + e)
        .or_insert(BTreeSet::new())
        .insert("ACE");
    results
        .entry(b + c + e)
        .or_insert(BTreeSet::new())
        .insert("BCE");
    results
        .entry(a + b + c + e)
        .or_insert(BTreeSet::new())
        .insert("ABCE");
    results.entry(d + e).or_insert(BTreeSet::new()).insert("DE");
    results
        .entry(a + d + e)
        .or_insert(BTreeSet::new())
        .insert("ADE");
    results
        .entry(b + d + e)
        .or_insert(BTreeSet::new())
        .insert("BDE");
    results
        .entry(a + b + d + e)
        .or_insert(BTreeSet::new())
        .insert("ABDE");
    results
        .entry(c + d + e)
        .or_insert(BTreeSet::new())
        .insert("CDE");
    results
        .entry(a + c + d + e)
        .or_insert(BTreeSet::new())
        .insert("ACDE");
    results
        .entry(b + c + d + e)
        .or_insert(BTreeSet::new())
        .insert("BCDE");
    results
        .entry(a + b + c + d + e)
        .or_insert(BTreeSet::new())
        .insert("ABCDE");

    for (_, results) in results.iter().rev() {
        for result in results {
            println!("{result}");
        }
    }
}
