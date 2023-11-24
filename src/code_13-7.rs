use proconio::input;
use std::iter::zip;

type Graph = Vec<Vec<usize>>;

fn dfs(
    g: &Graph,
    v: usize,
    p: Option<usize>,
    d: usize,
    depth: &mut Vec<Option<usize>>,
    subtree_size: &mut Vec<usize>,
) {
    depth[v] = Some(d);

    for c in g[v].iter() {
        if let Some(p) = p {
            if p == *c {
                continue;
            }
        }

        dfs(g, *c, Some(v), d + 1, depth, subtree_size);
    }

    subtree_size[v] = 1;

    for c in g[v].iter() {
        if let Some(p) = p {
            if p == *c {
                continue;
            }
        }

        subtree_size[v] += subtree_size[*c];
    }
}
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut g = vec![vec![]; n];

    for (a, b) in ab.iter() {
        g[*a].push(*b);
        g[*b].push(*a);
    }

    let root = 0;
    let mut depth = vec![None; n];
    let mut subtree_size = vec![0; n];

    dfs(&g, root, None, 0, &mut depth, &mut subtree_size);

    for (v, (depth, subtree_size)) in zip(depth.iter(), subtree_size.iter()).enumerate() {
        println!(
            "{}: depth = {:?}, subtree_size = {}",
            v, depth, subtree_size
        );
    }
}
