#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        g: [(usize, usize); n],
    }
    let mut dp = vec![vec![0; 100100]; 110];
    for i in 0..n {
        for sum_w in 0..=w {
            if sum_w >= g[i].0 {
                if dp[i + 1][sum_w] < dp[i][sum_w - g[i].0] + g[i].1 {
                    dp[i + 1][sum_w] = dp[i][sum_w - g[i].0] + g[i].1;
                }
            }
            if dp[i + 1][sum_w] < dp[i][sum_w] {
                dp[i + 1][sum_w] = dp[i][sum_w];
            }
        }
    }
    println!("{}", dp[n][w]);
}
