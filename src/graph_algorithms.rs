use std::collections::{HashSet, VecDeque};

/// Breadth-First Search (BFS)
pub fn bfs(start: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut result = Vec::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    queue.push_back(start);
    visited.insert(start);
    
    while let Some(node) = queue.pop_front() {
        result.push(node);
        for &neighbor in &graph[node] {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }
    
    result
}

/// Depth-First Search (DFS)
pub fn dfs(start: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut result = Vec::new();
    let mut visited = HashSet::new();
    dfs_helper(start, graph, &mut visited, &mut result);
    result
}

fn dfs_helper(node: usize, graph: &Vec<Vec<usize>>, visited: &mut HashSet<usize>, result: &mut Vec<usize>) {
    if visited.contains(&node) {
        return;
    }
    visited.insert(node);
    result.push(node);
    
    for &neighbor in &graph[node] {
        dfs_helper(neighbor, graph, visited, result);
    }
}

/// Union-Find (Connected Components)
pub fn count_connected_components(n: usize, edges: Vec<(usize, usize)>) -> usize {
    let mut parent: Vec<usize> = (0..n).collect();
    
    fn find(parent: &mut Vec<usize>, node: usize) -> usize {
        if parent[node] != node {
            parent[node] = find(parent, parent[node]);
        }
        parent[node]
    }
    
    fn union(parent: &mut Vec<usize>, u: usize, v: usize) {
        let root_u = find(parent, u);
        let root_v = find(parent, v);
        if root_u != root_v {
            parent[root_v] = root_u;
        }
    }
    
    for (u, v) in edges {
        union(&mut parent, u, v);
    }
    
    let mut components = 0;
    for i in 0..n {
        if parent[i] == i {
            components += 1;
        }
    }
    components
}
