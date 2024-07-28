use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub trait Graph<T: Clone + Eq + Hash> {
    fn neighbors(&self, vertex: &T) -> Vec<T>;
    fn vertices(&self) -> Vec<T>;
}

pub struct AdjacencyMatrix<T> {
    vertices: Vec<T>,
    matrix: Vec<Vec<bool>>,
}

impl<T: Clone + Eq + Hash> Graph<T> for AdjacencyMatrix<T> {
    fn neighbors(&self, vertex: &T) -> Vec<T> {
        let idx = self.vertices.iter().position(|v| v == vertex).unwrap();
        self.matrix[idx]
            .iter()
            .enumerate()
            .filter_map(|(i, &has_edge)| {
                if has_edge {
                    Some(self.vertices[i].clone())
                } else {
                    None
                }
            })
            .collect()
    }

    fn vertices(&self) -> Vec<T> {
        self.vertices.clone()
    }
}

pub struct AdjacencyList<T> {
    list: HashMap<T, Vec<T>>,
}

impl<T: Clone + Eq + Hash> Graph<T> for AdjacencyList<T> {
    fn neighbors(&self, vertex: &T) -> Vec<T> {
        self.list.get(vertex).cloned().unwrap_or_default()
    }

    fn vertices(&self) -> Vec<T> {
        self.list.keys().cloned().collect()
    }
}

pub struct EdgeList<T> {
    vertices: HashSet<T>,
    edges: Vec<(T, T)>,
}

impl<T: Clone + Eq + Hash> Graph<T> for EdgeList<T> {
    fn neighbors(&self, vertex: &T) -> Vec<T> {
        self.edges
            .iter()
            .filter_map(|(from, to)| {
                if from == vertex {
                    Some(to.clone())
                } else if to == vertex {
                    Some(from.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    fn vertices(&self) -> Vec<T> {
        self.vertices.iter().cloned().collect()
    }
}
