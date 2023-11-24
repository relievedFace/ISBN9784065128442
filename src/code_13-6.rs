use proconio::input;

type Graph = Vec<Vec<usize>>;

fn rec(g: &Graph, v: usize, seen: &mut Vec<bool>, order: &mut Vec<usize>) {
    seen[v] = true;

    for next_v in g[v].iter() {
        if seen[*next_v] {
            continue;
        }

        rec(g, *next_v, seen, order);
    }

    order.push(v);
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

    let mut seen = vec![false; n];
    let mut order = vec![];

    for v in 0..n {
        if seen[v] {
            continue;
        }

        rec(&g, v, &mut seen, &mut order);
    }

    order.reverse();

    for v in order.iter() {
        print!("{} -> ", v);
    }

    println!();
}
