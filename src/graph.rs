use crate::disjoint_subsets::DisjointSubsets;
use crate::heap::MinHeap;
use core::cmp::{max, min};
use core::fmt::{Display, Formatter};
use core::usize;
use std::collections::HashSet;

struct Vertex {
    i: usize,
    key: usize,
}

impl Vertex {
    pub fn new(i: usize, key: usize) -> Self {
        Self { i, key }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HalfEdge {
    vertex: usize,
    weight: usize,
}

impl HalfEdge {
    pub fn new(vertex: usize, weight: usize) -> Self {
        Self { vertex, weight }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Edge {
    start: usize,
    end: usize,
    weight: usize,
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} - {}: ({})", self.start, self.end, self.weight)?;
        Ok(())
    }
}

impl Edge {
    pub fn new(start: usize, end: usize, weight: usize) -> Self {
        Self { start, end, weight }
    }
}

#[derive(Debug)]
pub struct Graph {
    edges: Vec<Vec<HalfEdge>>,
}

impl Display for Graph {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let mut edges = self.list_of_edges();
        edges.sort_by_key(|e| (e.start, e.end));
        write!(f, "Graph(start - weight - end) {{\n")?;
        for e in edges.iter() {
            write!(f, "    {},\n", e)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl Graph {
    pub fn new(vertex_count: usize) -> Self {
        Self {
            edges: vec![Vec::new(); vertex_count],
        }
    }

    pub fn add_edge(&mut self, new_edge: &Edge) {
        self.edges[new_edge.start].push(HalfEdge::new(new_edge.end, new_edge.weight));
        self.edges[new_edge.end].push(HalfEdge::new(new_edge.start, new_edge.weight));
    }

    pub fn list_of_edges(&self) -> Vec<Edge> {
        let mut edges_set = HashSet::<Edge>::new();
        self.edges.iter().enumerate().for_each(|(start, adj)| {
            adj.iter().for_each(|e| {
                let (min_v, max_v) = (min(start, e.vertex), max(start, e.vertex));
                let sorted_edge = Edge::new(min_v, max_v, e.weight);
                edges_set.insert(sorted_edge);
            })
        });
        edges_set.iter().cloned().collect()
    }

    pub fn total_weight(&self) -> usize {
        self.edges.iter().flatten().map(|e| e.weight).sum::<usize>() / 2
    }

    pub fn min_span_tree_prim(&self) -> Self {
        let vertices_count = self.edges.len();
        //temporary data structure for saving min spanning tree
        let mut span_tree = vec![HalfEdge::new(0, 0); vertices_count];

        if vertices_count > 0 {
            let mut is_visited = vec![false; vertices_count];
            let mut keys = vec![usize::MAX; vertices_count];
            keys[0] = 0;
            let mut queue = MinHeap::from((
                vec![Vertex::new(0_usize, 0_usize)],
                |v1: &Vertex, v2: &Vertex| v1.key.cmp(&v2.key),
            ));
            while let Some(vertex) = queue.pop_root() {
                is_visited[vertex.i] = true;
                self.edges[vertex.i].iter().for_each(|e| {
                    if !is_visited[e.vertex] && (e.weight < keys[e.vertex]) {
                        queue.insert(Vertex::new(e.vertex, e.weight));
                        keys[e.vertex] = e.weight;
                        span_tree[e.vertex] = HalfEdge::new(vertex.i, e.weight);
                    }
                });
            }
        }
        let mut span_tree_graph = Graph::new(vertices_count);
        span_tree
            .iter()
            .enumerate()
            .skip(1)
            .for_each(|(start, e)| span_tree_graph.add_edge(&Edge::new(start, e.vertex, e.weight)));
        span_tree_graph
    }

    pub fn min_span_tree_kruskal(&self) -> Self {
        let vertices_count = self.edges.len();
        let mut all_edges = self.list_of_edges();
        all_edges.sort_unstable_by_key(|e| e.weight);
        let mut vertices = DisjointSubsets::new(vertices_count);
        let mut span_tree = Vec::with_capacity(vertices_count + 1);
        all_edges.iter().for_each(|e| {
            let set1 = vertices.find_set(e.start);
            let set2 = vertices.find_set(e.end);
            if set1 != set2 {
                span_tree.push(e.clone());
                vertices.union_sets(set1, set2);
            }
        });
        Graph::from(span_tree)
    }

    pub fn dijkstra(&self, start: usize) -> Vec<Vec<HalfEdge>> {
        unimplemented!()
    }
}

impl PartialEq for Graph {
    fn eq(&self, other: &Self) -> bool {
        let (mut edges1, mut edges2) = (self.edges.clone(), other.edges.clone());
        edges1
            .iter_mut()
            .for_each(|v| v.sort_unstable_by_key(|e| e.vertex));
        edges2
            .iter_mut()
            .for_each(|v| v.sort_unstable_by_key(|e| e.vertex));
        edges1 == edges2
    }
}

impl From<Vec<Edge>> for Graph {
    fn from(edges: Vec<Edge>) -> Self {
        match edges.is_empty() {
            true => Graph::new(0),
            false => {
                let max_vertex = max(
                    edges.iter().map(|e| e.start).max().unwrap(),
                    edges.iter().map(|e| e.end).max().unwrap(),
                );
                let mut graph = Graph::new(max_vertex + 1);
                edges.iter().for_each(|e| graph.add_edge(e));
                graph
            }
        }
    }
}

trait VertexCode {
    fn to_vertex_code(&self) -> usize;
}

impl VertexCode for char {
    fn to_vertex_code(&self) -> usize {
        match self {
            'A'..='Z' => *self as usize - 'A' as usize,
            'a'..='z' => *self as usize - 'a' as usize,
            _ => unimplemented!(),
        }
    }
}

impl From<Vec<(char, char, usize)>> for Graph {
    fn from(edges: Vec<(char, char, usize)>) -> Self {
        let edges: Vec<Edge> = edges
            .iter()
            .map(|e| Edge::new(e.0.to_vertex_code(), e.1.to_vertex_code(), e.2))
            .collect();
        Self::from(edges)
    }
}
