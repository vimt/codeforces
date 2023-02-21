pub struct Adjacency {
    head: Vec<usize>,
    e: Vec<(usize, usize)>,
    cnt: usize,
}

pub struct AdjacencyIter<'a> {
    adj: &'a Adjacency,
    p: usize,
}

impl<'a> Iterator for AdjacencyIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.p == 0 { return None; }
        let (np, v) = self.adj.e[self.p];
        self.p = np;
        Some(v)
    }
}

impl Adjacency {
    pub fn new(n: usize, edge_num: usize) -> Self {
        Self {
            head: vec![0; n],
            e: vec![(0, 0); edge_num],
            cnt: 0,
        }
    }

    pub fn add(&mut self, u: usize, v: usize) {
        self.cnt += 1;
        self.e[self.cnt] = (self.head[u], v);
        self.head[u] = self.cnt;
    }

    pub fn neigh(&self, u: usize) -> AdjacencyIter {
        return AdjacencyIter { adj: self, p: self.head[u] };
    }
}
