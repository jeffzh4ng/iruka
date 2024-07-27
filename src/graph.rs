use std::collections::HashMap;

pub struct AdjacencyMatrix {
    matrix: Vec<Vec<bool>>,
}

impl AdjacencyMatrix {
    pub fn new(size: usize) -> Self {
        AdjacencyMatrix {
            matrix: vec![vec![false; size]; size],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.matrix[from][to] = true;
        self.matrix[to][from] = true; // For undirected graph
    }

    pub fn has_edge(&self, from: usize, to: usize) -> bool {
        self.matrix[from][to]
    }
}

pub struct AdjacencyList {
    list: HashMap<usize, Vec<usize>>,
}

impl AdjacencyList {
    pub fn new() -> Self {
        AdjacencyList {
            list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.list.entry(from).or_insert_with(Vec::new).push(to);
        self.list.entry(to).or_insert_with(Vec::new).push(from); // For undirected graph
    }

    pub fn get_neighbors(&self, vertex: usize) -> Option<&Vec<usize>> {
        self.list.get(&vertex)
    }
}

pub struct EdgeList {
    edges: Vec<(usize, usize)>,
}

impl EdgeList {
    pub fn new() -> Self {
        EdgeList { edges: Vec::new() }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.push((from, to));
    }

    pub fn get_edges(&self) -> &Vec<(usize, usize)> {
        &self.edges
    }
}
