#![cfg(test)]
use crate::graph::SimpleGraph;

#[test]
fn test_dijkstra() {
    let mut g = SimpleGraph::<u32>::with_capacity(6);

    g.add_weighted_edges(0, 1, 7);
    g.add_weighted_edges(0, 2, 9);
    g.add_weighted_edges(0, 5, 14);
    g.add_weighted_edges(1, 2, 10);
    g.add_weighted_edges(1, 3, 15);
    g.add_weighted_edges(2, 5, 2);
    g.add_weighted_edges(2, 3, 11);
    g.add_weighted_edges(3, 4, 6);
    g.add_weighted_edges(4, 5, 9);

    let mut sp = g.sssp_dijkstra(0, &[4]);
    assert_eq!(1, sp.len());

    let sp = sp.pop().unwrap();
    assert_eq!(true, sp.is_feasible());
    assert_eq!(20, sp.dist());
    assert_eq!(&[0, 2, 5, 4], sp.path().as_slice());

    g.add_weighted_edges(6, 7, 2);
    g.add_weighted_edges(6, 8, 3);

    let lsp = g.sssp_dijkstra_lazy(0);
    let sp = lsp.get(7);
    assert_eq!(false, sp.is_feasible());

    let sp = lsp.get(4);
    assert_eq!(true, sp.is_feasible());
    assert_eq!(20, sp.dist());
    assert_eq!(&[0, 2, 5, 4], sp.path().as_slice());
}
