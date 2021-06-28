use std::ops::AddAssign;

use num_traits::{Bounded, Zero};

use crate::{graph::SimpleGraph, ph::HeapElmt, PairingHeap};

/// Find the minimum spanning tree (MST) in a graph using Prim's algorithm. The function returns the
/// total weight of the MST and a simple graph, whose edges are the MST's edges.
pub fn mst_prim<W>(graph: &SimpleGraph<W>, src: usize) -> (SimpleGraph<W>, W)
where
    W: Copy + PartialOrd + Bounded + Zero + AddAssign,
{
    let mut pq = PairingHeap::<usize, W>::new();
    let mut nodes: Vec<_> = (0..graph.n_nodes())
        .map(|ii| {
            let mut node = PrimNode::<W>::new();
            node.dist = if ii == src {
                <W as Zero>::zero()
            } else {
                <W as Bounded>::max_value()
            };
            node.idx = ii;
            node.heap = pq.insert2(ii, node.dist);
            node
        })
        .collect();

    let mut len = pq.len();

    while len != 0 {
        let (node, _) = pq.delete_min().unwrap();
        nodes[node].heap.none();

        if let Some(nb) = graph.neighbours(&node) {
            for (u, dist) in nb {
                let primnode = &mut nodes[*u];
                if !primnode.heap.is_none() && *dist < primnode.dist {
                    primnode.dist = *dist;
                    primnode.parent = Some(node);
                    pq.update_prio(&primnode.heap, primnode.dist);
                }
            }
        }

        len = pq.len();
    }

    let mut rg = SimpleGraph::<W>::with_capacity(graph.n_nodes());
    let mut dist = <W as Zero>::zero();
    for node in nodes {
        if let Some(p) = node.parent {
            rg.add_weighted_edges(p, node.idx, node.dist);
            dist += node.dist;
        }
    }

    (rg, dist)
}

#[derive(Clone, Debug)]
struct PrimNode<W> {
    idx: usize,
    parent: Option<usize>,
    heap: HeapElmt<usize, W>,
    dist: W,
}

impl<W> PrimNode<W> {
    pub fn new() -> Self
    where
        W: Bounded,
    {
        Self {
            idx: 0,
            parent: None,
            heap: HeapElmt::<usize, W>::default(),
            dist: <W as Bounded>::max_value(),
        }
    }
}
