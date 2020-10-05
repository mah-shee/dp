#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        h: [isize; n],
    }
    let mut dp = vec![0; n];
    for i in 1..n {
        let min = std::cmp::max(0, i as i64 - k as i64) as usize;
        dp[i] = dp[min..i]
            .iter()
            .enumerate()
            .map(|(j, &c)| c + (h[i] - h[j + min]).abs())
            .min()
            .unwrap();
    }
    println!("{}", dp[n - 1]);
}
