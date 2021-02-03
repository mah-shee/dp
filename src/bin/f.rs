#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut dp = vec![vec![0; 3100]; 3100];
    for i in 0..s.len() {
        for j in 0..t.len() {
            if s[i] == t[i] {
                if dp[i + 1][j + 1] < dp[i][j] + 1 {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                }
            }
            if dp[i + 1][j + 1] < dp[i + 1][j] {
                dp[i + 1][j + 1] = dp[i + 1][j];
            }
            if dp[i + 1][j + 1] < dp[i][j + 1] {
                dp[i + 1][j + 1] = dp[i][j + 1];
            }
        }
    }
    let mut res = String::new();
    let mut s_size = s.len();
    let mut t_size = t.len();

    while s_size > 0 && t_size > 0 {
        if dp[s_size][t_size] == dp[s_size - 1][t_size] {
            s_size -= 1;
        } else if dp[s_size][t_size] == dp[s_size][t_size - 1] {
            t_size -= 1;
        } else {
            res.insert(0, s[s_size - 1]);
            s_size -= 1;
            t_size -= 1;
        }
    }
    println!("{}", res);
}
