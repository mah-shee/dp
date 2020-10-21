#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::cmp::min;
#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        g: [(usize, usize); n],
    }
    let max_v = 100100;
    let mut dp = vec![vec![1 << 60; max_v]; 110];
    dp[0][0] = 0;
    for i in 0..n {
        for sum_v in 0..max_v {
            if sum_v >= g[i].1 {
                dp[i + 1][sum_v] = min(dp[i + 1][sum_v], dp[i][sum_v - g[i].1] + g[i].0);
            }
            dp[i + 1][sum_v] = min(dp[i + 1][sum_v], dp[i][sum_v]);
        }
    }
    let mut res = 0;
    for i in 0..max_v {
        if dp[n][i] <= w {
            res = i;
        }
    }
    println!("{}", res);
}
