#[snippet = "strongly_connected_component"]
#[allow(dead_code)]
pub fn strongly_connected_component(g: &[Vec<usize>]) -> Vec<usize> {
    fn dfs(v: usize, g: &[Vec<usize>], vs: &mut Vec<usize>, used: &mut [bool]) {
        used[v] = true;
        for &to in &g[v] {
            if !used[to] {
                dfs(to, g, vs, used);
            }
        }
        vs.push(v);
    }
    fn rdfs(v: usize, k: usize, g_rev: &[Vec<usize>], cmp: &mut [usize], used: &mut [bool]) {
        used[v] = true;
        cmp[v] = k;

        for &to in &g_rev[v] {
            if !used[to] {
                rdfs(to, k, g_rev, cmp, used);
            }
        }
    }

    let g_rev = {
        let mut g_rev = vec![Vec::new(); g.len()];
        for (i, es) in g.iter().enumerate() {
            for &to in es {
                g_rev[to].push(i);
            }
        }
        g_rev
    };

    let mut vs = Vec::new();
    let mut used = vec![false; g.len()];
    for v in 0..g.len() {
        if !used[v] {
            dfs(v, g, &mut vs, &mut used);
        }
    }
    let mut used = vec![false; g.len()];
    let mut cmp = vec![0; g.len()];
    let mut k = 0;
    for &v in vs.iter().rev() {
        if !used[v] {
            rdfs(v, k, &g_rev, &mut cmp, &mut used);
            k += 1;
        }
    }

    cmp
}

#[snippet = "LCA"]
#[allow(dead_code)]
pub struct LCA {
    pub depth: Vec<usize>,
    pub parent: Vec<Vec<Option<usize>>>,
}

#[snippet = "LCA"]
#[allow(dead_code)]
impl LCA {
    pub fn new(g: &[Vec<usize>]) -> LCA {
        LCA::with_root(0, g)
    }

    pub fn with_root(root: usize, g: &[Vec<usize>]) -> LCA {
        fn dfs(
            i: usize,
            p: Option<usize>,
            d: usize,
            g: &[Vec<usize>],
            depth: &mut [usize],
            parent: &mut [Vec<Option<usize>>],
        ) {
            parent[i][0] = p;
            depth[i] = d;

            for &t in &g[i] {
                if Some(t) != p {
                    dfs(t, Some(i), d + 1, g, depth, parent);
                }
            }
        }
        let n = g.len();
        let l2 = (1..).find(|i| 1usize << i > n).unwrap();
        let mut depth = vec![0; n];
        let mut parent = vec![vec![None; l2 + 1]; n];

        dfs(root, None, 0, &g, &mut depth, &mut parent);

        for i in 1..l2 + 1 {
            for j in 0..n {
                if let Some(p) = parent[j][i - 1] {
                    parent[j][i] = parent[p][i - 1];
                }
            }
        }

        LCA {
            depth: depth,
            parent: parent,
        }
    }

    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        use std::mem::swap;
        if self.depth[b] < self.depth[a] {
            swap(&mut a, &mut b);
        }

        while self.depth[a] != self.depth[b] {
            b = self.parent[b][(self.depth[b] - self.depth[a]).trailing_zeros() as usize].unwrap();
        }

        if a == b {
            return a;
        }

        for i in (0..self.parent[0].len()).rev() {
            if self.parent[a][i] != self.parent[b][i] {
                a = self.parent[a][i].unwrap();
                b = self.parent[b][i].unwrap();
            }
        }
        self.parent[a][0].unwrap()
    }
}

#[snippet = "Tree"]
#[snippet = "HeavyLightDecomposition"]
pub struct Tree {
    pub root: usize,
    pub parent: Vec<Option<usize>>,
    pub childs: Vec<Vec<usize>>,
}

#[snippet = "Tree"]
#[snippet = "HeavyLightDecomposition"]
impl Tree {
    pub fn from_neighbor_list(n: usize, root: usize, g: &[Vec<usize>]) -> Tree {
        let mut parent = vec![None; n];
        let mut childs = vec![Vec::new(); n];

        let mut stack = vec![(root, None)];

        while let Some((i, p)) = stack.pop() {
            parent[i] = p;

            for &to in &g[i] {
                if Some(to) != p {
                    stack.push((to, Some(i)));
                    childs[i].push(to);
                }
            }
        }

        Tree {
            root: root,
            parent: parent,
            childs: childs,
        }
    }
}

#[snippet = "HeavyLightDecomposition"]
pub struct HeavyLightDecomposition {
    pub ids: Vec<(usize, usize)>,
    pub parents: Vec<Option<(usize, usize)>>,
    pub parts: Vec<Vec<usize>>,
}

#[snippet = "HeavyLightDecomposition"]
impl HeavyLightDecomposition {
    pub fn new(tree: &Tree) -> HeavyLightDecomposition {
        fn size(i: usize, tree: &Tree, memo: &mut [Option<usize>]) -> usize {
            if let Some(res) = memo[i] {
                return res;
            }

            let res = tree.childs[i]
                .iter()
                .map(|&to| size(to, tree, memo))
                .sum::<usize>() + 1;
            memo[i] = Some(res);
            res
        }

        let n = tree.parent.len();
        let root = tree.root;
        let mut memo = vec![None; n];

        let mut ids = vec![(0, 0); n];
        let mut parts: Vec<Vec<usize>> = Vec::new();
        let mut stack = vec![(root, false, None)];
        let mut parents = Vec::new();

        while let Some((i, h, pid)) = stack.pop() {
            if h {
                let (k, _) = pid.unwrap();
                ids[i] = (k, parts[k].len());
                parts[k].push(i);
            } else {
                ids[i] = (parts.len(), 0);
                parts.push(vec![i]);
                parents.push(pid);
            }

            let id = ids[i];

            let heavy = tree.childs[i]
                .iter()
                .max_by_key(|&&to| size(to, &tree, &mut memo));

            for &to in &tree.childs[i] {
                if Some(&to) != heavy {
                    stack.push((to, false, Some(id)));
                }
            }

            if let Some(&h) = heavy {
                stack.push((h, true, Some(id)));
            }
        }

        HeavyLightDecomposition {
            ids: ids,
            parents: parents,
            parts: parts,
        }
    }
}

#[test]
fn test_lca() {
    use rand::{Rng, SeedableRng, StdRng};
    let size = 1024;

    let mut g = vec![Vec::new(); size + 1];

    for i in 2..size + 1 {
        g[i / 2].push(i);
        g[i].push(1 / 2);
    }

    let lca = LCA::with_root(1, &g);
    let mut rng = StdRng::from_seed(&[1, 2, 3, 1, 2]);

    for _ in 0..10000 {
        let mut a = rng.next_u32() as usize % size + 1;
        let mut b = rng.next_u32() as usize % size + 1;

        let p = lca.lca(a, b);

        while a.leading_zeros() > b.leading_zeros() {
            b >>= 1;
        }

        while b.leading_zeros() > a.leading_zeros() {
            a >>= 1;
        }

        while a != b {
            a >>= 1;
            b >>= 1;
        }

        assert_eq!(p, a);
    }
}
