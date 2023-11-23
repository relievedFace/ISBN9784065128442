use proconio::input;
use std::collections::VecDeque;

type Graph = Vec<Vec<usize>>;

fn bfs(g: &Graph, s: usize) -> Vec<Option<usize>> {
    let n = g.len();
    let mut dist = vec![None; n];
    let mut que = VecDeque::new();

    // 本に記載されているコード
    // 始点が0になってる？
    // dist[0] = Some(0);
    // que.push_back(0);

    // 始点がsの場合
    dist[s] = Some(0);
    que.push_back(s);

    while let Some(v) = que.pop_front() {
        for x in g[v].iter() {
            if dist[*x].is_some() {
                continue;
            }

            dist[*x] = dist[v].map(|dist_v| dist_v + 1);
            que.push_back(*x);
        }
    }

    dist
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut g = vec![vec![]; n];

    for (a, b) in ab.iter() {
        g[*a].push(*b);
        g[*b].push(*a);
    }

    let dist = bfs(&g, 0);

    for (v, dist_v) in dist.iter().enumerate() {
        println!("{}: {:?}", v, dist_v);
    }
}
