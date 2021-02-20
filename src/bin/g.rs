#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];

    for (x, y) in edges {
        graph[x].push(y);
    }
    let mut visit = vec![false; n];
    let mut dp = vec![0; n];
    let mut ans = 0;
    for v in 0..n {
        ans = std::cmp::max(ans, rec(v, &graph, &mut visit, &mut dp));
    }
    println!("{}", ans);
}
fn rec(v: usize, graph: &Vec<Vec<usize>>, visit: &mut Vec<bool>, dp: &mut Vec<usize>) -> usize {
    if visit[v] {
        return dp[v];
    }
    visit[v] = true;

    for &u in graph[v].iter() {
        dp[v] = std::cmp::max(dp[v], 1 + rec(u, graph, visit, dp));
    }
    dp[v]
}
