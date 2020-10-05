#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        activities: [[usize; 3]; n],
    }
    let mut dp = vec![vec![0; 3]; n + 1];

    for i in 0..n {
        for j in 0..3 {
            for k in 0..3 {
                if j == k {
                    continue;
                }
                if dp[i + 1][k] < dp[i][j] + activities[i][k] {
                    dp[i + 1][k] = dp[i][j] + activities[i][k];
                }
            }
        }
    }
    let mut max = dp[n][0];
    for i in 1..3 {
        if max < dp[n][i] {
            max = dp[n][i];
        }
    }
    println!("{}", max);
}
