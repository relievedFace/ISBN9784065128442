use proconio::input;
use std::{collections::BinaryHeap, vec};

const INF: i64 = 1 << 60;

#[derive(Clone)]
struct Edge {
    to: usize,
    w: i64,
}

impl Edge {
    fn new(to: usize, w: i64) -> Self {
        Edge { to, w }
    }
}

type _Graph = Vec<Vec<Edge>>;

fn chmin<T: Clone + Copy + Ord>(a: &mut T, b: &T) -> bool {
    if *a > *b {
        *a = *b;
        true
    } else {
        false
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        abw: [(usize, usize, i64); m],
    }

    let mut g = vec![vec![]; n];

    for (a, b, w) in abw.iter() {
        g[*a].push(Edge::new(*b, *w));
    }

    let mut dist = vec![INF; n];
    dist[s] = 0;

    let mut que = BinaryHeap::new();
    que.push((dist[s], s));

    while let Some((d, v)) = que.pop() {
        if d > dist[v] {
            break;
        }

        for e in g[v].iter() {
            let dvw = dist[e.to] + e.w;

            if chmin(&mut dist[e.to], &dvw) {
                que.push((dist[e.to], e.to));
            }
        }
    }

    for dist_v in dist.iter() {
        if *dist_v < INF {
            println!("{}", *dist_v);
        } else {
            println!("INF");
        }
    }
}
