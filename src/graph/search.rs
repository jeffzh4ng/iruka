use super::Graph;
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

pub fn bfs<T: Clone + Eq + Hash, G: Graph<T>>(graph: &G, start: &T) -> Vec<T> {
    let mut result = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(start.clone());
    visited.insert(start.clone());

    while let Some(v) = queue.pop_front() {
        result.push(v.clone());

        for neighbor in graph.neighbors(&v) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor.clone());
                queue.push_back(neighbor);
            }
        }
    }

    result
}

pub fn dfs<T: Clone + Eq + Hash, G: Graph<T>>(graph: &G, start: &T) -> Vec<T> {
    let mut result = Vec::new();
    let mut visited = HashSet::new();

    dfs_recursive(graph, start, &mut visited, &mut result);

    result
}

fn dfs_recursive<T: Clone + Eq + Hash, G: Graph<T>>(
    graph: &G,
    vertex: &T,
    visited: &mut HashSet<T>,
    result: &mut Vec<T>,
) {
    visited.insert(vertex.clone());
    result.push(vertex.clone());

    for neighbor in graph.neighbors(vertex) {
        if !visited.contains(&neighbor) {
            dfs_recursive(graph, &neighbor, visited, result);
        }
    }
}
