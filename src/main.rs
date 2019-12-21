mod disjoint_subsets;
mod graph;
mod heap;
mod test;

fn main() {
    let graph = test::graph2();
    let mst_reference = test::graph2_mst();
    let mst_prim = graph.min_span_tree_prim();
    println!("{}", &mst_reference);
    println!("{}", &mst_prim);
}
