use proconio::input;

struct UnionFind {
    par: Vec<Option<usize>>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: vec![None; n],
            siz: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if let Some(par_x) = self.par[x] {
            self.par[x] = Some(self.root(par_x));
            self.par[x].unwrap()
        } else {
            x
        }
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unit(&mut self, x: usize, y: usize) -> bool {
        let x = self.root(x);
        let y = self.root(y);

        if x == y {
            return false;
        }

        if self.siz[x] < self.siz[y] {
            self.siz.swap(x, y);
        }

        self.par[y] = Some(x);
        self.siz[x] += self.siz[y];
        true
    }

    fn size(&mut self, x: usize) -> usize {
        let x = self.root(x);
        self.siz[x]
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut edges: [(i64, usize, usize); m],
    }

    edges.sort();

    let mut res = 0;
    let mut uf = UnionFind::new(n);

    for (w, u, v) in edges.iter() {
        if uf.is_same(*u, *v) {
            continue;
        }

        res += w;
        uf.unit(*u, *v);
    }

    println!("{}", res);
}
