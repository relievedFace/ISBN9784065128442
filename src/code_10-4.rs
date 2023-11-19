use proconio::input;

#[derive(Clone)]
struct Edge {
    to: usize,
    w: usize,
}

impl Edge {
    fn new(to: usize, w: usize) -> Self {
        Edge { to, w }
    }
}

type Graph = Vec<Vec<Edge>>;

fn main() {
    input! {
        n: usize,
        m: usize,
        abw: [(usize, usize, usize); m],
    }

    let mut g: Graph = vec![vec![]; n];

    for &(a, b, w) in &abw {
        g[a].push(Edge::new(b, w));
    }
}
