use proconio::{input, marker::Usize1};
use smallvec::SmallVec;

fn dfs(i: usize, prev: usize, edges: &[SmallVec<[usize; 2]>], nums: &mut [usize]) -> usize {
    let mut count = 1;
    for &j in &edges[i] {
        if j != prev {
            count += dfs(j, i, edges, nums);
        }
    }
    nums[i] = count;
    count
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1]
    }

    let mut edges = vec![SmallVec::<[_; 2]>::new(); n];
    for &(a, b) in &ab {
        edges[a].push(b);
        edges[b].push(a);
    }
    let mut nums = vec![0; n];
    dfs(0, 0, &edges, &mut nums);

    let mut count = 0;
    for &(a, b) in &ab {
        let x = nums[a].min(nums[b]);
        count += x * (n - x);
    }
    println!("{}", count);
}
