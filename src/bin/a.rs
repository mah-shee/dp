#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        h: [isize; n],
    }
    let mut dp = vec![0; n];
    dp[1] = (h[0] - h[1]).abs();
    for i in 2..n {
        // if (h[i] - h[i - 1]).abs() <= (h[i] - h[i - 2]).abs() {
        //    dp[i] = (h[i] - h[i - 1]).abs() + dp[i - 1];
        // } else {
        //     dp[i] = (h[i] - h[i - 2]).abs() + dp[i - 2];
        // }
        dp[i] = std::cmp::min(
            (h[i] - h[i - 1]).abs() + dp[i - 1],
            (h[i] - h[i - 2]).abs() + dp[i - 2],
        );
    }
    println!("{}", dp[n - 1]);
}
