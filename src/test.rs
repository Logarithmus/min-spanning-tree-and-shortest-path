use crate::graph::{Edge, Graph};

pub fn graph1() -> Graph {
    Graph::from(vec![
        Edge::new(0, 1, 4),
        Edge::new(0, 2, 3),
        Edge::new(1, 2, 1),
        Edge::new(1, 3, 2),
        Edge::new(2, 3, 4),
        Edge::new(3, 4, 2),
        Edge::new(4, 5, 6),
    ])
}

pub fn graph1_mst() -> Graph {
    Graph::from(vec![
        Edge::new(0, 2, 3),
        Edge::new(1, 2, 1),
        Edge::new(1, 3, 2),
        Edge::new(3, 4, 2),
        Edge::new(4, 5, 6),
    ])
}

pub fn graph2() -> Graph {
    Graph::from(vec![
        Edge::new(0, 1, 5),
        Edge::new(0, 2, 2),
        Edge::new(1, 2, 2),
        Edge::new(1, 3, 3),
        Edge::new(1, 4, 7),
        Edge::new(2, 3, 3),
        Edge::new(2, 6, 9),
        Edge::new(3, 4, 2),
        Edge::new(3, 6, 6),
        Edge::new(4, 6, 5),
        Edge::new(4, 5, 8),
        Edge::new(4, 7, 7),
        Edge::new(5, 7, 3),
        Edge::new(5, 8, 4),
        Edge::new(6, 7, 2),
    ])
}

pub fn graph2_mst() -> Graph {
    Graph::from(vec![
        Edge::new(0, 2, 2),
        Edge::new(2, 1, 2),
        Edge::new(2, 3, 3),
        Edge::new(3, 4, 2),
        Edge::new(4, 6, 5),
        Edge::new(6, 7, 2),
        Edge::new(7, 5, 3),
        Edge::new(5, 8, 4),
    ])
}

pub fn graph_a() -> Graph {
    Graph::from(vec![
        ('A', 'B', 2),
        ('A', 'F', 3),
        ('F', 'D', 4),
        ('D', 'G', 2),
        ('F', 'G', 1),
        ('D', 'E', 2),
        ('G', 'H', 1),
        ('B', 'G', 3),
        ('B', 'E', 4),
        ('B', 'C', 5),
        ('E', 'C', 2),
        ('C', 'H', 4),
        ('E', 'H', 3),
    ])
}

pub fn graph_b() -> Graph {
    Graph::from(vec![
        ('A', 'B', 2),
        ('A', 'D', 6),
        ('A', 'C', 4),
        ('B', 'C', 2),
        ('B', 'E', 6),
        ('C', 'D', 1),
        ('C', 'E', 3),
        ('D', 'E', 2),
        ('D', 'F', 3),
        ('E', 'G', 5),
        ('F', 'G', 4),
    ])
}

pub fn graph_c() -> Graph {
    Graph::from(vec![
        ('A', 'B', 1),
        ('A', 'F', 2),
        ('A', 'D', 5),
        ('B', 'E', 2),
        ('F', 'G', 4),
        ('D', 'G', 2),
        ('D', 'E', 1),
        ('G', 'E', 2),
        ('G', 'H', 5),
        ('E', 'C', 4),
    ])
}

pub fn graph_d() -> Graph {
    Graph::from(vec![
        ('A', 'B', 1),
        ('A', 'D', 2),
        ('B', 'E', 1),
        ('C', 'D', 4),
        ('B', 'C', 1),
        ('D', 'E', 2),
        ('D', 'F', 2),
        ('G', 'H', 1),
        ('F', 'G', 4),
        ('F', 'H', 5),
        ('E', 'H', 3),
        ('E', 'F', 5),
    ])
}

#[test]
fn test1_mst_prim() {
    let graph = graph1();
    let mst_prim = graph.min_span_tree_prim();
    println!("{}", &mst_prim);
    assert_eq!(mst_prim.total_weight(), 14);
}

#[test]
fn test2_mst_prim() {
    let graph = graph2();
    let mst_prim = graph.min_span_tree_prim();
    println!("{}", &mst_prim);
    assert_eq!(mst_prim.total_weight(), 23);
}

#[test]
fn test1_mst_kruskal() {
    let graph = graph1();
    let mst_kruskal = graph.min_span_tree_kruskal();
    println!("{}", &mst_kruskal);
    assert_eq!(mst_kruskal.total_weight(), 14);
}

#[test]
fn test2_mst_kruskal() {
    let graph = graph2();
    let mst_kruskal = graph.min_span_tree_kruskal();
    println!("{}", &mst_kruskal);
    assert_eq!(mst_kruskal.total_weight(), 23);
}
