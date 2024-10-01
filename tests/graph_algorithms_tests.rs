#[cfg(test)]
mod tests {
    use super::super::graph_algorithms::*;

    #[test]
    fn test_bfs() {
        let graph = vec![
            vec![1, 2], // Node 0
            vec![0, 3], // Node 1
            vec![0, 3], // Node 2
            vec![1, 2], // Node 3
        ];
        assert_eq!(bfs(0, &graph), vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dfs() {
        let graph = vec![
            vec![1, 2], // Node 0
            vec![0, 3], // Node 1
            vec![0, 3], // Node 2
            vec![1, 2], // Node 3
        ];
        assert_eq!(dfs(0, &graph), vec![0, 1, 3, 2]);
    }

    #[test]
    fn test_count_connected_components() {
        let edges = vec![(0, 1), (1, 2), (3, 4)];
        assert_eq!(count_connected_components(5, edges), 2);
    }
}
