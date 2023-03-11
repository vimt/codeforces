pub struct Graph<T=()> {
    head: Vec<usize>,
    e: Vec<(usize, usize, T)>,
}

pub struct GraphIter<'a, T> {
    adj: &'a Graph<T>,
    p: usize,
}

impl<'a, T: Sized> Iterator for GraphIter<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.p == usize::MAX { return None; }
        let (np, v, val) = &self.adj.e[self.p];
        self.p = *np;
        Some((*v, val))
    }
}

impl<T> Graph<T> {
    pub fn new(n: usize, edge_num: usize) -> Self {
        Self {
            head: vec![usize::MAX; n],
            e: Vec::with_capacity(edge_num),
        }
    }

    pub fn add(&mut self, u: usize, v: usize, val: T) {
        self.e.push((self.head[u], v, val));
        self.head[u] = self.e.len() - 1;
    }

    pub fn neigh(&self, u: usize) -> GraphIter<T> {
        return GraphIter { adj: self, p: self.head[u] };
    }
}
